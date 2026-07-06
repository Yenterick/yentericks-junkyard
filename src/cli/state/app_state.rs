use crate::cli::components::confirmation_modal::ConfirmationModal;

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
    pub current_page: usize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
            current_page: 0,
        }
    }
}
