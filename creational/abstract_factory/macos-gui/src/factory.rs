use crate::button::MacButton;
use crate::checkbox::MacCheckBox;
use gui::button::Button;
use gui::checkbox::Checkbox;
use gui::factories::{GuiFactory, GuiFactoryDynamic};

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

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        todo!()
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        todo!()
    }
}
