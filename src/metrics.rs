pub mod cadence;

use ratatui::{Frame, layout::Rect};

pub trait RenderMetric {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub struct MetricsReport {}
