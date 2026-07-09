use crate::cli::{
    components::confirmation_modal::ConfirmationModal,
    state::{
        models_state::ModelsState, sidebar_state::SidebarState,
        template_selection_state::TemplateSelectionState,
    },
};

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
    pub sidebar_state: SidebarState,
    pub template_selection_state: TemplateSelectionState,
    pub models_state: ModelsState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
            sidebar_state: SidebarState::new(),
            template_selection_state: TemplateSelectionState::new(),
            models_state: ModelsState::new(),
        }
    }
}
