//! Turning a [`Plot`] into geometry: expressions evaluated, gaps cut, the
//! second axis folded into the first, and a frame that can be divided by.
//!
//! Neither renderer reads the model directly — they both ask for a
//! [`ResolvedPlot`] — so this is the one place that decides what a curve is,
//! and the only reason the screen and the page agree on it.

use crate::expr::Expr;
use crate::plot::axis::{SecondaryAxis, Y_TICKS, nice_step};
use crate::plot::{Axis, Param, Plot, SeriesData, Shape};

/// How many points an expression curve or a sampled closure is drawn from.
const PLOT_SAMPLES: u32 = 400;

/// How much clear air to leave above and below the data, as a fraction of its
/// span, so a curve never sits flush against the frame.
const Y_PADDING: f64 = 0.1;

impl Plot {
    /// Turn the plot into the geometry a renderer draws, with every
    /// expression evaluated at `values` — one per declared parameter, in
    /// declaration order.
    pub(crate) fn resolve(&self, values: &[f64]) -> ResolvedPlot {
        let mut bindings = Bindings::new(&self.params, values);
        let mut errors = Vec::new();
        let mut resolved: Vec<(Axis, ResolvedSeries)> = Vec::new();

        for (index, series) in self.series.iter().enumerate() {
            let points = match &series.data {
                SeriesData::Points(points) => points.clone(),
                SeriesData::Expression(source) => match self.evaluate(source, &mut bindings) {
                    Ok(points) => points,
                    Err(message) => {
                        errors.push(message);
                        continue;
                    }
                },
            };
            resolved.push((
                series.axis,
                ResolvedSeries {
                    name: series.name.clone(),
                    // Indexed by declaration, not by how many series drew, so
                    // one broken expression does not recolour its neighbours.
                    color: PALETTE[index % PALETTE.len()],
                    shape: series.shape,
                    segments: segments(points),
                },
            ));
        }

        // The declared domain is always in view, whether or not a series
        // reaches its ends: it is the x range the author said the plot covers.
        let mut x = Extent::default();
        let (mut primary, mut secondary) = (Extent::default(), Extent::default());
        x.extend(self.domain[0]);
        x.extend(self.domain[1]);
        for (axis, series) in &resolved {
            for point in series.segments.iter().flatten() {
                x.extend(point[0]);
                match axis {
                    Axis::Primary => primary.extend(point[1]),
                    Axis::Secondary => secondary.extend(point[1]),
                }
            }
        }
        for &y in &self.hlines {
            primary.extend(y);
        }
        for &value in &self.vlines {
            x.extend(value);
        }
        // A plot with nothing on the left axis would otherwise be framed
        // against a meaningless 0..1; borrow the right axis's own range so the
        // mapping below is the identity.
        if primary.is_empty() {
            primary = secondary;
        }

        let x_bounds = x.frame(0.0);
        let y_bounds = primary.frame(Y_PADDING);
        // Rounded outward rather than padded by a fraction: the right-hand
        // axis then runs between round numbers, and a percentage that starts
        // at zero is not given a tick below it.
        let secondary_axis =
            (!secondary.is_empty()).then(|| SecondaryAxis::new(secondary.rounded_outward(Y_TICKS), y_bounds));

        let series = resolved
            .into_iter()
            .map(|(axis, mut series)| {
                if let (Axis::Secondary, Some(map)) = (axis, secondary_axis) {
                    for point in series.segments.iter_mut().flatten() {
                        point[1] = map.to_primary(point[1]);
                    }
                }
                series
            })
            .collect();

        ResolvedPlot { series, errors, x: x_bounds, y: y_bounds, secondary: secondary_axis }
    }

    fn evaluate(&self, source: &str, bindings: &mut Bindings<'_>) -> Result<Vec<[f64; 2]>, String> {
        let expr = Expr::parse(source).map_err(|error| format!("`{source}`: {error}"))?;
        if let Some(name) = expr.variables().into_iter().find(|name| !self.declares(name)) {
            return Err(format!("`{source}` reads `{name}`, which no .param() declares"));
        }
        Ok(sample(self.domain, |x| expr.eval(bindings.at(x))))
    }
}

/// A plot with its expressions evaluated and its geometry settled: what both
/// renderers actually draw.
pub(crate) struct ResolvedPlot {
    /// Every series that drew, in declaration order. Secondary-axis series
    /// arrive already mapped into the primary axis's coordinates, so a
    /// renderer only ever plots one coordinate space.
    pub(crate) series: Vec<ResolvedSeries>,
    /// Expressions that could not be drawn, phrased for an inline label.
    pub(crate) errors: Vec<String>,
    pub(crate) x: [f64; 2],
    pub(crate) y: [f64; 2],
    /// Present when any series was put [`Plot::secondary`]; its only remaining
    /// job is labelling the right-hand axis.
    pub(crate) secondary: Option<SecondaryAxis>,
}

