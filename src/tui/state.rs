use crate::error::Result;
use crate::tui::ui::Page;

pub struct State {
    pub loading: bool,
    pub page: Page,
}

impl State {
    /**
     * By default new stats will be loading
     */
    pub fn new() -> Result<State> {
        let state = State {
            loading: true,
            page: Page::Overview,
        };
        Ok(state)
    }

    pub fn tab(&mut self) -> Page {
        let next_page = match self.page {
            Page::Overview => Page::Cadence,
            Page::Cadence => Page::Todo,
            Page::Todo => Page::Overview,
        };
        self.page = next_page;
        next_page
    }
}
