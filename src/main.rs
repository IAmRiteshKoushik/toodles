use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = AppState {
        is_add_new: false,
        ..Default::default()
    };

    // Sample data to check the list contents
    // state.items.push(TodoItem {
    //     is_done: false,
    //     description: String::from("Finish app dev"),
    // });
    // state.items.push(TodoItem {
    //     is_done: false,
    //     description: String::from("Finish web dev"),
    // });
    // state.items.push(TodoItem {
    //     is_done: false,
    //     description: String::from("Finish ai dev"),
    // });
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

// Main loop
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        let _ = terminal.draw(|f| render(f, app_state));

        // Input handling
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                        app_state.input_value.clear();
                    }
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        _ => {}
    }
    FormAction::None
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char(char) => match char {
            'a' => {
                app_state.is_add_new = true;
            }
            'd' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
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
    false
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    if app_state.is_add_new {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .title(" Input Description".to_span().into_centered_line())
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(border_area, frame.buffer_mut());
    } else {
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .title(" Todo List".to_span().into_centered_line())
            .border_type(BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());

        let list = List::new(
            app_state
                .items
                .iter()
                .map(|x| ListItem::from(x.description.clone())),
        )
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Green));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
