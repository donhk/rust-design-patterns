use crate::button::MacButton;
use crate::checkbox::MacCheckBox;
use gui::factories::GuiFactory;

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckBox;

    fn create_button(&self) -> Self::B {
        todo!()
    }

    fn create_checkbox(&self) -> Self::C {
        todo!()
    }
}
