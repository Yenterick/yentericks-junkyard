use ratatui::widgets::ListState;

pub struct TemplateSelectionState {
    pub list_state: ListState,
    pub selected_template: Option<usize>,
}

impl TemplateSelectionState {
    pub fn new() -> TemplateSelectionState {
        let mut status: TemplateSelectionState = TemplateSelectionState {
            list_state: ListState::default(),
            selected_template: None,
        };

        status.list_state.select_first();
        status
    }
}
