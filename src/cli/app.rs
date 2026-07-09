use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, BorderType, Borders, Clear, StatefulWidget, Widget},
};

use crate::cli::{
    components::{
        confirmation_modal::ConfirmationModal, error_modal::ErrorModal, outline::Outline,
        sidebar::Sidebar,
    },
    events::{confirmation_choice::ConfirmationChoice, pages::Pages, screen_action::ScreenAction},
    pages::{models::Models, page::Page, template_selection::TemplateSelection},
    state::app_state::AppState,
    theme::color_scheme::ColorScheme,
};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state: AppState = AppState::new();
    let mut template_selection_page: TemplateSelection = TemplateSelection::new();
    let mut models_page: Models = Models::new();

    loop {
        terminal.draw(|frame: &mut Frame<'_>| {
            render(
                frame,
                &mut app_state,
                &mut template_selection_page,
                &mut models_page,
            )
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if let Some(modal) = &mut app_state.error_modal {
                match modal.handle_key(key) {
                    ScreenAction::Confirm => {
                        app_state.error_modal = None;
                        continue;
                    }

                    _ => {}
                }
            }

            match (key.code, key.modifiers) {
                (event::KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    app_state.confirmation_modal = Some(ConfirmationModal::new(String::from(
                        "Are you sure you want to exit?",
                    )));
                    continue;
                }

                _ => {}
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

            match app_state.sidebar_state.current_page {
                Pages::TemplateSelection => match template_selection_page
                    .handle_key(key, &mut app_state.template_selection_state)
                {
                    ScreenAction::NextPage(page) => {
                        app_state.sidebar_state.go_to(page);
                    }

                    _ => {}
                },

                Pages::Models => match models_page.handle_key(key, &mut app_state.models_state) {
                    ScreenAction::PreviousPage(page) => {
                        app_state.sidebar_state.go_back(page);
                    }

                    ScreenAction::OpenError(error) => {
                        app_state.error_modal = Some(ErrorModal::new(String::from(error)));
                    }

                    _ => {}
                },

                Pages::Fields => todo!(),
                Pages::ProjectConfiguration => todo!(),
                Pages::Generation => todo!(),
            }
        }
    }

    Ok(())
}

fn render(
    frame: &mut Frame,
    app_state: &mut AppState,
    template_selection_page: &mut TemplateSelection,
    models_page: &mut Models,
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

    match &app_state.sidebar_state.current_page {
        Pages::TemplateSelection => {
            template_selection_page.render(
                page_content,
                frame.buffer_mut(),
                &mut app_state.template_selection_state,
            );
        }

        Pages::Models => models_page.render(
            page_content,
            frame.buffer_mut(),
            &mut app_state.models_state,
        ),

        _ => {}
    }

    if let Some(modal) = &app_state.error_modal {
        modal.render(complete, frame.buffer_mut());
    }

    if let Some(modal) = &app_state.confirmation_modal {
        modal.render(complete, frame.buffer_mut());
    }
}
