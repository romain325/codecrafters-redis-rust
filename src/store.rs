use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref STORE: Mutex<HashMap<String,String>> = Mutex::new(HashMap::new());
}

pub fn set_in_store(key: String, value: String) {
    STORE.lock().unwrap().insert(key, value);
}

pub fn get_in_store(key: String) -> String {
    match STORE.lock().unwrap().get(&key) {
        Some(val) => val.clone(),
        None => "".to_string()
    }
}