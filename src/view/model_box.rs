use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::StatefulWidget};

enum WidgetFocus {
    FieldName,
    FieldType,
    UniqueCheckbox,
    NullableCheckbox,
    ForeignKeySelector,
}

struct ModelEditorState {
    focused_field: usize,
    focused_widget: WidgetFocus
}

pub struct ModelBox {
    name: String
}

impl StatefulWidget for ModelBox {
    type State = ModelEditorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ModelEditorState) {
        let [header, body] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).margin(1).areas(area);
    }
}