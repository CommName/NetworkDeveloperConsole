use self::hosts::Hosts;
use crossterm::event::{KeyCode};

pub mod hosts;

pub struct App {
    pub tab_index: ViewState,
    pub hosts: Hosts
}


impl App {

    pub fn new() -> Self {
        Self {
            tab_index: ViewState::NetworkSelection,
            hosts: Hosts::new()
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) {
        match self.tab_index {
            ViewState::NetworkSelection => self.hosts.handle_key_code(code),
            _ => {}
        }
    }
}

pub enum ViewState {
    NetworkSelection,
    NetworkView,
    APIView,
}