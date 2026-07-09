use ratatui::widgets::ListState;

use crate::cli::components::{confirmation_modal::ConfirmationModal, input_modal::InputModal};

pub struct ModelsState {
    pub list_state: ListState,
    pub selected_model: Option<usize>,
    pub input_modal: Option<InputModal>,
    pub delete_confirmation_modal: Option<ConfirmationModal>,
}

impl ModelsState {
    pub fn new() -> ModelsState {
        let mut status: ModelsState = ModelsState {
            list_state: ListState::default(),
            selected_model: None,
            input_modal: None,
            delete_confirmation_modal: None,
        };

        status.list_state.select_first();
        status
    }
}
