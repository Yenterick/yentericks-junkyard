use crate::cli::{
    components::confirmation_modal::ConfirmationModal,
    state::{sidebar_state::SidebarState, template_selection_status::TemplateSelectionStatus},
};

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
    pub sidebar_state: SidebarState,
    pub template_selection_state: TemplateSelectionStatus,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
            sidebar_state: SidebarState::new(),
            template_selection_state: TemplateSelectionStatus::new(),
        }
    }
}
