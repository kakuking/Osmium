pub trait GuiT {
    fn want_pointer_input(&self) -> bool { false }
    fn wants_keyboard_input(&self) -> bool { false }
}