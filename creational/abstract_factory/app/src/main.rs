mod render;

use render::render;

use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;

//Here, the abstract factory is implemented via generics which lets the
//compiler create a code that does NOT require dynamic dispatch in runtime.
fn main() {
    let windows = true;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}
