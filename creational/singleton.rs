use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

#[derive(Debug)]
struct Config {
    db_connection_str: String,
}

fn get_config() -> &'static Mutex<Config> {
    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Config {
            db_connection_str: "test config".to_string(),
        }));
    });

    unsafe { &*CONF.as_ptr() }
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
