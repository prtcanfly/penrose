use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReflectHorizontal},
        MainAndStack,
    },
    core::layout::LayoutStack,
    stack,
};

pub fn layouts() -> LayoutStack {
    let max_main = 1;
    let ratio = 0.5;
    let ratio_step = 0.1;
    let outer_px = 5;
    let inner_px = 5;

    stack!(
        MainAndStack::side(max_main, ratio, ratio_step),
        ReflectHorizontal::wrap(MainAndStack::side(max_main, ratio, ratio_step)),
        MainAndStack::bottom(max_main, ratio, ratio_step)
    )
    .map(|layout| Gaps::wrap(layout, outer_px, inner_px))
}
