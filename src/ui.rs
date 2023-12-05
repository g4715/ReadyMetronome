// This is loosely based on the JSON Editor tutorial for ratatui
// A lot of this is taken wholesale from the ratatui tutorial and tweaked for Ready Metronome, I will comment
// on what each piece does. Tutorial found here https://ratatui.rs/tutorials/json-editor/ui/
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    menu::Menu,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

// This is the function to render the UI
pub fn ui(f: &mut Frame, app: &App, main_menu: &mut Menu, edit_menu: &mut Menu) {
    // This will define a layout in three sections with the middle one being resizeable
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title bar -------------------------------------------------------------------------------------------------------
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Ready Metronome",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // Main screen -----------------------------------------------------------------------------------------------------
    // For the main menu screen we will use a widgets::List and ListState which we define from items in main.rs
    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
    let main_items: Vec<ListItem> = main_menu
        .items
        .iter()
        .map(|i| ListItem::new(i.as_str()))
        .collect();
    let main_list = List::new(main_items)
        .block(
            Block::default()
                .title("Control Panel")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(active_style);

    let edit_items: Vec<ListItem> = edit_menu
        .items
        .iter()
        .map(|i| ListItem::new(i.as_str()))
        .collect();
    let edit_list = List::new(edit_items)
        .block(Block::default().title("Status").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(active_style);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[1]);

    f.render_stateful_widget(main_list, main_chunks[0], &mut main_menu.state);
    f.render_stateful_widget(edit_list, main_chunks[1], &mut edit_menu.state);

    // Bottom nav ------------------------------------------------------------------------------------------------------
    // It displays information about the current screen and controls for the user
    let current_navigation_text = vec![match app.current_screen {
        CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exiting => {
            Span::styled("Really Quit?", Style::default().fg(Color::LightRed))
        }
    }
    .to_owned()];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    // This displays the current keys the user can use
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "Use (arrow keys) to navigate, (enter) to select an option, or (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "Use (arrow keys) to navigate, (enter) to select an option, (tab) to go back to the main menu, or (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (n) to return to main menu",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    // Here is where we create the actual footer chunks for rendering, we pass the last chunks[] element (footer)
    // to split and render those. The screen name gets 25% of the length and the hints get 75%
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[2]);

    // Render the footer
    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
// Note: This is taken wholesale from the ratatui popup example: https://github.com/ratatui-org/ratatui/blob/main/examples/popup.rs
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
