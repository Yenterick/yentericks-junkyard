use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::cli::{
    components::{confirmation_modal::ConfirmationModal, outline::Outline, sidebar::Sidebar},
    events::{confirmation_choice::ConfirmationChoice, pages::Pages, screen_action::ScreenAction},
    models::template::Template,
    pages::{page::Page, template_selection::TemplateSelection},
    state::app_state::AppState,
    theme::color_scheme::ColorScheme,
};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state: AppState = AppState::new();
    let mut template_selection_page: TemplateSelection = TemplateSelection {
        templates: vec![
            Template {
                name: String::from("Express - Sequelize"),
                key: 0,
            },
            Template {
                name: String::from("Example 2"),
                key: 1,
            },
        ],
    };

    loop {
        terminal.draw(|frame: &mut Frame<'_>| {
            render(frame, &mut app_state, &mut template_selection_page)
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if let Some(modal) = &mut app_state.confirmation_modal {
                modal.handle_key(key);

                match modal.choice().unwrap_or(ConfirmationChoice::No) {
                    ConfirmationChoice::Yes => {
                        break;
                    }

                    ConfirmationChoice::No => {
                        app_state.confirmation_modal = None;
                    }
                }
            }

            match template_selection_page.handle_key(key, &mut app_state.template_selection_state) {
                ScreenAction::NextPage(page) => {
                    app_state.sidebar_state.current_page = page;
                }

                _ => {}
            }

            match (key.code, key.modifiers) {
                (event::KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    app_state.confirmation_modal = Some(ConfirmationModal::new(String::from(
                        "Are you sure you want to exit?",
                    )))
                }

                _ => {}
            }
        }
    }

    Ok(())
}

fn render(
    frame: &mut Frame,
    app_state: &mut AppState,
    template_selection_page: &mut TemplateSelection,
) {
    let [complete] = Layout::horizontal([Constraint::Fill(1)]).areas(frame.area());

    let [sidebar, page_content] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)])
        .margin(1)
        .areas(complete);

    Block::new()
        .borders(Borders::RIGHT)
        .border_type(BorderType::Plain)
        .fg(ColorScheme::AIR_FORCE_BLUE)
        .render(sidebar, frame.buffer_mut());

    Outline.render(complete, frame.buffer_mut());
    Sidebar.render(sidebar, frame.buffer_mut(), &mut app_state.sidebar_state);

    if let Some(modal) = &app_state.confirmation_modal {
        modal.render(complete, frame.buffer_mut());
    }

    match &app_state.sidebar_state.current_page {
        Pages::TemplateSelection => {
            template_selection_page.render(
                page_content,
                frame.buffer_mut(),
                &mut app_state.template_selection_state,
            );
        }

        _ => {}
    }
}
