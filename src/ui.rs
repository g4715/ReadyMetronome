// This is loosely based on the JSON Editor tutorial for ratatui
// A lot of this is taken wholesale from the ratatui tutorial and tweaked for Ready Metronome, I will comment 
// on what each piece does. Tutorial found here https://ratatui.rs/tutorials/json-editor/ui/

use crate::app::{App, CurrentScreen, CurrentlyEditing};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// This is the function to render the UI
pub fn ui(f: &mut Frame, app: &App, list_state: &mut ListState, items: &Vec<String>) {
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
    let items2: Vec<ListItem>= items.iter().map(|i| ListItem::new(i.as_str())).collect();
    let list = List::new(items2)
        .block(Block::default().title("Control Panel").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[1]);

    // Get the current status of the metronome and display that on the right panel
    let status_block = Block::default()
        .borders(Borders::ALL)
        .title("Status")
        .style(Style::default());

    let current_bpm = "bpm: ".to_owned() + &app.settings.bpm.load(Ordering::Relaxed).to_string();
    let ms_delay = "millisecond delay: ".to_owned() + &app.settings.ms_delay.load(Ordering::Relaxed).to_string();
    let volume = "volume: ".to_owned() + &app.settings.volume.load(Ordering::Relaxed).to_string();
    let mut is_playing = "playing: ".to_owned();

    if app.settings.is_running.load(Ordering::Relaxed) == true {
        is_playing = is_playing + "yes";
    } else {
        is_playing = is_playing + "no";
    }

    let status_readout = is_playing + "\n" + &current_bpm + "\n" + &volume + "\n" + &ms_delay;

    let status_text = Paragraph::new(Text::styled(
        status_readout, 
        Style::default(),
    ))
    .block(status_block);


    f.render_stateful_widget(list, main_chunks[0], list_state);
    f.render_widget(status_text, main_chunks[1]);

    // Bottom nav ------------------------------------------------------------------------------------------------------
    // It displays information about the current screen and controls for the user
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => {
                Span::styled("Really Quit?", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
    ];

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
