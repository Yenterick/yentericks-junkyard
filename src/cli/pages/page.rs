pub trait Page {
    type State;

    fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut Self::State,
    );

    fn handle_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
    ) -> ScreenAction;
}