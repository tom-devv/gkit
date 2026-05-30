use ratatui::{
    layout::Layout,
    widgets::{Block, Borders, RenderDirection},
};

use crate::{metrics::RenderMetric, tui::state::State};

#[derive(Default)]
pub struct OverviewMetric {}

impl RenderMetric for OverviewMetric {
    fn render(
        &self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        state: &State,
    ) {
        let block_widg = Block::default()
            .title("Overview")
            .title_alignment(ratatui::layout::HorizontalAlignment::Center)
            .borders(Borders::ALL);

        frame.render_widget(block_widg, area);
    }
}
