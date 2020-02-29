//! Builder is a creational design pattern, which allows constructing complex objects step by step.

#[derive(Clone)]
struct Product {
    parts: Vec<String>,
}

impl Product {
    fn new() -> Product {
        Product { parts: Vec::new() }
    }
    fn list_parts(&self) {
        let parts_list = String::from(" parts ");
        println!("{0}{1}{0}", "*".repeat(10), parts_list);
        for v in &self.parts {
            println!("{}", v);
        }
        println!("{0}{1}{0}", "*".repeat(10), "*".repeat(parts_list.len()));
    }
}

/**
 * The Builder trait specifies methods for creating the different parts of
 * the Product objects.
 */
trait Builder {
    fn produce_part_a(&mut self);
    fn produce_part_b(&mut self);
    fn produce_part_c(&mut self);
    fn get_product(&mut self) -> Product;
}

/**
 * The Concrete Builder classes follow the Builder interface and provide
 * specific implementations of the building steps. Your program may have several
 * variations of Builders, implemented differently.
 */
struct ContreteBuilder1 {
    product: Product,
}

impl ContreteBuilder1 {
    fn new() -> ContreteBuilder1 {
        ContreteBuilder1 {
            product: Product::new(),
        }
    }
}

impl Builder for ContreteBuilder1 {
    fn produce_part_a(&mut self) {
        self.product.parts.push("part a1".to_string());
    }
    fn produce_part_b(&mut self) {
        self.product.parts.push("part b1".to_string());
    }
    fn produce_part_c(&mut self) {
        self.product.parts.push("part c1".to_string());
    }
    fn get_product(&mut self) -> Product {
        let p = self.product.clone();
        self.product = Product::new();
        p
    }
}

struct ContreteBuilder2 {
    product: Product,
}

impl ContreteBuilder2 {
    fn new() -> ContreteBuilder2 {
        ContreteBuilder2 {
            product: Product::new(),
        }
    }
}

impl Builder for ContreteBuilder2 {
    fn produce_part_a(&mut self) {
        self.product.parts.push("part a ~~~~ 2".to_string());
    }
    fn produce_part_b(&mut self) {
        self.product.parts.push("part b ~~~~ 2".to_string());
    }
    fn produce_part_c(&mut self) {
        self.product.parts.push("part c ~~~~ 2".to_string());
    }
    fn get_product(&mut self) -> Product {
        let p = Product {
            parts: self.product.parts.clone(),
            ..self.product
        };
        self.product = Product::new();
        p
    }
}

/**
 * The Director is only responsible for executing the building steps in a
 * particular sequence. It is helpful when producing products according to a
 * specific order or configuration. Strictly speaking, the Director class is
 * optional, since the client can control builders directly.
 */
struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(builder: Box<Builder>) -> Director {
        Director { builder: builder }
    }

    fn construct(&mut self) {
        self.builder.produce_part_a();
        self.builder.produce_part_b();
        self.builder.produce_part_c();
    }
}

fn main() {
    let builder1 = Box::new(ContreteBuilder1::new());
    let mut direct = Director::new(builder1);
    direct.construct();
    let product = direct.builder.get_product();
    product.list_parts();
    // output:
    /*
    ********** parts **********
    part a1
    part b1
    part c1
    ***************************
    */

    let build2 = Box::new(ContreteBuilder2::new());
    let mut direct = Director::new(build2);
    direct.construct();
    let product = direct.builder.get_product();
    product.list_parts();
    // output:
    /*
    ********** parts **********
    part a ~~~~ 2
    part b ~~~~ 2
    part c ~~~~ 2
    ***************************
    */
}
