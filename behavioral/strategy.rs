trait FlyBehavior {
    fn fly(&self) {
        println!("i can fly!~~");
    }
}

struct CanFly;

impl FlyBehavior for CanFly {}

struct CanNotFly;
impl FlyBehavior for CanNotFly {
    fn fly(&self) {
        println!("i can't fly!");
    }
}

struct Duck {
    fly_behaviour: Box<dyn FlyBehavior>,
}

impl Duck {
    fn fly(&self) {
        self.fly_behaviour.fly();
    }

    fn new(fly_behaviour: Box<dyn FlyBehavior>) -> Duck {
        Duck { fly_behaviour }
    }

    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehavior>) {
        self.fly_behaviour = fly_behaviour;
    }
}

fn main() {
    let mut model_duck = Duck::new(Box::new(CanFly));
    model_duck.fly();
    model_duck.set_fly_behaviour(Box::new(CanNotFly));
    model_duck.fly();

    let mallard_duck = Duck::new(Box::new(CanFly));
    mallard_duck.fly();
}
