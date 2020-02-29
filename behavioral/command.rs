//! Each action is encapsulated into a struct with the trait Command

use std::collections::HashMap;

trait Command {
    fn execute(&self);
}

#[derive(Copy, Clone)]
struct TV;
impl TV {
    fn new() -> TV {
        TV
    }
    fn on(&self) {
        println!("TV is on, watch movies.");
    }
    fn off(&self) {
        println!("TV is off");
    }
}

struct TVOnCommand {
    tv: TV,
}

impl TVOnCommand {
    fn new(tv: TV) -> TVOnCommand {
        TVOnCommand { tv }
    }
}

impl Command for TVOnCommand {
    fn execute(&self) {
        self.tv.on();
    }
}

struct TVOffCommand {
    tv: TV,
}

impl TVOffCommand {
    fn new(tv: TV) -> TVOffCommand {
        TVOffCommand { tv }
    }
}

impl Command for TVOffCommand {
    fn execute(&self) {
        self.tv.off();
    }
}

struct TVRemoteControl {
    commands: HashMap<i32, Box<dyn Command>>,
}

impl TVRemoteControl {
    fn new() -> TVRemoteControl {
        TVRemoteControl {
            commands: HashMap::new(),
        }
    }
    fn set_command(&mut self, idx: i32, cmd: Box<dyn Command>) {
        self.commands.insert(idx, cmd);
    }
    fn press_button(&self, idx: i32) {
        if let Some(cmd) = self.commands.get(&idx) {
            cmd.execute();
        } else {
            println!("do nothing.");
        }
    }
}

fn main() {
    let tv = TV::new();
    let mut remote_control = TVRemoteControl::new();
    remote_control.press_button(0);

    remote_control.set_command(1, Box::new(TVOnCommand::new(tv)));
    remote_control.set_command(2, Box::new(TVOffCommand::new(tv)));

    remote_control.press_button(1);
    remote_control.press_button(2);
}
