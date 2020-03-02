//! Proxy is a structural design pattern that lets you provide a substitute or placeholder for another object.
//! A proxy controls access to the original object, allowing you to perform something either before or after the request gets through to the original object.

// The Subject trait declares common operations for both RealSubject and
// the Proxy. As long as the client works with RealSubject using this
// interface, you'll be able to pass it a proxy instead of a real subject.
trait Subject {
    fn request(&self);
}

// The RealSubject contains some core business logic. Usually, RealSubjects
// are capable of doing some useful work which may also be very slow or
// sensitive - e.g. correcting input data. A Proxy can solve these issues
// without any changes to the RealSubject's code.
struct RealSubject {}
impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: handling request.");
    }
}

struct Proxy<'a> {
    real_subject: &'a RealSubject,
}
impl<'a> Proxy<'a> {
    fn new(real_subject: &'a RealSubject) -> Proxy {
        Proxy { real_subject }
    }
    fn check_access(&self) -> bool {
        // Some real checks should go here.
        println!("Proxy: checking access prior to firing a real request.");
        true
    }
    fn log_access(&self) {
        println!("Proxy: logging the request.");
    }
}

impl<'a> Subject for Proxy<'a> {
    // The most common applications of the Proxy pattern are lazy loading,
    // caching, controlling the access, logging, etc. A Proxy can perform one of
    // these things and then, depending on the result, pass the execution to the
    // same method in a linked RealSubject object.
    fn request(&self) {
        if self.check_access() {
            self.real_subject.request();
            self.log_access();
        }
    }
}

struct Client;
impl Client {
    // The client code is supposed to work with all objects (both subjects
    // and proxies) via the Subject interface in order to support both real
    // subjects and proxies. In real life, however, clients mostly work with
    // their real subjects directly. In this case, to implement the pattern
    // more easily, you can extend your proxy from the real subject's class.
    fn client_code<T: Subject>(subject: &T) {
        subject.request();
    }
}

fn main() {
    let real_subject = RealSubject {};
    println!("client: executing the client code with a real subject:");
    Client::client_code(&real_subject);

    println!("");
    println!("client: executing the same client code with a proxy:");
    let proxy = Proxy::new(&real_subject);
    Client::client_code(&proxy);
}
