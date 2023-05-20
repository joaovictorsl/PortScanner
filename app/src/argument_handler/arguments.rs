use std::{net::IpAddr, str::FromStr};

use super::error::ArgumentError;

const HELP_FLAG: [&str; 2] = ["-h", "--help"];
const THREADS_FLAG: &str = "-t";

#[derive(Debug)]
pub enum Arguments {
    Execution(ExecutionArguments),
    Help(&'static str),
}

#[derive(Debug)]
pub struct ExecutionArguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl ExecutionArguments {
    pub fn get_ipaddr(&self) -> &IpAddr {
        &self.ipaddr
    }

    pub fn get_threads(&self) -> u16 {
        self.threads
    }
}

impl Arguments {
    pub fn new(args: Vec<String>) -> Result<Arguments, ArgumentError> {
        // Removing program name argument
        let args = &args[1..];
        // Check if #arguments is correct
        if args.len() != 1 && args.len() != 3 {
            return Err(
                ArgumentError::InvalidNumberOfArguments(
                    "Invalid number of argumenst. There should be 1 (in ip only) or 3 (in ip + number of threads) arguments only."
                )
            );
        }
        // Deal with -h flag
        if (HELP_FLAG.contains(&args[0].as_str())) && args.len() == 1 {
            return Ok(Arguments::Help(
                "Usage:
                \r    -t to select how many threads you want
                \r    -h or --help to show this help message",
            ));
        }
        // Getting number of threads
        let threads = if args.len() == 1 {
            4
        } else {
            Arguments::get_num_of_threads(args)?
        };
        // Getting ip address
        let ipaddr = Arguments::get_ipaddr(&args)?;
        // Returning ExecutionArguments
        Ok(Arguments::Execution(ExecutionArguments { ipaddr, threads }))
    }

    fn get_ipaddr(args: &[String]) -> Result<IpAddr, ArgumentError> {
        let ipaddr_index = if args.len() == 1 { 0 } else { 2 };
        let res = IpAddr::from_str(&args[ipaddr_index]);

        match res {
            Ok(valid_ip) => return Ok(valid_ip),
            Err(_) => return Err(ArgumentError::InvalidIpAddr("Invalid IP address")),
        }
    }

    fn get_num_of_threads(args: &[String]) -> Result<u16, ArgumentError> {
        let flag = &args[0];
        // If it's not THREADS_FLAG, it's an invalid flag
        if flag != THREADS_FLAG {
            let invalid_flag_msg = format!("{} is not a valid flag.", flag);
            return Err(ArgumentError::InvalidFlag(invalid_flag_msg));
        }

        let threads_num = u16::from_str(&args[1]);

        match threads_num {
            Ok(valid_num) => return Ok(valid_num),
            Err(_) => {
                return Err(ArgumentError::InvalidNumberOfThreads(
                    "Invalid number of threads",
                ))
            }
        }
    }
}
