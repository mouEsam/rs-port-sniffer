use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{stdout, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Sender};

#[derive(Debug)]
pub enum SnifferError {
    NotEnoughArgs,
    InvalidArgs(String),
    MissingArg(String),
    Help,
}
impl Error for SnifferError {}
impl Display for SnifferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            SnifferError::NotEnoughArgs => f.write_str("Not enough arguments"),
            SnifferError::InvalidArgs(arg) => {
                f.write_str(format!("Invalid argument: {}", arg).as_str())
            }
            SnifferError::MissingArg(arg) => {
                f.write_str(format!("Missing argument for: {}", arg).as_str())
            }
            SnifferError::Help => f.write_str("help"),
        };
    }
}

pub struct Arguments {
    num_threads: i32,
    ip_addr: IpAddr,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, SnifferError> {
        if args.is_empty() {
            return Err(SnifferError::NotEnoughArgs);
        }
        let args = Vec::from(args);
        let mut num_threads: i32 = 4;
        let mut ipaddr = Option::<IpAddr>::None;
        let mut iterator = args.iter();

        loop {
            let first_arg = iterator.next();
            if let Some(first_arg) = first_arg {
                match first_arg.as_str() {
                    "--help" | "-h" => {
                        println!("Help");
                        return Err(SnifferError::Help);
                    }
                    "-j" => {
                        if let Some(threads) = iterator.next() {
                            if let Ok(threads) = threads.parse::<i32>() {
                                num_threads = threads;
                            } else {
                                return Err(SnifferError::InvalidArgs(threads.clone()));
                            }
                        } else {
                            return Err(SnifferError::MissingArg(first_arg.clone()));
                        }
                    }
                    other => {
                        if let Some(_) = ipaddr {
                            return Err(SnifferError::InvalidArgs(other.to_owned()));
                        } else if let Ok(ip) = IpAddr::from_str(other) {
                            ipaddr = Some(ip);
                        } else {
                            return Err(SnifferError::InvalidArgs(other.to_owned()));
                        }
                    }
                }
            } else {
                break;
            }
        }

        if let Some(ipaddr) = ipaddr {
            Ok(Arguments {
                num_threads,
                ip_addr: ipaddr,
            })
        } else {
            return Err(SnifferError::MissingArg(String::from("ip_addr")));
        }
    }

    pub fn num_threads(&self) -> i32 {
        self.num_threads
    }

    pub fn ip_addr(&self) -> IpAddr {
        self.ip_addr
    }
}

const MAX_PORTS: i32 = 65535;

pub fn scan(start: i32, increment: i32, sender: Sender<i32>, addr: IpAddr) {
    let mut port = start;
    loop {
        let socket = TcpStream::connect(SocketAddr::new(addr, port as u16));
        if let Ok(_) = socket {
            print!(".");
            stdout().flush().unwrap();
            sender.send(port).unwrap();
        }

        if MAX_PORTS - port <= increment {
            break;
        }
        port += increment;
    }
}