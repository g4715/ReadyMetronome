// This is loosely based on the JSON Editor tutorial for ratatui
// A lot of this is taken wholesale from the ratatui tutorial and tweaked for Ready Metronome, I will comment
// on what each piece does. Tutorial found here https://ratatui.rs/tutorials/json-editor/ui/
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    menu::Menu,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

// This is the function to render the UI
pub fn ui(f: &mut Frame, app: &mut App, main_menu: &mut Menu, edit_menu: &mut Menu) {
    // Popup block to use for editing / quit dialog
    let popup_block = Block::default()
        .title("Editing Value")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Black));
    let area = centered_rect(50, 50, f.size());
    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
    let quit_style = Style::default().fg(Color::Red);
    let error_style = Style::default().fg(Color::Red);

    // This will define the overall layout in three sections with the middle one being resizeable
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

    // Editing Value Pop Up --------------------------------------------------------------------------------------------
    if let Some(editing) = app.currently_editing {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        f.render_widget(popup_block, area);

        // Here we create two layouts, one to split the pop up vertically into two slices, and another to split the top
        // slice into two slices to hold the original value and the new value currently being edited
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let sub_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        // Default to BPM editing, match cases for other settings
        let original_block;
        let key_block;
        let original_text;

        let alert_block = Block::default().title("Notification").borders(Borders::ALL);
        let alert_text = Paragraph::new(app.alert_string.clone().red()).block(alert_block);

        match editing {
            CurrentlyEditing::Volume => {
                key_block = Block::default()
                    .title("Enter New Volume")
                    .borders(Borders::ALL);
                original_block = Block::default()
                    .title("Current Volume")
                    .borders(Borders::ALL);
                original_text = Paragraph::new(app.get_volume().to_string()).block(original_block);
            }
            CurrentlyEditing::Bpm => {
                key_block = Block::default()
                    .title("Enter New Bpm")
                    .borders(Borders::ALL);
                original_block = Block::default().title("Current Bpm").borders(Borders::ALL);
                original_text = Paragraph::new(app.get_bpm().to_string()).block(original_block);
            }
        }

        let key_text =
            Paragraph::new(Span::styled(app.edit_string.clone(), active_style)).block(key_block);

        f.render_widget(original_text, sub_layout[0]);
        f.render_widget(key_text, sub_layout[1]);
        f.render_widget(alert_text, layout[1]);
    }

    // Quit pop up -----------------------------------------------------------------------------------------------------
    if app.current_screen == CurrentScreen::Exiting {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let quit_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let quit_block = Block::default().title("Quitting").borders(Borders::ALL);
        let quit_text = Paragraph::new(Span::styled(
            "Are you sure you wish to quit? y / n".to_string(),
            quit_style,
        ))
        .block(quit_block);
        f.render_widget(quit_text, quit_layout[0]);
    }

    // Bottom nav ------------------------------------------------------------------------------------------------------
    // It displays information about the current screen and controls for the user
    let current_navigation_text = vec![match app.current_screen {
        CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exiting => {
            Span::styled("Really Quit?", Style::default().fg(Color::LightRed))
        }
        CurrentScreen::Error => Span::styled("ERROR", Style::default().fg(Color::Red)),
    }
    .to_owned()];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    // This displays the current keys the user can use
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "Use (arrow keys) to navigate, (enter) to select an option, or (q) to quit",
                Style::default().fg(Color::Green),
            ),
            CurrentScreen::Editing => {
                if app.currently_editing.is_some() {
                    Span::styled("Please enter a new value. Press (enter) to save, (esc) to discard changes or (q) to quit", Style::default().fg(Color::Yellow))
                } else {
                    Span::styled("Use (arrow keys) to navigate, (enter) to select, (esc) to go to main menu, or (q) to quit", Style::default().fg(Color::Yellow))
                }
            }
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (n) to return to main menu",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Error => Span::styled(
                "Something went wrong! Please press 'q' to quit",
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

    // Error Pop Up ----------------------------------------------------------------------------------------------------
    if app.current_screen == CurrentScreen::Error {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let error_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let error_block = Block::default()
            .title("Unexpected ERROR!")
            .borders(Borders::ALL);
        let error_text = Paragraph::new(Span::styled(
            "Something went wrong! Please press 'q' to quit".to_string(),
            error_style,
        ))
        .block(error_block);
        f.render_widget(error_text, error_layout[0]);
    }
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
