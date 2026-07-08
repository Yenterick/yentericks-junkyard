use crate::cli::{
    components::confirmation_modal::ConfirmationModal, state::sidebar_state::SidebarState,
};

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
    pub sidebar_state: SidebarState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
            sidebar_state: SidebarState::new(),
        }
    }
}
