use std::mem::MaybeUninit;
use std::sync::Once;

#[derive(Debug)]
struct Config {
    db_connection_str: String,
}

fn get_config() -> &'static mut Config {
    static mut CONF: MaybeUninit<Config> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Config {
            db_connection_str: "test config".to_string(),
        });
    });

    unsafe { &mut *CONF.as_mut_ptr() }
}

fn main() {
    let mut f1 = get_config();
    println!("{:?}", f1);
    // modify
    {
        let mut conf = &mut f1;
        conf.db_connection_str = "hello".to_string();
    }

    let f2 = get_config();
    println!("{:?}", f2);
    let conf2 = &f2;

    assert_eq!(conf2.db_connection_str, "hello".to_string())
}
