//! Observer is a behavioral design pattern that allows one objects to notify other objects about changes in their state.

trait IObserver {
    fn update(&self);
}

trait ISubject<'a, T: IObserver> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
    fn notify_observers(&self);
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}
impl<'a, T: IObserver + PartialEq> Subject<'a, T> {
    fn new() -> Subject<'a, T> {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Subject<'a, T> {
    fn attach(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }
    fn detach(&mut self, observer: &'a T) {
        if let Some(idx) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(idx);
        }
    }
    fn notify_observers(&self) {
        for item in self.observers.iter() {
            item.update();
        }
    }
}

#[derive(PartialEq)]
struct ConcreteObserver {
    id: i32,
}
impl IObserver for ConcreteObserver {
    fn update(&self) {
        println!("Observer id:{} received event!", self.id);
    }
}

fn main() {
    let mut subject = Subject::new();
    let observer_a = ConcreteObserver { id: 1 };
    let observer_b = ConcreteObserver { id: 2 };

    subject.attach(&observer_a);
    subject.attach(&observer_b);
    subject.notify_observers();

    subject.detach(&observer_b);
    subject.notify_observers();
}
