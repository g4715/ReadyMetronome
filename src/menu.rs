// The purpose of this file is to house the main functionality for selectable menus

// References
// List state / Menu reference: https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html
// List: https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html

use ratatui::widgets::ListState;

pub struct Menu {
    pub items: Vec<String>,
    pub state: ListState,
}

// This provides a struct to hold a selectable menu state. See stateful widget reference above.
impl Menu {
    pub fn new(items: Vec<String>) -> Menu {
        Menu {
            items,
            state: ListState::default(),
        }
    }
    // Resets the menu items and selects the first on the list
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
        self.state.select(Some(0));
    }
    // Select the next item in the list
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    // Select the previous item in the list
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    // Deselect (unused for now)
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn select(&mut self, index :usize) {
        self.state.select(Some(index));
    }
}