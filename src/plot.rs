//! A plot as data: named line and scatter series, reference lines, a second
//! y-axis, and curves the reader can retune with a slider.
//!
//! Everything here is plain numbers and strings, so a plot survives
//! `save`/`load` like the rest of a lesson. This file is the surface an
//! author writes against; [`resolve`] turns what they wrote into geometry and
//! [`axis`] settles the numbers along the edges.

use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

mod axis;
mod resolve;

pub(crate) use axis::{SecondaryAxis, Ticks, Y_TICKS, format_tick};
pub(crate) use resolve::{ResolvedPlot, ResolvedSeries, SeriesColor};

use resolve::sample;

/// The name an expression curve's horizontal position goes by.
pub(crate) const FREE_VARIABLE: &str = "x";

/// [`Plot`]'s default height, in egui points — tall enough to read the curve
/// without the reading column growing to accommodate it.
const DEFAULT_HEIGHT: f32 = 260.0;

/// A plot: one or more series over a shared x domain, with optional reference
/// lines and tunable parameters.
///
/// ```
/// # use math_me::Plot;
/// let plot = Plot::new(-5.0..=5.0)
///     .curve("today's variance", "omega + alpha * x^2")
///     .param("omega", 0.0..=0.2, 0.05)
///     .param("alpha", 0.0..=0.3, 0.09)
///     .hline(5.0)
///     .x_label("yesterday's shock");
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plot {
    pub(crate) series: Vec<Series>,
    pub(crate) params: Vec<Param>,
    pub(crate) hlines: Vec<f64>,
    pub(crate) vlines: Vec<f64>,
    /// The x range the plot covers: what expression curves and sampled
    /// closures are drawn over, and always in view.
    pub(crate) domain: [f64; 2],
    pub(crate) x_label: Option<String>,
    pub(crate) y_label: Option<String>,
    pub(crate) y2_label: Option<String>,
    pub(crate) caption: String,
    pub(crate) height: f32,
}

impl Plot {
    /// An empty plot over `x_range`, ready for series to be added to it.
    pub fn new(x_range: RangeInclusive<f64>) -> Self {
        Self {
            series: Vec::new(),
            params: Vec::new(),
            hlines: Vec::new(),
            vlines: Vec::new(),
            domain: [*x_range.start(), *x_range.end()],
            x_label: None,
            y_label: None,
            y2_label: None,
            caption: String::new(),
            height: DEFAULT_HEIGHT,
        }
    }

    /// A plot of one unnamed curve, sampled from `f` across `range`.
    ///
    /// The shortest way to draw a single fixed curve. A plot of several, or
    /// one the reader can retune, starts from [`Plot::new`] instead.
    pub fn from_fn(range: RangeInclusive<f64>, f: impl Fn(f64) -> f64) -> Self {
        Self::new(range).line_fn("", f)
    }

    /// A plot of one unnamed curve through points that are already sampled.
    /// The domain is taken from the points.
    pub fn from_points(points: Vec<[f64; 2]>) -> Self {
        let [start, end] = resolve::domain_of(&points);
        Self::new(start..=end).line("", points)
    }

    /// Add a line through `points`.
    ///
    /// A named series earns a legend entry on screen and in the PDF; pass `""`
    /// for a plot whose single curve needs no naming.
    pub fn line(self, name: impl Into<String>, points: Vec<[f64; 2]>) -> Self {
        self.push(name, SeriesData::Points(points), Shape::Line)
    }

    /// Add a line by sampling `f` across the plot's domain.
    pub fn line_fn(self, name: impl Into<String>, f: impl Fn(f64) -> f64) -> Self {
        let points = sample(self.domain, f);
        self.line(name, points)
    }

    /// Add a line from an expression in `x`, redrawn whenever the reader moves
    /// one of the plot's [parameters](Self::param).
    ///
    /// `x` is the free variable and every other name must be a declared
    /// parameter; [`crate::Lesson::audit`] reports an expression that does not
    /// parse or that reads a name nothing declares. Samples that come out
    /// non-finite — a pole, the log of a negative — leave a gap in the line
    /// rather than a spike.
    pub fn curve(self, name: impl Into<String>, expression: impl Into<String>) -> Self {
        self.push(name, SeriesData::Expression(expression.into()), Shape::Line)
    }

    /// Add `points` as unconnected markers.
    pub fn scatter(self, name: impl Into<String>, points: Vec<[f64; 2]>) -> Self {
        self.push(name, SeriesData::Points(points), Shape::Scatter)
    }

    /// Move the series added last onto the right-hand y-axis, which carries
    /// its own scale and [`Self::y2_label`] — for a quantity that shares the
    /// x-axis but not the units.
    ///
    /// Reads as a suffix on the series it moves, on the same line:
    /// `.curve("gap closed", "100 * (1 - exp(-k * x))").secondary()`. Does
    /// nothing if no series has been added yet.
    pub fn secondary(mut self) -> Self {
        if let Some(series) = self.series.last_mut() {
            series.axis = Axis::Secondary;
        }
        self
    }

