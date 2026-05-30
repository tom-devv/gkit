pub mod cadence;
pub mod overview;

use ratatui::{Frame, layout::Rect};

use crate::tui::state::State;

pub trait RenderMetric {
    fn render(&self, frame: &mut Frame, area: Rect, state: &State);
}
