use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Config {
    db_connection_str: String,
}

fn get_config() -> Arc<Mutex<Config>> {
    static mut CONF: Option<Arc<Mutex<Config>>> = None;
    unsafe {
        CONF.get_or_insert_with(|| {
            println!("init"); // do once
            Arc::new(Mutex::new(Config {
                db_connection_str: "abcdef".to_string(),
            }))
        })
        .clone()
    }
}

fn main() {
    let f1 = get_config();
    println!("{:?}", f1);
    // modify 
    {
        let mut conf = f1.lock().unwrap();
        conf.db_connection_str = "hello".to_string();
    }

    let f2 = get_config();
    println!("{:?}", f2);
    let conf2 = f2.lock().unwrap();

    assert_eq!(conf2.db_connection_str, "hello".to_string())
}
