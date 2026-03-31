use ratatui::widgets::ListState;
use crate::git::TagInfo;
use semver::Version;

pub struct App {
    pub should_quit: bool,
    pub list_state: ListState,
    pub tags: Vec<TagInfo>,
}

impl App {
    pub fn new(tags: Vec<TagInfo>) -> Self {
        let mut app = Self {
            should_quit: false,
            list_state: ListState::default(),
            tags,
        };
        app.sort_by_date();
        if !app.tags.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tags.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tags.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn sort_by_date(&mut self) {
        self.tags.sort_by(|a, b| b.date.cmp(&a.date));
    }

    pub fn sort_by_semver(&mut self) {
        self.tags.sort_by(|a, b| {
            let a_v = Version::parse(a.name.trim_start_matches('v'));
            let b_v = Version::parse(b.name.trim_start_matches('v'));
            match (a_v, b_v) {
                (Ok(av), Ok(bv)) => bv.cmp(&av),
                (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                (Err(_), Err(_)) => b.name.cmp(&a.name),
            }
        });
    }
}
