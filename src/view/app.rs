use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Ok, Result};
use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget, Wrap},
};

// Custom imports
use crate::{
    models::model::Model,
    view::{
        color_scheme::ColorScheme,
        model_box::ModelBox,
        screen_action::{self, ScreenAction},
    },
};

#[derive(Debug, Default)]
struct AppState {
    models: Vec<Model>,
    templates: Vec<TemplateSet>,
    list_state: ListState,
    selected_template: Option<TemplateSet>,
    models_state: ListState,
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

    let mut models_state: ListState = ListState::default();
    models_state.select(Some(0));

    let mut state: AppState = AppState {
        models: vec![],
        templates: vec![template_set],
        list_state,
        selected_template: None,
        models_state: models_state,
    };

    loop {
        // Rendering
        terminal.draw(|frame: &mut Frame<'_>| render(frame, path, name, &mut state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if state.selected_template.is_some() {
                match handle_add_models(key, &mut state) {
                    ScreenAction::Back => {
                        state.selected_template = None;
                    }

                    ScreenAction::None => {}
                    _ => {}
                }
            } else {
                match handle_key(key, &mut state) {
                    ScreenAction::Exit => break,
                    ScreenAction::None => {}
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

fn handle_add_models(key: KeyEvent, state: &mut AppState) -> ScreenAction {
    match key.code {
        event::KeyCode::Esc => ScreenAction::Back,

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

fn render(frame: &mut Frame, path: &str, name: &str, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [reminder_template, vertical_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(16)])
            .flex(Flex::Center)
            .areas(border_area);

    let [template_area] = Layout::horizontal([Constraint::Length(32)])
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
                "back ",
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

    if let Some(_) = &app_state.selected_template {
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

        let list: List<'_> = List::new(
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
        format!(" Generating \"{}\" on : 📁 {} ", name, absolute_path.display()),
        Style::default()
            .fg(ColorScheme::White.color())
            .bg(ColorScheme::Orange.color()),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(reminder, reminder_template);
}