pub(crate) struct ResolvedSeries {
    pub(crate) name: String,
    pub(crate) color: SeriesColor,
    pub(crate) shape: Shape,
    /// Contiguous runs of points. A non-finite sample ends one run and starts
    /// the next, so a pole leaves a gap rather than a spike to infinity.
    pub(crate) segments: Vec<Vec<[f64; 2]>>,
}

/// A series colour, in both palettes at once.
///
/// The viewer's `#4ADE80` is tuned to glow on near-black; on white paper it
/// has to go several shades darker to keep the same weight. Every colour
/// below is that same pairing, so a lesson looks like itself in either place.
#[derive(Clone, Copy)]
pub(crate) struct SeriesColor {
    pub(crate) screen: [u8; 3],
    pub(crate) print: [u8; 3],
}

/// Assigned to series in declaration order, and deliberately not author-
/// facing: a lesson picking its own colours is a lesson that can pick two the
/// reader cannot tell apart, or one that vanishes on paper.
const PALETTE: [SeriesColor; 5] = [
    SeriesColor { screen: [0x4A, 0xDE, 0x80], print: [0x16, 0xA3, 0x4A] },
    SeriesColor { screen: [0x60, 0xA5, 0xFA], print: [0x1D, 0x4E, 0xD8] },
    SeriesColor { screen: [0xFB, 0xBF, 0x24], print: [0xB4, 0x53, 0x09] },
    SeriesColor { screen: [0xF4, 0x72, 0xB6], print: [0xBE, 0x18, 0x5D] },
    SeriesColor { screen: [0xA7, 0x8B, 0xFA], print: [0x6D, 0x28, 0xD9] },
];

/// The names an expression may read while a curve is sampled: the free
/// variable, whose value moves with every sample, and the plot's parameters,
/// which hold still.
struct Bindings<'a> {
    slots: Vec<(&'a str, f64)>,
}

impl<'a> Bindings<'a> {
    fn new(params: &'a [Param], values: &[f64]) -> Self {
        let mut slots = vec![(super::FREE_VARIABLE, 0.0)];
        slots.extend(params.iter().zip(values).map(|(param, value)| (param.name.as_str(), *value)));
        Self { slots }
    }

    fn at(&mut self, x: f64) -> &[(&'a str, f64)] {
        self.slots[0].1 = x;
        &self.slots
    }
}

/// The x range `points` cover, for a plot that was handed its data rather
/// than a domain to sample over.
pub(super) fn domain_of(points: &[[f64; 2]]) -> [f64; 2] {
    let mut domain = Extent::default();
    for point in points {
        domain.extend(point[0]);
    }
    domain.frame(0.0)
}

pub(super) fn sample(domain: [f64; 2], mut f: impl FnMut(f64) -> f64) -> Vec<[f64; 2]> {
    let [start, end] = domain;
    let span = end - start;
    (0..=PLOT_SAMPLES)
        .map(|step| {
            let x = start + span * f64::from(step) / f64::from(PLOT_SAMPLES);
            [x, f(x)]
        })
        .collect()
}

/// Split `points` into the runs a renderer can draw as unbroken paths,
/// dropping the non-finite samples that separate them.
fn segments(points: Vec<[f64; 2]>) -> Vec<Vec<[f64; 2]>> {
    let mut runs = Vec::new();
    let mut run = Vec::new();
    for point in points {
        if point[0].is_finite() && point[1].is_finite() {
            run.push(point);
        } else if !run.is_empty() {
            runs.push(std::mem::take(&mut run));
        }
    }
    if !run.is_empty() {
        runs.push(run);
    }
    runs
}

/// The smallest interval covering everything fed to it.
#[derive(Clone, Copy)]
struct Extent {
    min: f64,
    max: f64,
}

impl Default for Extent {
    fn default() -> Self {
        Self { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }
}

impl Extent {
    fn extend(&mut self, value: f64) {
        if value.is_finite() {
            self.min = self.min.min(value);
            self.max = self.max.max(value);
        }
    }

    fn is_empty(self) -> bool {
        self.min > self.max
    }

    /// The extent widened by `padding` (as a fraction of its span) into a
    /// range a renderer can safely divide by: always finite, always with room
    /// between its ends, whatever it was fed.
    fn frame(self, padding: f64) -> [f64; 2] {
        let pad = (self.max - self.min) * padding;
        let framed = [self.min - pad, self.max + pad];
        if framed[0].is_finite() && framed[1].is_finite() && framed[1] - framed[0] > f64::EPSILON {
            return framed;
        }
        if !self.min.is_finite() {
            return [0.0, 1.0];
        }
        // Half a unit either side of a single value — except out where the
        // gaps between floats are wider than that, and half a unit would
        // round straight back to the value it was meant to clear.
        let clearance = 0.5f64.max(self.min.abs() * f64::EPSILON * 4.0);
        [self.min - clearance, self.min + clearance]
    }

