use std::{fs, io};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::TableState;
use ratatui::{DefaultTerminal, Frame};

use crate::types::{DirectoryView, EntryInfo, FileEntry, View};
use crate::ui;

#[derive(Debug)]
pub struct App {
    pub exit: bool,
    selected_index: usize,
    path_list_selected_index: usize,
    table_state: TableState,
    current_view: View,
    directory_view: Option<DirectoryView>,
}

impl Default for App {
    fn default() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            exit: false,
            selected_index: 0,
            path_list_selected_index: 0,
            table_state,
            current_view: View::PathList,
            directory_view: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.current_view {
            View::PathList => ui::draw_path_list(frame, &mut self.table_state),
            View::DirectoryContents => {
                if let Some(directory_view) = &self.directory_view {
                    ui::draw_directory_contents(frame, &mut self.table_state, directory_view);
                }
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event)
                if key_event.kind == crossterm::event::KeyEventKind::Press =>
            {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let Some(directory_view) = &mut self.directory_view {
            if directory_view.search_mode && self.current_view == View::DirectoryContents {
                match key_event.code {
                    KeyCode::Char(c) => {
                        directory_view.search_query.push(c);
                        self.filter_contents();
                    }
                    KeyCode::Backspace => {
                        directory_view.search_query.pop();
                        if directory_view.search_query.is_empty() {
                            directory_view.filtered_contents.clear();
                            self.selected_index = 0;
                            self.table_state.select(Some(0));
                        } else {
                            self.filter_contents();
                        }
                    }
                    KeyCode::Enter => {
                        directory_view.search_mode = false;
                    }
                    KeyCode::Esc => {
                        self.clear_search();
                    }
                    _ => {}
                }
                return;
            }
        }

        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('/') => {
                if self.current_view == View::DirectoryContents {
                    self.start_search();
                }
            }
            KeyCode::Up => self.navigate_up(),
            KeyCode::Down => self.navigate_down(),
            KeyCode::Enter => {
                if self.current_view == View::PathList {
                    self.select_path();
                }
            }
            KeyCode::Esc => {
                if self.current_view == View::DirectoryContents {
                    if let Some(directory_view) = &self.directory_view {
                        if !directory_view.search_query.is_empty() {
                            self.clear_search();
                        } else {
                            self.go_back_to_path_list();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn navigate_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.table_state.select(Some(self.selected_index));
        }
    }

    fn navigate_down(&mut self) {
        let max_index = match self.current_view {
            View::PathList => std::env::var("PATH")
                .unwrap_or_else(|_| String::new())
                .split(':')
                .count()
                .saturating_sub(1),
            View::DirectoryContents => {
                if let Some(directory_view) = &self.directory_view {
                    directory_view
                        .get_display_contents()
                        .len()
                        .saturating_sub(1)
                } else {
                    0
                }
            }
        };

        if self.selected_index < max_index {
            self.selected_index += 1;
            self.table_state.select(Some(self.selected_index));
        }
    }

    fn select_path(&mut self) {
        let path_env = std::env::var("PATH").unwrap_or_else(|_| String::new());
        let paths: Vec<&str> = path_env.split(':').collect();

        if let Some(selected_path) = paths.get(self.selected_index) {
            // Save current PATH selection before switching views
            self.path_list_selected_index = self.selected_index;
            
            let mut directory_view = DirectoryView::new(selected_path.to_string());
            self.load_directory_contents(&mut directory_view);
            self.directory_view = Some(directory_view);
            self.current_view = View::DirectoryContents;
            self.selected_index = 0;
            self.table_state.select(Some(0));
        }
    }

    fn load_directory_contents(&mut self, directory_view: &mut DirectoryView) {
        directory_view.directory_contents.clear();

        match fs::read_dir(&directory_view.current_path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        let path = entry.path();
                        let is_symlink = path.is_symlink();
                        let info = if is_symlink {
                            if let Ok(target) = fs::read_link(&path) {
                                if let Some(target_str) = target.to_str() {
                                    EntryInfo::SymlinkTarget(target_str.to_string())
                                } else {
                                    EntryInfo::SymlinkTarget("<invalid UTF-8>".to_string())
                                }
                            } else {
                                EntryInfo::SymlinkTarget("<broken link>".to_string())
                            }
                        } else {
                            EntryInfo::None
                        };

                        directory_view.directory_contents.push(FileEntry {
                            name: name.to_string(),
                            is_symlink,
                            info,
                        });
                    }
                }
                directory_view
                    .directory_contents
                    .sort_by(|a, b| a.name.cmp(&b.name));
            }
            Err(err) => {
                directory_view.directory_contents.push(FileEntry {
                    name: "⚠️  Unable to read directory".to_string(),
                    is_symlink: false,
                    info: EntryInfo::Error(format!("Error: {}", err)),
                });
            }
        }
    }

    fn start_search(&mut self) {
        if let Some(directory_view) = &mut self.directory_view {
            directory_view.search_mode = true;
            directory_view.search_query.clear();
        }
    }

    fn filter_contents(&mut self) {
        if let Some(directory_view) = &mut self.directory_view {
            directory_view.filtered_contents = directory_view
                .directory_contents
                .iter()
                .filter(|entry| {
                    entry
                        .name
                        .to_lowercase()
                        .contains(&directory_view.search_query.to_lowercase())
                })
                .cloned()
                .collect();

            self.selected_index = 0;
            self.table_state.select(Some(0));
        }
    }

    fn clear_search(&mut self) {
        if let Some(directory_view) = &mut self.directory_view {
            directory_view.search_mode = false;
            directory_view.search_query.clear();
            directory_view.filtered_contents.clear();
            self.selected_index = 0;
            self.table_state.select(Some(0));
        }
    }

    fn go_back_to_path_list(&mut self) {
        self.current_view = View::PathList;
        // Restore the previously selected PATH entry
        self.selected_index = self.path_list_selected_index;
        self.table_state.select(Some(self.path_list_selected_index));
        self.directory_view = None;
    }
}
