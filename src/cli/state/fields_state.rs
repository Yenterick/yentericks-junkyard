use ratatui::widgets::ListState;

use crate::cli::components::{confirmation_modal::ConfirmationModal, field_modal::FieldModal};

pub struct FieldsState {
    pub list_state: ListState,
    pub selected_field: Option<usize>,
    pub input_modal: Option<FieldModal>,
    pub delete_confirmation_modal: Option<ConfirmationModal>,
}

impl FieldsState {
    pub fn new() -> FieldsState {
        let mut status: FieldsState = FieldsState {
            list_state: ListState::default(),
            selected_field: None,
            input_modal: None,
            delete_confirmation_modal: None,
        };

        status.list_state.select(Some(2));
        status
    }
}