    /// Declare a parameter the reader can move, giving it a slider under the
    /// plot. Its `name` is what a [`Self::curve`] expression calls it by, and
    /// `default` is the value the PDF freezes it at.
    ///
    /// [`crate::Lesson::audit`] reports a parameter no curve reads, since its
    /// slider would move and change nothing.
    pub fn param(mut self, name: impl Into<String>, range: RangeInclusive<f64>, default: f64) -> Self {
        self.params.push(Param {
            name: name.into(),
            min: *range.start(),
            max: *range.end(),
            default,
        });
        self
    }

    /// Add a horizontal reference line at `y`, such as a long-run level.
    pub fn hline(mut self, y: f64) -> Self {
        self.hlines.push(y);
        self
    }

    /// Add a vertical reference line at `x`, such as the date of an event.
    pub fn vline(mut self, x: f64) -> Self {
        self.vlines.push(x);
        self
    }

    /// Label the x-axis, under the plot in both renderings.
    pub fn x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Label the left-hand y-axis.
    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Label the right-hand axis the [`Self::secondary`] series are read
    /// against.
    pub fn y2_label(mut self, label: impl Into<String>) -> Self {
        self.y2_label = Some(label.into());
        self
    }

    /// The line under the plot that says what to look at, and what dragging
    /// the sliders will do.
    pub fn caption(mut self, text: impl Into<String>) -> Self {
        self.caption = text.into();
        self
    }

    /// How tall the plot is drawn on screen, in egui points. Defaults to a
    /// height tuned to the reading column; raise it for a curve whose shape
    /// needs the room, at the cost of a longer scroll. The PDF sets every
    /// plot at one size, so the page stays even.
    pub fn height(mut self, points: f32) -> Self {
        self.height = points;
        self
    }

    fn push(mut self, name: impl Into<String>, data: SeriesData, shape: Shape) -> Self {
        self.series.push(Series {
            name: name.into(),
            data,
            shape,
            axis: Axis::Primary,
        });
        self
    }

    /// Where every parameter's slider starts, and where the PDF pins it.
    pub(crate) fn defaults(&self) -> Vec<f64> {
        self.params.iter().map(|param| param.default).collect()
    }

    /// Whether `name` is something a curve's expression may read.
    pub(crate) fn declares(&self, name: &str) -> bool {
        name == FREE_VARIABLE || self.params.iter().any(|param| param.name == name)
    }

    /// The source of every tunable curve, for [`crate::Lesson::audit`].
    pub(crate) fn expressions(&self) -> impl Iterator<Item = &str> {
        self.series.iter().filter_map(|series| match &series.data {
            SeriesData::Expression(source) => Some(source.as_str()),
            SeriesData::Points(_) => None,
        })
    }

    /// A [`Self::y2_label`] with nothing to label. The right-hand axis is
    /// only drawn for a series moved onto it, so until one is, that text
    /// cannot reach the reader in either rendering.
    pub(crate) fn stray_y2_label(&self) -> Option<&str> {
        let label = self.y2_label.as_deref()?;
        self.series
            .iter()
            .all(|series| series.axis == Axis::Primary)
            .then_some(label)
    }

    /// The parameters none of `read` names — sliders that would move and
    /// change nothing.
    ///
    /// A parameter called `x` is one of them however often the curves say
    /// `x`, because the free variable always wins that lookup and the slider
    /// is never what is read.
    pub(crate) fn unread_parameters<'a>(&'a self, read: &'a [String]) -> impl Iterator<Item = &'a str> {
        self.params
            .iter()
            .map(|param| param.name.as_str())
            .filter(move |name| *name == FREE_VARIABLE || !read.iter().any(|found| found == name))
    }
}

/// One line or scatter on a plot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Series {
    /// Empty for a series that wants no legend entry.
    pub(crate) name: String,
    pub(crate) data: SeriesData,
    pub(crate) shape: Shape,
    pub(crate) axis: Axis,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum SeriesData {
    Points(Vec<[f64; 2]>),
    /// An expression in `x` and the plot's parameters, sampled at draw time.
    Expression(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Shape {
    Line,
    Scatter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Axis {
    Primary,
    Secondary,
}

/// A knob under the plot: the reader drags it, every expression curve is
/// re-evaluated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Param {
    pub(crate) name: String,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) default: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tunable_plot_survives_json_with_its_series_and_params_intact() {
        let plot = Plot::new(-2.0..=3.5)
            .curve("tunable", "a * x^2")
            .scatter("observed", vec![[0.0, 1.0]])
            .secondary()
            .param("a", -1.0..=1.0, 0.25)
            .hline(0.5)
            .vline(1.5)
            .y2_label("counts");

        let json = serde_json::to_string(&plot).expect("a plot is plain data");
        let restored: Plot = serde_json::from_str(&json).expect("and reads back as itself");

        assert_eq!(restored, plot);
        assert_eq!(restored.series[0].data, SeriesData::Expression("a * x^2".to_owned()));
        assert_eq!(restored.series[1].shape, Shape::Scatter);
        assert_eq!(restored.series[1].axis, Axis::Secondary, ".secondary() moved the series added last");
        assert_eq!(
            restored.params,
            vec![Param { name: "a".to_owned(), min: -1.0, max: 1.0, default: 0.25 }]
        );
    }
}
