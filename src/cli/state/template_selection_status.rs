use ratatui::widgets::ListState;

pub struct TemplateSelectionStatus {
    pub list_state: ListState,
    pub selected_template: Option<usize>,
}

impl TemplateSelectionStatus {
    pub fn new() -> TemplateSelectionStatus {
        let mut status: TemplateSelectionStatus = TemplateSelectionStatus {
            list_state: ListState::default(),
            selected_template: None,
        };

        status.list_state.select_first();

        status
    }
}
