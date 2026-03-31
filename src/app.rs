use ratatui::widgets::ListState;
use crate::git::{TagInfo, CommitInfo};
use crate::config::Config;
use git2::Repository;
use semver::Version;

pub struct App {
    pub should_quit: bool,
    pub list_state: ListState,
    pub tags: Vec<TagInfo>,
    pub selected_base: Option<usize>,
    pub compare_mode: bool,
    pub commits_between: Vec<CommitInfo>,
    pub config: Config,
}

impl App {
    pub fn new(tags: Vec<TagInfo>, config: Config) -> Self {
        let mut app = Self {
            should_quit: false,
            list_state: ListState::default(),
            tags,
            selected_base: None,
            compare_mode: false,
            commits_between: Vec::new(),
            config,
        };
        app.sort_by_date();
        if !app.tags.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }

    pub fn toggle_compare(&mut self, repo: &Repository) {
        if self.compare_mode {
            self.compare_mode = false;
            self.selected_base = None;
            self.commits_between.clear();
        } else {
            if let Some(selected) = self.list_state.selected() {
                if let Some(base_idx) = self.selected_base {
                    if base_idx != selected {
                        let from = self.tags[base_idx].commit_id;
                        let to = self.tags[selected].commit_id;
                        if let Ok(commits) = crate::git::get_commits_between(repo, from, to) {
                            self.commits_between = commits;
                            self.compare_mode = true;
                        }
                    }
                } else {
                    self.selected_base = Some(selected);
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use git2::Oid;

    fn mock_tag(name: &str, days_ago: i64) -> TagInfo {
        TagInfo {
            name: name.to_string(),
            commit_id: Oid::from_bytes(&[0; 20]).unwrap(),
            message: None,
            author: None,
            date: Utc::now() - chrono::Duration::days(days_ago),
        }
    }

    #[test]
    fn test_sort_by_date() {
        let tags = vec![
            mock_tag("old", 10),
            mock_tag("new", 1),
            mock_tag("mid", 5),
        ];
        let mut app = App::new(tags, Config::default());
        app.sort_by_date();
        assert_eq!(app.tags[0].name, "new");
        assert_eq!(app.tags[1].name, "mid");
        assert_eq!(app.tags[2].name, "old");
    }

    #[test]
    fn test_sort_by_semver() {
        let tags = vec![
            mock_tag("v1.2.0", 0),
            mock_tag("v1.10.0", 0),
            mock_tag("v1.2.1", 0),
        ];
        let mut app = App::new(tags, Config::default());
        app.sort_by_semver();
        assert_eq!(app.tags[0].name, "v1.10.0");
        assert_eq!(app.tags[1].name, "v1.2.1");
        assert_eq!(app.tags[2].name, "v1.2.0");
    }

    #[test]
    fn test_sort_custom_convention() {
        let tags = vec![
            mock_tag("2026.3.40-staging", 2), // 2 days ago
            mock_tag("2026.3.1-prod", 1),    // 1 day ago
            mock_tag("2026.3.41-staging", 0), // Today
        ];
        let mut app = App::new(tags, Config::default());
        
        // Test date sorting (newest first)
        app.sort_by_date();
        assert_eq!(app.tags[0].name, "2026.3.41-staging");
        assert_eq!(app.tags[1].name, "2026.3.1-prod");
        assert_eq!(app.tags[2].name, "2026.3.40-staging");

        // Test semver sorting (newest version first)
        // Semver will see 41 > 40 > 1
        app.sort_by_semver();
        assert_eq!(app.tags[0].name, "2026.3.41-staging");
        assert_eq!(app.tags[1].name, "2026.3.40-staging");
        assert_eq!(app.tags[2].name, "2026.3.1-prod");
    }
}
