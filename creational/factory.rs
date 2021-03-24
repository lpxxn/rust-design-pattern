//! Factory method creational design pattern allows creating objects without having to specify the exact type of the object that will be created.

pub trait Knife {
    fn make_sellable(&mut self);
}

pub trait KnifeFactory {
    fn create_knife(&self) -> Box<dyn Knife>;
}

pub struct KnifeStore<'t> {
    factory: &'t Box<dyn KnifeFactory>
}

impl<'t> KnifeStore<'t> {
    pub fn order_knife(&self) -> Box<dyn Knife> {
        let mut knife = self.factory.create_knife();
        knife.make_sellable();
        knife
    }

    pub fn new(factory: &'t Box<dyn KnifeFactory>) -> Self {
        Self { factory }
    }
}

struct SteakKnife {
    is_sellable: bool
}

impl Knife for SteakKnife {
    fn make_sellable(&mut self) {
        println!("Make the steak knife sellable");
        self.is_sellable = true
    }
}

struct SteakFactory {}

impl KnifeFactory for SteakFactory {
    fn create_knife(&self) -> Box<dyn Knife> {
        println!("Make a steak knife");
        Box::from(SteakKnife { is_sellable: false })
    }
}

impl SteakFactory {
    pub fn new() -> Box<dyn KnifeFactory> {
        Box::from(SteakFactory {})
    }
}

#[test]
fn test() {
    let factory = SteakFactory::new();
    let store = KnifeStore::new(&factory);
    let _knife = store.order_knife();
}
