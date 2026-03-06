use ratatui::prelude::*;

use crate::Hypertile;
use crate::types::PaneSnapshot;

/// [`StatefulWidget`] that calls a closure for each pane.
pub struct HypertileWidget<F>
where
    F: FnMut(PaneSnapshot, &mut Buffer),
{
    render_pane: F,
}

impl<F> HypertileWidget<F>
where
    F: FnMut(PaneSnapshot, &mut Buffer),
{
    /// The closure receives the pane snapshot and shared buffer.
    pub fn new(render_pane: F) -> Self {
        Self { render_pane }
    }
}

impl<F> StatefulWidget for HypertileWidget<F>
where
    F: FnMut(PaneSnapshot, &mut Buffer),
{
    type State = Hypertile;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.compute_layout(area);
        for pane in state.panes_iter() {
            (self.render_pane)(pane, buf);
        }
    }
}
