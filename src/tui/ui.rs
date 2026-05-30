use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Tabs,
};

use crate::tui::state::State;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum Page {
    #[default]
    Overview = 0, // Numbering is for easy recogonition of order on the top bar
    Cadence = 1,
    Todo = 2,
}

impl Page {
    pub fn to_tab(&self) -> &'static str {
        match self {
            Page::Overview => "Overview",
            Page::Cadence => "Cadence",
            Page::Todo => "Todo",
        }
    }

    pub fn size() -> u16 {
        3 // todo fix this?
    }
}

pub fn nav(state: &State) -> Tabs<'static> {
    let ordered_pages = [Page::Overview, Page::Cadence, Page::Todo];
    let tab_titles = ordered_pages.iter().map(|page| page.to_tab());
    let tabs = Tabs::new(tab_titles)
        .select(state.page as usize)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        );

    tabs
}