    /// The extent rounded outward to whole steps, so an axis drawn against it
    /// begins and ends on a round number.
    fn rounded_outward(self, count: u32) -> [f64; 2] {
        let framed = self.frame(0.0);
        let step = nice_step((framed[1] - framed[0]) / f64::from(count));
        [(framed[0] / step).floor() * step, (framed[1] / step).ceil() * step]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve(plot: &Plot) -> ResolvedPlot {
        plot.resolve(&plot.defaults())
    }

    #[test]
    fn from_fn_samples_the_given_range_into_one_unnamed_line() {
        let plot = Plot::from_fn(-1.0..=1.0, |x| x * x);
        let resolved = resolve(&plot);

        let [series] = &resolved.series[..] else {
            panic!("expected exactly one series, got {}", resolved.series.len());
        };
        let [points] = &series.segments[..] else {
            panic!("expected one unbroken segment, got {}", series.segments.len());
        };
        assert!(series.name.is_empty());
        assert_eq!(points.len(), PLOT_SAMPLES as usize + 1);
        assert_eq!(points.first(), Some(&[-1.0, 1.0]));
        assert_eq!(points.last(), Some(&[1.0, 1.0]));
    }

    #[test]
    fn a_curve_is_redrawn_from_the_values_it_is_resolved_at() {
        let plot = Plot::new(0.0..=1.0).curve("c", "a * x").param("a", 0.0..=10.0, 2.0);

        let at_default = resolve(&plot);
        let at_five = plot.resolve(&[5.0]);

        assert_eq!(at_default.series[0].segments[0].last(), Some(&[1.0, 2.0]));
        assert_eq!(at_five.series[0].segments[0].last(), Some(&[1.0, 5.0]));
    }

    #[test]
    fn a_pole_breaks_the_line_in_two_rather_than_spiking() {
        let plot = Plot::new(-1.0..=1.0).curve("c", "1 / x");

        let resolved = resolve(&plot);

        assert_eq!(resolved.series[0].segments.len(), 2, "expected a gap at the pole");
        assert!(resolved.errors.is_empty());
    }

    #[test]
    fn a_broken_expression_reports_itself_and_leaves_the_other_series_alone() {
        let plot = Plot::new(0.0..=1.0)
            .curve("good", "x * 2")
            .curve("bad", "x +")
            .curve("undeclared", "x * gamma");

        let resolved = resolve(&plot);

        assert_eq!(resolved.series.len(), 1);
        assert_eq!(resolved.series[0].name, "good");
        assert_eq!(resolved.errors.len(), 2);
        assert!(resolved.errors[1].contains("gamma"), "{:?}", resolved.errors);
    }

    #[test]
    fn a_secondary_series_is_drawn_where_its_own_axis_labels_it() {
        let plot = Plot::new(0.0..=1.0)
            .line("left", vec![[0.0, 0.0], [0.5, 4.0], [1.0, 10.0]])
            .line("right", vec![[0.0, 0.0], [0.5, 40.0], [1.0, 91.0]])
            .secondary();

        let resolved = resolve(&plot);
        let map = resolved.secondary.expect("a secondary axis was asked for");

        // The contract between the two renderers and the reader: wherever a
        // secondary point was moved to, reading that spot off the right-hand
        // axis gives back the number the author plotted.
        for (drawn, plotted) in resolved.series[1].segments[0].iter().zip([0.0, 40.0, 91.0]) {
            assert!(
                (map.to_secondary(drawn[1]) - plotted).abs() < 1e-9,
                "{drawn:?} should read back as {plotted}"
            );
        }
        // And it is framed against round numbers, so the axis is labelled in
        // them: 0..91 is drawn against 0..100, not 0..91 padded a tenth.
        assert!((map.to_secondary(resolved.y[0])).abs() < 1e-9, "the frame should start at 0");
        assert!((map.to_secondary(resolved.y[1]) - 100.0).abs() < 1e-9, "and end at 100");
    }

    #[test]
    fn the_declared_domain_stays_in_view_whatever_the_series_reach() {
        let plot = Plot::new(0.0..=10.0).scatter("clustered", vec![[4.0, 1.0], [5.0, 2.0]]);

        let resolved = resolve(&plot);

        assert_eq!(resolved.x, [0.0, 10.0], "the x range the author asked for should be the frame");
    }

    #[test]
    fn an_empty_plot_still_frames_something_a_renderer_can_divide_by() {
        let huge = 1e17;
        for plot in [
            Plot::new(0.0..=0.0),
            Plot::from_points(Vec::new()),
            Plot::new(-1.0..=1.0),
            // Far enough out that half a unit either side rounds back to the
            // value itself, which used to leave a frame of zero width.
            Plot::from_points(vec![[huge, huge]]),
        ] {
            let resolved = resolve(&plot);
            assert!(resolved.x[1] > resolved.x[0], "x bounds collapsed: {:?}", resolved.x);
            assert!(resolved.y[1] > resolved.y[0], "y bounds collapsed: {:?}", resolved.y);
        }
    }
}
