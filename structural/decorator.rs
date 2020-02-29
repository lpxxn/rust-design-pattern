use std::rc::Rc;

// The base Component trait defines operations that can be altered by
// decorators.
trait Component {
    fn operation(&self) -> String;
}

// Concrete Components provide default implementations of the operations.
// There might be several variations of these classes.
struct ConcreteComponent {}

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        "ConcreteComponent".to_string()
    }
}

// The base Decorator class follows the same interface as the other
// components. The primary purpose of this class is to define the wrapping
// interface for all concrete decorators. The default implementation of the
// wrapping code might include a field for storing a wrapped component and
// the means to initialize it.
trait Decorator: Component {
    fn new(component: Rc<dyn Component>) -> Self;
}

// Concrete Decorators call the wrapped object and alter its result in some
// way.
struct ConcreteDecoratorA {
    component: Rc<dyn Component>,
}

impl Decorator for ConcreteDecoratorA {
    fn new(component: Rc<dyn Component>) -> Self {
        ConcreteDecoratorA { component }
    }
}

impl Component for ConcreteDecoratorA {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorA: {}", self.component.operation())
    }
}

struct ConcreteDecoratorB {
    component: Rc<dyn Component>,
}

impl Decorator for ConcreteDecoratorB {
    fn new(component: Rc<dyn Component>) -> Self {
        ConcreteDecoratorB { component }
    }
}

impl Component for ConcreteDecoratorB {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorB: {}", self.component.operation())
    }
}

// The client code works with all objects using the Component interface.
// This way it can stay independent of the concrete classes of
// components it works with.
struct Client;

impl Client {
    fn client_code<T: Component>(component: &T) {
        println!("result: {}", component.operation())
    }
}

fn main() {
    let component = Rc::new(ConcreteComponent {});
    println!("client: i get a simple component: ");
    Client::client_code(component.as_ref());
    println!("client: now I've got a decorated component:");
    let decorator_a1 = ConcreteDecoratorA::new(component.clone());
    Client::client_code(&decorator_a1);

    let decorator_a2 = ConcreteDecoratorB::new(Rc::new(decorator_a1));
    Client::client_code(&decorator_a2);
}
