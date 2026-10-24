//! Fitting a diagram to the width the host has. Ported from pele's `diagrams/common/fit-width.ts`.

use crate::RenderOptions;
use crate::diagram::Output;

fn available(options: &RenderOptions) -> Option<f64> {
    options.max_width.filter(|&w| w > 0.0)
}

/// Draws a chart whose plot can be any width so that the whole diagram fits the width the host
/// has. What surrounds the plot (labels, a legend) is only known once it is drawn, so the plot
/// gives up the overflow and is drawn again. `narrow` tells the chart that it did not fit at
/// its own width, for a chart that then arranges itself differently.
pub(crate) fn fit_width(
    options: &RenderOptions,
    natural: f64,
    least: f64,
    mut draw: impl FnMut(f64, bool) -> Output,
) -> Output {
    let result = draw(natural, false);
    let Some(available) = available(options).filter(|&a| result.width > a) else {
        return result;
    };
    let floor = least.min(natural);
    let mut plot = natural;
    let mut result = draw(plot, true);
    for _ in 0..3 {
        if result.width <= available || plot <= floor {
            break;
        }
        plot = floor.max(plot - (result.width - available));
        result = draw(plot, true);
    }
    result
}

/// The width a title may take before it wraps.
pub(crate) fn title_room(options: &RenderOptions) -> f64 {
    available(options).map_or(4000.0, |a| {
        80f64.max(a - 2.0 * options.padding.unwrap_or(8.0))
    })
}
