mod network;

use crate::network::net::connect;
use clap::{Arg, Command};

fn main() {

    let matches = Command::new("Daves App")
        .version("1.0")
        .author("David McMahon")
        .about("Does amazing things")
        .arg(
            Arg::new("connect")
            .short('c')
            .long("connect")
            .value_name("Connect to a Server")
            .help("Option to connect to a server with a port number")
            .required(true)
        )
        .arg(
            Arg::new("port_number")
            .short('p')
            .long("port number to bind to")
            .value_name("Port Number for Server")
            .help("A port number like 8080 or 3535")
            .required(true)
        )
        .get_matches();

    let server = matches.get_one::<String>("connect").unwrap().to_string();
    let port = matches.get_one::<String>("port_number").unwrap().to_string();
    
    let connection = connect(server, port);
    println!("{}", connection);
}

