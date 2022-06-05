use crossterm::event::KeyCode;
use tui::widgets::{TableState};
use std::sync::RwLock;
use tui::{
    widgets::Cell,
};

const ITEM: usize = 3;

pub struct Hosts {
    pub table_state: TableState,
    hosts: RwLock<Vec<Host>>
}

#[derive(Clone)]
pub struct Host{
    pub host: String,
    pub address: String,
    pub slected: bool,
}

impl Hosts {
    pub fn new() -> Self {

        let hosts = vec!(
            Host { host: "192.168.200.1".to_string(), address:"www.facebook.com".to_string(), slected: false },
            Host { host: "192.168.200.22".to_string(), address:"www.instagrams.com".to_string(), slected: false },
            Host { host: "192.168.200.33".to_string(), address:"loclahost".to_string(), slected: true },
        );


        Self {
            table_state: Default::default(),
            hosts: RwLock::new(hosts),
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => self.previous(),
            KeyCode::Down => self.next(),
            KeyCode::Enter => self.toggle_selected_host(),
            _ => {}
        }
    }

    pub fn get_hosts(&self) -> Vec<Host> {
        self.hosts.read().unwrap().clone()
    }


    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= ITEM - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    ITEM - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn toggle_selected_host(&self) {
        match self.table_state.selected() {
            Some(i) => {
                self.hosts.write()
                    .unwrap()
                    .get_mut(i)
                    .map( | host|
                        {
                            host.slected = !host.slected;
                            host
                        }
                    );
            },
            None => ()
        };
    }
}


impl Host {
    pub fn into_cells(&self) -> Vec<Cell> {
        let mut vec = Vec::new();
        vec.push(Cell::from(self.host.clone()));
        vec.push(Cell::from(self.address.clone()));
        vec.push(Cell::from(format!("{}",self.slected)));
        vec
    }
}