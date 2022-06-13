extern crate term;
extern crate whoami;
use ansi_term::Colour::Red;
fn main() {
    // Welcome promt
    println!("Welcome to {}", Red.paint("JonteCLI"));
    println!("---");
    println!("The availible commands are:");
    println!("help - displays this help");
    println!("blog - displays articles from the blog");
    println!("exit - exits the program");
    println!("---");
    println!("Please enter a command:");
    // Running the core logic of the program
    command_handler();
}
fn command_handler() {
    // Experimentally using a variable for keeping track of current path, nessary for the cd command.
    // let mut path = String::new();
    // path += "/home/";
    // path += &whoami::username();
    // path += "/";

    // Dependencies
    use std::io;
    use std::io::Write;
    let mut line = String::new();
    // Print the prompt
    print!("[{}@jontes.page ~]$ ", whoami::username());
    io::stdout().flush().unwrap();
    // Read standard input
    std::io::stdin().read_line(&mut line).unwrap();
    // Handle the command
    if line.starts_with("cat") {
        for s in line.split(" ").skip(1) {
            if s == "credits.txt\n" {
                println!("Welcome to this shitty \"shell simulation program\". Mainly coded by Jonte. Contributers: None.");
            } else {
                println!("cat: {}: No such file or directory", s.trim());
            }
            command_handler();
        }
    }
    match line.trim() {
        "help" => help(),
        "exit" => println!("{}, See ya!", Red.paint("Goodbye")),
        "blog" => blog(),
        "ls" => ls(),
        "" => command_handler(),
        "cat" => {
            println!("Please enter a file name");
            command_handler();
        }
        "clear" => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            command_handler()
        }
        "cat credits.txt" => {
            command_handler();
        }
        "pwd" => {
            println!("/home/{}/", whoami::username());
            command_handler()
        }
        _ => {
            println!("jonteshell: {}: command not found...", line.trim());
            command_handler()
        }
    }
}
fn help() {
    println!("---");
    println!("Welcome to Help - The availible commands are:");
    println!("help - displays this help");
    println!("blog - displays articles from the blog");
    println!("exit - exits the program");
    println!("ls - lists files in the current directory");
    println!("cat - view contents of a file");
    println!("---");
    command_handler();
}
fn blog() {
    println!("This is {}, in CLI", Red.paint("Jonte's Blog"));
    println!("---");
    println!("This is a list of articles:");
}
fn ls() {
    // Add path functionality later
    println!("credits.txt");
    command_handler();
}
