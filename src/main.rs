use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;

    // Sample data to check the list contents
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish app dev"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish app dev"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish app dev"),
    });

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

// Main loop
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state));
        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Exiting if Esc key is pressed
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'k' => {
                        app_state.list_state.select_previous();
                    }
                    'j' => {
                        app_state.list_state.select_next();
                    }
                    _ => {}
                },
                // No action if any other key is pressed
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    )
    .highlight_style(Style::default().fg(Color::Green));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
