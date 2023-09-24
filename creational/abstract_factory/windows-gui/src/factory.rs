use crate::button::WinButton;
use crate::checkbox::WinCheckBox;
use gui::factories::gui_factory::GuiFactory;

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WinButton;
    type C = WinCheckBox;

    fn create_button(&self) -> Self::B {
        todo!()
    }

    fn create_checkbox(&self) -> Self::C {
        todo!()
    }
}
