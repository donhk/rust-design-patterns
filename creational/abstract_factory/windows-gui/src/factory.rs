use crate::button::WinButton;
use crate::checkbox::WinCheckBox;
use gui::button::Button;
use gui::checkbox::Checkbox;
use gui::factories::{GuiFactory, GuiFactoryDynamic};

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

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        todo!()
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        todo!()
    }
}
