use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, StatefulWidget, Widget},
};

use crate::{
    cli::{
        components::{confirmation_modal::ConfirmationModal, input_modal::InputModal},
        events::{
            confirmation_choice::ConfirmationChoice, pages::Pages, screen_action::ScreenAction,
        },
        pages::page::Page,
        state::models_state::ModelsState,
        theme::color_scheme::ColorScheme,
    },
    models::model::Model,
};

pub struct Models {
    pub models: Vec<Model>,
}

impl Models {
    pub fn new() -> Models {
        Models { models: Vec::new() }
    }
}

impl Page for Models {
    type State = ModelsState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        if let Some(modal) = &mut state.input_modal {
            match modal.handle_key(key) {
                ScreenAction::Back => {
                    state.input_modal = None;
                    ScreenAction::None
                }

                ScreenAction::ReturnInput(input) => {
                    self.models.push(Model::new(input));
                    state.list_state.select_last();
                    state.input_modal = None;
                    ScreenAction::None
                }

                ScreenAction::OpenError(error) => ScreenAction::OpenError(error),

                _ => ScreenAction::None,
            }
        } else if let Some(modal) = &mut state.delete_confirmation_modal {
            modal.handle_key(key);

            match modal.choice().unwrap_or(ConfirmationChoice::No) {
                ConfirmationChoice::Yes => {
                    self.models.remove(state.list_state.selected().unwrap_or(0));
                    state.delete_confirmation_modal = None;
                    ScreenAction::None
                }

                ConfirmationChoice::No => {
                    state.delete_confirmation_modal = None;
                    ScreenAction::None
                }
            }
        } else {
            match (key.code, key.modifiers) {
                (KeyCode::Char('j'), _) => {
                    state.list_state.select_next();
                    ScreenAction::None
                }

                (KeyCode::Char('k'), _) => {
                    state.list_state.select_previous();
                    ScreenAction::None
                }

                (KeyCode::Char('q'), _) => ScreenAction::PreviousPage(Pages::TemplateSelection),

                (KeyCode::Enter, _) => {
                    state.selected_model = state.list_state.selected();
                    ScreenAction::NextPage(Pages::Fields)
                }

                (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                    state.input_modal =
                        Some(InputModal::new(String::from("Insert the new model name: ")));
                    ScreenAction::None
                }

                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    if self.models.len() < 1 {
                        ScreenAction::OpenError(String::from(
                            "You need to have at least one model!",
                        ))
                    } else {
                        ScreenAction::NextPage(Pages::ProjectConfiguration)
                    }
                }

                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    if self.models.len() < 1 {
                        ScreenAction::OpenError(String::from("There's nothing to delete!"))
                    } else {
                        state.delete_confirmation_modal = Some(ConfirmationModal::new(format!(
                            "Delete \"{}\"?",
                            self.models[state.list_state.selected().unwrap_or(0)].name
                        )));
                        ScreenAction::None
                    }
                }

                _ => ScreenAction::None,
            }
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let centered_area: Rect =
            area.centered(Constraint::Percentage(50), Constraint::Percentage(50));
        let [list_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(centered_area);

        Block::bordered()
            .border_type(BorderType::Plain)
            .fg(ColorScheme::BABY_BLUE)
            .title(Span::from(" Models ").into_right_aligned_line().bold())
            .title_bottom(Line::from_iter([
                Span::from(" ^a "),
                Span::from("add ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::BABY_BLUE),
                Span::from(" ^c "),
                Span::from("confirm ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::BABY_BLUE),
                Span::from(" ^d "),
                Span::from("delete ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::BABY_BLUE),
                Span::from(" "),
            ]))
            .render(centered_area, buf);

        let list: List = List::new(
            self.models
                .iter()
                .map(|model: &Model| ListItem::from(format!(" {}", &model.name))),
        )
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bg(ColorScheme::BABY_BLUE)
                .fg(ColorScheme::INK_BLACK)
                .add_modifier(Modifier::BOLD),
        );

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);

        if let Some(modal) = &state.input_modal {
            modal.render(area, buf);
        }

        if let Some(modal) = &state.delete_confirmation_modal {
            modal.render(area, buf);
        }
    }
}
