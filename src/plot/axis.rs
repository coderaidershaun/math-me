//! The numbers along the edges of a plot: where the ticks fall, how they are
//! written, and how a right-hand axis is faked out of a single coordinate
//! space.
//!
//! Both renderers draw their own axes — `egui_plot` labels the strips it is
//! given, the PDF writes text into an SVG — so this is what they have to
//! agree on for a lesson to look like itself on screen and on paper.

/// How many ticks a y-axis carries, near enough: the count the PDF splits on,
/// and the count the right-hand axis's range is rounded to sit on.
pub(crate) const Y_TICKS: u32 = 5;

/// The linear map between the right-hand axis's values and the primary
/// coordinates everything is drawn in.
///
/// `egui_plot` has one coordinate space per plot, and the SVG exporter draws
/// into one frame, so a genuine second scale is not available to either. Both
/// instead stretch the secondary series onto the primary range and label the
/// right-hand strip with [`Self::to_secondary`], which is the standard way to
/// fake a second axis and is exact for the linear scales this library draws.
#[derive(Clone, Copy)]
pub(crate) struct SecondaryAxis {
    offset: f64,
    scale: f64,
}

impl SecondaryAxis {
    pub(super) fn new(secondary: [f64; 2], primary: [f64; 2]) -> Self {
        let scale = (primary[1] - primary[0]) / (secondary[1] - secondary[0]);
        Self { offset: primary[0] - secondary[0] * scale, scale }
    }

    /// Where a value on the right-hand axis is drawn, in the coordinates both
    /// renderers plot in.
    pub(crate) fn to_primary(self, value: f64) -> f64 {
        self.offset + value * self.scale
    }

    /// What a tick at `primary` should be labelled on the right-hand axis.
    pub(crate) fn to_secondary(self, primary: f64) -> f64 {
        (primary - self.offset) / self.scale
    }

    /// A gap of `primary` in the plot's coordinates, in the right-hand axis's
    /// own units — how far apart its labels are, which is what decides how
    /// many decimals they are worth printing to.
    pub(crate) fn span_to_secondary(self, primary: f64) -> f64 {
        primary / self.scale
    }
}

/// The tick values along one axis, and the step between them.
pub(crate) struct Ticks {
    pub(crate) step: f64,
    pub(crate) values: Vec<f64>,
}

impl Ticks {
    /// At most `count` ticks across `range`, on the roundest step that covers
    /// it — the numbers a reader can hold in their head, rather than wherever
    /// an equal split of the data happened to land.
    pub(crate) fn over(range: [f64; 2], count: u32) -> Self {
        let step = nice_step((range[1] - range[0]) / f64::from(count));
        let first = (range[0] / step).ceil();
        Self {
            step,
            // Bounded rather than looped to the end of the range: a step this
            // size cannot fit more than `count` times into it, so nothing here
            // depends on a float comparison to terminate.
            values: (0..=count)
                .map(|offset| (first + f64::from(offset)) * step)
                .take_while(|value| *value <= range[1] + step * TICK_TOLERANCE)
                .collect(),
        }
    }
}

/// How far past the end of a range a tick may land and still be drawn, as a
/// fraction of the step: enough to keep the last tick of an exactly-divided
/// range from being lost to rounding.
const TICK_TOLERANCE: f64 = 1e-9;

/// `rough` rounded up to the nearest round number: 1, 2, 2.5 or 5 times a
/// power of ten.
pub(super) fn nice_step(rough: f64) -> f64 {
    let magnitude = 10f64.powf(rough.abs().log10().floor());
    let steps = [1.0, 2.0, 2.5, 5.0, 10.0];
    let step = steps.into_iter().find(|step| rough.abs() <= step * magnitude).unwrap_or(10.0);
    step * magnitude
}

/// A tick's label, at the precision its `step` earns: two significant digits
/// of the step, then trimmed. A step of 10 prints `20`, a step of 0.25 prints
/// `0.25`, and a right-hand axis whose ticks land on 98.615 prints `99`.
pub(crate) fn format_tick(value: f64, step: f64) -> String {
    let decimals = (1.0 - step.abs().log10().floor()).clamp(0.0, 6.0) as usize;
    let text = format!("{value:.decimals$}");
    let trimmed = match text.contains('.') {
        true => text.trim_end_matches('0').trim_end_matches('.'),
        false => text.as_str(),
    };
    (if trimmed.is_empty() || trimmed == "-0" { "0" } else { trimmed }).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tick_is_labelled_to_the_precision_its_step_earns() {
        assert_eq!(format_tick(-5.0, 1.0), "-5");
        assert_eq!(format_tick(100.0, 10.0), "100", "a whole number must not be trimmed to nothing");
        assert_eq!(format_tick(4.4, 0.2), "4.4");
        assert_eq!(format_tick(-0.001, 1.0), "0");
        assert_eq!(format_tick(0.25, 0.25), "0.25");
        // What the right-hand axis hands it: primary ticks mapped back into
        // its own units land nowhere round, and printing them in full is what
        // made a percentage axis read `98.62`.
        assert_eq!(format_tick(98.6153, 15.38), "99");
    }

    #[test]
    fn ticks_land_on_round_numbers_inside_the_range() {
        let cases: &[([f64; 2], u32, f64, &[f64])] = &[
            ([0.0, 30.0], 8, 5.0, &[0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0]),
            ([-5.0, 5.0], 8, 2.0, &[-4.0, -2.0, 0.0, 2.0, 4.0]),
            ([17.6, 44.8], 5, 10.0, &[20.0, 30.0, 40.0]),
            ([0.0, 1.0], 5, 0.2, &[0.0, 0.2, 0.4, 0.6, 0.8, 1.0]),
            ([0.0, 0.9], 4, 0.25, &[0.0, 0.25, 0.5, 0.75]),
        ];

        for (range, count, step, expected) in cases {
            let ticks = Ticks::over(*range, *count);
            assert_eq!(ticks.step, *step, "step for {range:?}");
            assert_eq!(ticks.values.len(), expected.len(), "{:?} for {range:?}", ticks.values);
            for (found, want) in ticks.values.iter().zip(*expected) {
                assert!((found - want).abs() < 1e-9, "{:?} for {range:?}", ticks.values);
            }
        }
    }
}
