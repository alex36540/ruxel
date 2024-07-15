pub struct NewFileModal {
    pub width: usize,
    pub height: usize,
    pub show_modal: bool,
    pub show_modal_toggle: bool,
}

impl Default for NewFileModal {
    fn default() -> Self {
        NewFileModal {
            width: 32,
            height: 32,
            show_modal: false,
            show_modal_toggle: false,
        }
    }
}

impl NewFileModal {
    pub fn activate(&mut self) {
        self.show_modal = true;
        self.show_modal_toggle = true;
    }

    pub fn is_active(&self) -> bool {
        self.show_modal && self.show_modal_toggle
    }
}
