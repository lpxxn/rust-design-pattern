trait FlyBehaviour {
    fn fly(&self);
}

struct FlyWithWings;

impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("i can fly!")
    }
}

struct FlyNoWay;

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("i can't fly!~~")
    }
}

trait Duck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour;
    fn fly(&self) {
        let s = self.get_fly_behaviour();
        s.fly();
    }
}

struct MallardDuck {
    fly_behaviour: Box<dyn FlyBehaviour>,
}

impl Duck for MallardDuck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl MallardDuck {
    fn new(fly_behaviour: Box<dyn FlyBehaviour>) -> Self {
        MallardDuck { fly_behaviour }
    }
    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fly_behaviour;
    }
}

struct ModelDuck {
    fly_behaviour: Box<FlyNoWay>,
}

impl Duck for ModelDuck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl ModelDuck {
    fn new(fly_behaviour: Box<FlyNoWay>) -> Self {
        ModelDuck { fly_behaviour }
    }
}

pub fn main() {
    let mut mallard_duck = MallardDuck::new(Box::new(FlyWithWings));
    mallard_duck.fly();
    mallard_duck.set_fly_behaviour(Box::new(FlyNoWay));
    mallard_duck.fly();

    let model_duck = ModelDuck::new(Box::new(FlyNoWay));
    model_duck.fly();
}
