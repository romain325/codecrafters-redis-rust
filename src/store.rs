use std::time::{Duration};
use std::sync::Mutex;
use std::collections::HashMap;
lazy_static! {
    #[derive(Debug)]
    static ref STORE: Mutex<HashMap<String,String>> = Mutex::new(HashMap::new());
}

async fn remove_from_store(key: String, expiry: u64) {
    println!("will wait {}", expiry); 
    std::thread::sleep(Duration::from_millis(expiry));
    println!("Remove {} after {}", key, expiry);
    STORE.lock().unwrap().remove(&key);
}

pub async fn set_in_store(key: String, value: String, expiry: i64) {
    STORE.lock().unwrap().insert(key.clone(), value);
    if expiry > 0 {
        println!("start thread {}", expiry);
        tokio::spawn(remove_from_store(key.clone(), expiry as u64));
    }
}

pub fn get_in_store(key: String) -> String {
    match STORE.lock().unwrap().get(&key) {
        Some(val) => val.clone(),
        None => "".to_string()
    }
}
