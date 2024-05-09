pub trait WithList {
    fn get_list_items_len(&self) -> usize;
    fn get_list_state_selected(&self) -> Option<usize>;
    fn set_selected(&mut self, idx: Option<usize>);
    fn unselect(&mut self) {
        self.set_selected(None);
    }
    fn select_next(&mut self) {
        let list_state_selected = self.get_list_state_selected();
        let list_len = self.get_list_items_len();
        let idx = match list_state_selected {
            Some(selected_idx) => {
                if selected_idx == list_len - 1 {
                    0
                } else {
                    selected_idx + 1
                }
            }
            None => 0,
        };
        self.set_selected(Some(idx))
    }
    fn select_previous(&mut self) {
        let list_state_selected = self.get_list_state_selected();
        let list_len = self.get_list_items_len();
        let idx = match list_state_selected {
            Some(selected_idx) => {
                if selected_idx == 0 {
                    list_len - 1
                } else {
                    selected_idx - 1
                }
            }
            None => list_len - 1,
        };
        self.set_selected(Some(idx))
    }
}
