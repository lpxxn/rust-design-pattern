/*
Factory method creational design pattern allows creating objects without having to specify the exact type of the object that will be created.
*/

trait Shap {
    fn draw(&self);
}

enum ShapType {
    Rectangle,
    Circl,
}

struct Rectangle {}

impl Shap for Rectangle {
    fn draw(&self) {
        println!("draw a rectangle!");
    }
}

struct Circl {}

impl Shap for Circl {
    fn draw(&self) {
        println!("draw a circl!");
    }
}

struct ShapFactory;
impl ShapFactory {
    fn new_shap(s: &ShapType) -> Box<dyn Shap> {
        match s {
            ShapType::Circl => Box::new(Circl {}),
            ShapType::Rectangle => Box::new(Rectangle {}),
        }
    }
}

fn main() {
    let shap = ShapFactory::new_shap(&ShapType::Circl);
    shap.draw(); // output: draw a circl!

    let shap = ShapFactory::new_shap(&ShapType::Rectangle);
    shap.draw(); // output: draw a rectangle!
}
