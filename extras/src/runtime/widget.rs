use ratatui::prelude::*;

use super::HypertileRuntime;

/// [`StatefulWidget`] wrapper for [`HypertileRuntime`](super::HypertileRuntime).
pub struct HypertileView;

impl StatefulWidget for HypertileView {
    type State = HypertileRuntime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.render(area, buf);
    }
}
