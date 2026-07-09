use ratatui::widgets::ListState;

pub struct ModelsState {
    pub list_state: ListState,
    pub selected_model: Option<usize>,
}

impl ModelsState {
    pub fn new() -> ModelsState {
        let mut status: ModelsState = ModelsState {
            list_state: ListState::default(),
            selected_model: None,
        };

        status.list_state.select_first();
        status
    }
}
