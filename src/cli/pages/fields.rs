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
        components::{confirmation_modal::ConfirmationModal, field_modal::FieldModal},
        events::{
            confirmation_choice::ConfirmationChoice, pages::Pages, screen_action::ScreenAction,
        },
        state::fields_state::FieldsState,
        theme::color_scheme::ColorScheme,
    },
    models::model::{Field, Model},
};

pub struct Fields {
    pub selected_model: usize,
}

impl Fields {
    pub fn new(selected_model: usize) -> Self {
        Self { selected_model }
    }
}

impl Fields {
    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        state: &mut FieldsState,
        models: &mut Vec<Model>,
    ) -> ScreenAction {
        if let Some(modal) = &mut state.input_modal {
            match modal.handle_key(key) {
                ScreenAction::Back => {
                    state.input_modal = None;
                    ScreenAction::None
                }

                ScreenAction::ReturnField(field) => {
                    models[self.selected_model].fields.push(field);
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
                    models[self.selected_model]
                        .fields
                        .remove(state.list_state.selected().unwrap_or(2));
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

                (KeyCode::Char('q'), _) => ScreenAction::PreviousPage(Pages::Models),

                (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                    state.input_modal =
                        Some(FieldModal::new(models[self.selected_model].name.to_owned()));
                    ScreenAction::None
                }

                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    if state.list_state.selected() <= Some(1) {
                        ScreenAction::OpenError(String::from("You can't delete those fields!"))
                    } else {
                        state.delete_confirmation_modal = Some(ConfirmationModal::new(format!(
                            "Delete \"{}\"?",
                            models[self.selected_model].fields
                                [state.list_state.selected().unwrap_or(2)]
                            .name
                        )));
                        ScreenAction::None
                    }
                }

                _ => ScreenAction::None,
            }
        }
    }

    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut FieldsState,
        models: &mut Vec<Model>,
    ) {
        let centered_area: Rect =
            area.centered(Constraint::Percentage(50), Constraint::Percentage(50));
        let [list_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(centered_area);

        Block::bordered()
            .border_type(BorderType::Plain)
            .fg(ColorScheme::BABY_BLUE)
            .title(
                Span::from(format!(" {} Fields ", models[self.selected_model].name))
                    .into_right_aligned_line()
                    .bold(),
            )
            .title_bottom(Line::from_iter([
                Span::from(" ^a "),
                Span::from("add ")
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
            models[self.selected_model]
                .fields
                .iter()
                .map(|field: &Field| ListItem::from(format!(" {}", &field.name))),
        )
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bg(ColorScheme::BABY_BLUE)
                .fg(ColorScheme::INK_BLACK)
                .add_modifier(Modifier::BOLD),
        );

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);

        if let Some(modal) = state.input_modal.as_mut() {
            modal.render(area, buf);
        }

        if let Some(modal) = &state.delete_confirmation_modal {
            modal.render(area, buf);
        }
    }
}
