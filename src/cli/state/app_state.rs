use crate::cli::{
    components::{confirmation_modal::ConfirmationModal, error_modal::ErrorModal},
    state::{
        models_state::ModelsState, sidebar_state::SidebarState,
        template_selection_state::TemplateSelectionState,
    },
};

pub struct AppState {
    pub confirmation_modal: Option<ConfirmationModal>,
    pub error_modal: Option<ErrorModal>,
    pub sidebar_state: SidebarState,
    pub template_selection_state: TemplateSelectionState,
    pub models_state: ModelsState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            confirmation_modal: None,
            error_modal: None,
            sidebar_state: SidebarState::new(),
            template_selection_state: TemplateSelectionState::new(),
            models_state: ModelsState::new(),
        }
    }
}
