
pub(crate) fn ping() -> Vec<u8> {
    b"+PONG\r\n".to_vec()
}

pub fn echo(command_part: Vec<&str> ) -> Vec<u8> {
    let text = command_part[1..command_part.len()].join(" ");
    format!("${}\r\n{}\r\n",text.len(), text).as_bytes().to_vec()
}