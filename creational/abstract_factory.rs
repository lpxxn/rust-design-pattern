//! Abstract Factory is a creational design pattern that lets you produce families of related objects without specifying their concrete classes.

trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

struct WinFactory;
impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton {})
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox {})
    }
}

struct MacFactory;
impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton {})
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox {})
    }
}

trait Button {
    fn paint(&self);
}

struct WinButton;
impl Button for WinButton {
    fn paint(&self) {
        println!("windows os button");
    }
}

struct MacButton;
impl Button for MacButton {
    fn paint(&self) {
        println!("mac os button");
    }
}

trait Checkbox {
    fn paint(&self);
}

struct WinCheckbox;
impl Checkbox for WinCheckbox {
    fn paint(&self) {
        println!("windows os checkbox");
    }
}

struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn paint(&self) {
        println!("mac os checkbox");
    }
}

struct Application;
impl Application {
    fn new_gui_factory(os: &str) -> Box<dyn GUIFactory> {
        match os {
            "mac" => Box::new(MacFactory {}),
            "win" => Box::new(WinFactory {}),
            _ => panic!("error"),
        }
    }
}

fn main() {
    let mac_app = Application::new_gui_factory("mac");
    let btn = mac_app.create_button();
    btn.paint(); // output: mac os button
    let cb = mac_app.create_checkbox();
    cb.paint(); // output: mac os checkbox

    let win_app = Application::new_gui_factory("win");
    let btn = win_app.create_button();
    btn.paint(); // output: windows os button
    let cb = win_app.create_checkbox();
    cb.paint(); // output: windows os checkbox
}
