use crate::store::get_in_store;
use crate::store::set_in_store;

pub(crate) fn ping() -> Vec<u8> {
    b"+PONG\r\n".to_vec()
}

pub fn echo(command_part: Vec<&str> ) -> Vec<u8> {
    let text = command_part[1..command_part.len()].join(" ");
    format!("${}\r\n{}\r\n",text.len(), text).as_bytes().to_vec()
}

pub fn get(command_part: Vec<&str>) -> Vec<u8> {
    format!("+{}\r\n", get_in_store(command_part[1].to_string())).as_bytes().to_vec()
}

pub async fn set(command_part : Vec<&str>) -> Vec<u8> {
    if command_part.len() < 2 || command_part[1] == "" {
        b"-Error while adding to store\r\n".to_vec()
    } else {
        if command_part.len() >= 5 && command_part[3] == "PX" && command_part[4].parse::<i64>().is_ok() {
            set_in_store(command_part[1].to_string(), command_part[2].to_string(), command_part[4].parse().unwrap()).await;
        } else {
            set_in_store(command_part[1].to_string(), command_part[2].to_string(), -1).await;
        }
        b"+OK\r\n".to_vec()
    }
}