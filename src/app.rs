use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};

use crate::process;
use crate::session::{self, Session};
use crate::warp;

pub struct App {
    pub sessions: Vec<Session>,
    pub selected: usize,
    pub effort_level: String,
    pub should_quit: bool,
    prev_sessions: HashMap<String, Session>,
}

impl App {
    pub fn new() -> Self {
        let effort_level = read_effort_level().unwrap_or_else(|| "medium".to_string());
        App {
            sessions: Vec::new(),
            selected: 0,
            effort_level,
            should_quit: false,
            prev_sessions: HashMap::new(),
        }
    }

    pub fn refresh(&mut self) {
        let procs = process::discover_claude_processes();
        let sessions = session::resolve_sessions(&procs, &self.prev_sessions);

        // Store for next incremental parse
        self.prev_sessions = sessions
            .iter()
            .map(|s| (s.session_id.clone(), s.clone()))
            .collect();

        self.sessions = sessions;

        // Clamp selection
        if self.selected >= self.sessions.len() && !self.sessions.is_empty() {
            self.selected = self.sessions.len() - 1;
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.sessions.is_empty() {
                    self.selected = (self.selected + 1).min(self.sessions.len() - 1);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Enter => {
                self.jump_to_session(self.selected);
            }
            KeyCode::Char('r') => {
                self.refresh();
            }
            KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                let n = c as u8 - b'0';
                self.jump_to_session((n - 1) as usize);
            }
            _ => {}
        }
    }

    fn jump_to_session(&self, idx: usize) {
        if idx < self.sessions.len() {
            warp::switch_to_tab_number((idx + 1) as u8);
        }
    }
}

fn read_effort_level() -> Option<String> {
    let home = dirs::home_dir()?;
    let path = home.join(".claude").join("settings.json");
    let content = std::fs::read_to_string(path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&content).ok()?;
    v.get("effortLevel")?.as_str().map(|s| s.to_string())
}
