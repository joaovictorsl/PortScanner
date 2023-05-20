mod argument_handler;

use argument_handler::{Arguments, ExecutionArguments};
use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
};
// Multithreaded
use std::sync::mpsc::{channel, Sender};
use std::thread;

// Highest TCP port number
const MAX_PORT: u16 = 65535;

fn main() {
    // Get args from command line
    let args: Vec<String> = env::args().collect();
    // Parse args
    let args = Arguments::new(args).unwrap_or_else(|err| panic!("{}", err));

    match args {
        Arguments::Help(msg) => {
            println!("{}", msg);
            process::exit(0);
        }
        Arguments::Execution(exec_args) => run(exec_args),
    }
}

fn run(args: ExecutionArguments) {
    let num_threads = args.get_threads();
    let (sender, receiver) = channel::<u16>();

    for i in 1..=num_threads {
        let sender_i = sender.clone();
        let ipaddr = args.get_ipaddr().clone();
        // Spawns a new thread
        thread::spawn(move || {
            scan(sender_i, i, ipaddr, num_threads);
        });
    }

    // Drops main's thread sender so that the receiver loop can end.
    drop(sender);
    // Vector for results
    let mut out = vec![];

    for port in receiver {
        out.push(port);
    }
    // Jumps a line after the dots
    println!();
    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}

fn scan(sender: Sender<u16>, mut start_port: u16, addr: IpAddr, num_threads: u16) {
    while MAX_PORT - start_port >= num_threads {
        // Gets addr:port string
        let target = addr.to_string() + ":" + &start_port.to_string();
        // Gets a handler for the standard output of the
        // current process so threads can print to it.
        let mut process_stdout = io::stdout();

        match TcpStream::connect(target) {
            Ok(_) => {
                // If connection to the TCP port worked it
                // means this port is open. (Since we'll disconnect)
                print!(".");
                // Flushes the buffer so the dots can be printed
                process_stdout.flush().unwrap();
                // Sends the open port to the main thread
                sender.send(start_port).unwrap();
            }
            Err(_) => {}
        }

        start_port += num_threads;
    }
}
