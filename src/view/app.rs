use std::path::PathBuf;

use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget},
};

// Custom imports
use crate::{
    models::model::{Field, Model},
    view::{
        color_scheme::ColorScheme,
        field_box::{FieldBox, FieldBoxState},
        model_box::{ModelBox, ModelBoxState},
        screen_action::ScreenAction,
    },
};

#[derive(Debug, Default)]
pub struct AppState {
    pub models: Vec<Model>,
    templates: Vec<TemplateSet>,
    list_state: ListState,
    selected_template: Option<TemplateSet>,
    selected_model: Option<usize>,
    models_state: ModelBoxState,
    fields_state: FieldBoxState,
}

#[derive(Debug, Default, Clone)]
struct TemplateSet {
    name: String,
}

pub fn run(mut terminal: DefaultTerminal, path: &str, name: &str) -> Result<()> {
    let template_set: TemplateSet = TemplateSet {
        name: String::from("Express - Sequelize"),
    };

    let mut list_state: ListState = ListState::default();
    list_state.select(Some(0));

    let mut state: AppState = AppState {
        models: vec![],
        templates: vec![template_set],
        list_state,
        selected_template: None,
        selected_model: None,
        models_state: ModelBoxState::default(),
        fields_state: FieldBoxState::default(),
    };

    loop {
        // Rendering
        terminal.draw(|frame: &mut Frame<'_>| render(frame, path, name, &mut state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if state.selected_model.is_some() {
                match handle_fields(key, &mut state) {
                    ScreenAction::Back => {
                        state.selected_model = None;
                    }

                    _ => {}
                }
            } else if state.models_state.add_mode {
                match handle_add_model(key, &mut state) {
                    ScreenAction::Back => {
                        state.models_state.input_buffer.clear();
                        state.models_state.add_mode = false;
                    }

                    ScreenAction::Confirm => {
                        state.models.push(Model {
                            name: state.models_state.input_buffer.clone(),
                            fields: vec![],
                        });
                        state.models_state.input_buffer.clear();
                        state.models_state.add_mode = false;
                    }

                    _ => {}
                }
            } else if state.selected_template.is_some() {
                match handle_models(key, &mut state) {
                    ScreenAction::Back => {
                        state.selected_template = None;
                    }

                    _ => {}
                }
            } else {
                match handle_key(key, &mut state) {
                    ScreenAction::Exit => break,

                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn handle_key(key: KeyEvent, state: &mut AppState) -> ScreenAction {
    match key.code {
        event::KeyCode::Esc => ScreenAction::Exit,

        event::KeyCode::Enter => {
            if let Some(index) = state.list_state.selected() {
                state.selected_template = Some(state.templates[index].clone());
            }
            ScreenAction::None
        }

        event::KeyCode::Char('k') => {
            state.list_state.select_previous();
            ScreenAction::None
        }

        event::KeyCode::Char('j') => {
            state.list_state.select_next();
            ScreenAction::None
        }

        _ => ScreenAction::None,
    }
}

fn handle_models(key: KeyEvent, state: &mut AppState) -> ScreenAction {
    match key.code {
        event::KeyCode::Esc => ScreenAction::Back,

        event::KeyCode::Enter => {
            state.selected_model = Some(state.models_state.selected());
            ScreenAction::None
        }

        event::KeyCode::Char('A') => {
            state.models_state.add_mode = true;
            ScreenAction::None
        }

        event::KeyCode::Char('D') => {
            state.models.remove(state.models_state.selected());
            ScreenAction::None
        }

        event::KeyCode::Char('C') => ScreenAction::Confirm,

        event::KeyCode::Char('k') => {
            state.models_state.select_previous();
            ScreenAction::None
        }

        event::KeyCode::Char('j') => {
            state.models_state.select_next();
            ScreenAction::None
        }

        _ => ScreenAction::None,
    }
}

fn handle_add_model(key: KeyEvent, state: &mut AppState) -> ScreenAction {
    match key.code {
        event::KeyCode::Enter => {
            if !state.models_state.input_buffer.trim().is_empty()
                && !state
                    .models_state
                    .input_buffer
                    .strip_prefix('_')
                    .is_some_and(|s| !s.is_empty())
            {
                state.models_state.input_buffer =
                    if let Some(value) = state.models_state.input_buffer.trim().strip_prefix('_') {
                        value.to_string()
                    } else {
                        state.models_state.input_buffer.clone()
                    };
                return ScreenAction::Confirm;
            }
            ScreenAction::Back
        }

        event::KeyCode::Esc => ScreenAction::Back,

        event::KeyCode::Char(c) => {
            if c != ' ' && state.models_state.input_buffer.len() < 16 {
                state.models_state.input_buffer.push(c.to_ascii_lowercase());
            } else {
                state.models_state.input_buffer.push('_');
            }

            ScreenAction::None
        }

        event::KeyCode::Backspace => {
            state.models_state.input_buffer.pop();
            ScreenAction::None
        }

        _ => ScreenAction::None,
    }
}

fn handle_fields(key: KeyEvent, state: &mut AppState) -> ScreenAction {
    match key.code {
        event::KeyCode::Esc => ScreenAction::Back,

        event::KeyCode::Char('A') => {
            state.fields_state.add_mode = true;
            ScreenAction::None
        }

        event::KeyCode::Char('D') => {
            // TODO: Handle delete field
            ScreenAction::None
        }

        event::KeyCode::Char('k') => {
            state.fields_state.select_previous();
            ScreenAction::None
        }

        event::KeyCode::Char('j') => {
            state.fields_state.select_next();
            ScreenAction::None
        }

        _ => ScreenAction::None,
    }
}

fn render(frame: &mut Frame, path: &str, name: &str, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [reminder_template, vertical_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(16)])
            .flex(Flex::Center)
            .areas(border_area);

    let [template_area] = Layout::horizontal([Constraint::Length(36)])
        .flex(Flex::Center)
        .areas(vertical_area);

    let [list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(template_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(ColorScheme::Orange.color())
        .title(
            Span::from(" | ⚙️ Yenterick's Junkyard | ")
                .bold()
                .into_centered_line(),
        )
        .title_bottom(Line::from_iter([
            Span::from(" esc ").bold(),
            Span::styled(
                if app_state.selected_model.is_some() || app_state.selected_template.is_some() {
                    "back "
                } else {
                    "exit "
                },
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  h ").bold(),
            Span::styled(
                "left ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  j ").bold(),
            Span::styled(
                "down ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  k ").bold(),
            Span::styled(
                "up ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  l ").bold(),
            Span::styled(
                "right ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  ↵ ").bold(),
            Span::styled(
                "confirm ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from(" "),
        ]))
        .render(border_area, frame.buffer_mut());

    if let Some(index) = app_state.selected_model {
        frame.render_stateful_widget(
            FieldBox {
                model: &app_state.models[index],
            },
            vertical_area,
            &mut app_state.fields_state,
        );
    } else if app_state.selected_template.is_some() {
        frame.render_stateful_widget(
            ModelBox {
                models: &app_state.models,
            },
            vertical_area,
            &mut app_state.models_state,
        );
    } else {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(ColorScheme::Blue.color())
            .title(
                Span::from(" | 📃 Template Selection | ")
                    .bold()
                    .into_centered_line(),
            )
            .title_bottom(Span::from("...").bold().into_centered_line())
            .render(template_area, frame.buffer_mut());

        let list: List = List::new(
            app_state
                .templates
                .iter()
                .map(|template: &TemplateSet| ListItem::from(template.name.as_str())),
        )
        .highlight_symbol(" > ")
        .scroll_padding(1)
        .highlight_style(
            Style::default()
                .fg(ColorScheme::White.color())
                .bg(ColorScheme::Green.color())
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        );

        frame.render_stateful_widget(list, list_area, &mut app_state.list_state);
    }

    let absolute_path = std::env::current_dir()
        .map(|dir| dir.join(path))
        .unwrap_or_else(|_| PathBuf::from(path));

    let reminder = Paragraph::new(Line::from(Span::styled(
        format!(
            " Generating \"{}\" on : 📁 {} ",
            name,
            absolute_path.display()
        ),
        Style::default()
            .fg(ColorScheme::White.color())
            .bg(ColorScheme::Orange.color()),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(reminder, reminder_template);
}
