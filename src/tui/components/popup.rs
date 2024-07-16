pub trait WithPopup {
    fn set_popup_state(&mut self, open: bool);
    fn get_popup_state(&self) -> bool;
    fn open_popup(&mut self) {
        self.set_popup_state(true)
    }

    fn close_popup(&mut self) {
        self.set_popup_state(false)
    }

    fn is_popup_open(&self) -> bool {
        self.get_popup_state()
    }
}
