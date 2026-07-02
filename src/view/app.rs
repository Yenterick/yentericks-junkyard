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

#[derive(Debug, Default)]
struct AppState {
    templates: Vec<TemplateSet>,
    list_state: ListState,
    selected_template: Option<TemplateSet>,
}

#[derive(Debug, Default, Clone)]
struct TemplateSet {
    name: String,
}

// Custom imports
use crate::view::{
    color_scheme::ColorScheme,
    screen_action::{self, ScreenAction},
};

pub fn run(mut terminal: DefaultTerminal, path: &str) -> Result<()> {
    let template_set: TemplateSet = TemplateSet {
        name: String::from("Express - Sequelize"),
    };

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let mut state: AppState = AppState {
        templates: vec![template_set],
        list_state,
        selected_template: None,
    };

    loop {
        // Rendering
        terminal.draw(|frame: &mut Frame<'_>| render(frame, path, &mut state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if state.selected_template.is_some() {
                match handle_add_models(key) {
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

fn handle_add_models(key: KeyEvent) -> ScreenAction {
    match key.code {
        event::KeyCode::Esc => ScreenAction::Back,
        _ => ScreenAction::None,
    }
}

fn render(frame: &mut Frame, path: &str, app_state: &mut AppState) {
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

    if let Some(template) = &app_state.selected_template {
        // pass
    } else {
        let absolute_path = std::env::current_dir()
            .map(|dir| dir.join(path))
            .unwrap_or_else(|_| PathBuf::from(path));

        let reminder = Paragraph::new(Line::from(Span::styled(
            format!(" Working Dir: 📁 {} ", absolute_path.display()),
            Style::default()
                .fg(ColorScheme::White.color())
                .bg(ColorScheme::Orange.color()),
        )))
        .alignment(Alignment::Center);

        frame.render_widget(reminder, reminder_template);

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

        let list = List::new(
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
}
