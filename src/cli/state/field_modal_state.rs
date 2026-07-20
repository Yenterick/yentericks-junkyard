use ratatui::widgets::ListState;

pub struct FieldModalState {
    pub input_buffer: String,
    pub list_state: ListState,
    pub unique: bool,
    pub allow_null: bool,
}

impl FieldModalState {
    pub fn new() -> Self {
        let mut state: FieldModalState = FieldModalState {
            input_buffer: String::new(),
            list_state: ListState::default(),
            unique: false,
            allow_null: false,
        };

        state.list_state.select(Some(0));
        state
    }
}
