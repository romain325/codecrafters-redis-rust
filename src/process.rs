use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use std::io::{Error, ErrorKind};
use async_recursion::async_recursion;

use crate::command::ping;
use crate::command::echo;

pub async fn process_session(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let empty_string = match String::from_utf8(b"\r\n".to_vec()) {
        Ok(val) => val,
        Err(err) => return Err(Error::new(ErrorKind::InvalidData, err))
    };

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).await?;
        if input == empty_string {
            continue;
        }

        let command = parse_str(input, &mut reader).await?;
        let output  = exec_command(command)?;
        reader.write_all(output.as_bytes()).await?;
    }
}

#[async_recursion]
async fn parse_str(val: String, reader: &mut BufReader<&mut TcpStream>) -> std::io::Result<String> {
    let new_line_offset = match val.as_bytes().iter().zip(val.as_bytes().iter().skip(1)).position(|(cr,lf)| *cr == b'\r' && *lf == b'\n') {
        Some(val) => val,
        None => 0 
    };

    if new_line_offset == 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    return match val.chars().nth(0) {
        Some(first) => {
            let command = (&val[1..new_line_offset]).to_string();
            match first {
                '+' => Ok(command),
                '-' => Ok(command),
                '*' => {
                    let nb_elem = get_decimal(command)?;
                    let mut res = Vec::new();
                    for _i in 0..nb_elem {
                        let mut content = String::new();
                        reader.read_line(&mut content).await?;
                        match content.chars().nth(0) {
                            Some(val) => {
                                if val == '*' {
                                    return Err(Error::new(ErrorKind::PermissionDenied, "Recursive array not supported"));
                                }
                            },
                            None => return Err(Error::new(ErrorKind::InvalidInput, "Empty data")),
                        }

                        res.push(parse_str(content, reader).await?);
                    }
                    Ok(res.join(" "))
                },
                '$' => {
                    let mut content = String::new();
                    reader.read_line(&mut content).await?; 
                    get_bulk(command, content)
                }
                ':' => {
                    match get_decimal(command) {
                        Ok(dec) => Ok(dec.to_string()),
                        Err(err) => Err(err),
                    }
                },
                _default => Err(Error::from(ErrorKind::InvalidData))
            }
        },
        None => Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn get_decimal(command: String) -> std::io::Result<u32> {
    match command.parse::<u32>() {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::new(ErrorKind::Other, err))
    }
}

fn get_bulk(command: String, rest: String) -> std::io::Result<String> {
    let size = get_decimal(command)?;
    let value = (&rest[..rest.len()-2]).to_string();

    Ok(value.chars().into_iter().take(size as usize).collect())
}

fn exec_command(command: String) -> std::io::Result<String> {
    let command_part : Vec<&str> = command.split(" ").collect();
    println!("command: {}", command);
    let result = match command_part[0] {
        "ping" => ping(),
        "PING" => ping(),
        "QUIT" => return Err(Error::new(ErrorKind::ConnectionAborted, "Goodbye")),
        "ECHO" => echo(command_part),
        "echo" => echo(command_part),
        _default => ping()
    };
    Ok(String::from_utf8(result).unwrap())
}
