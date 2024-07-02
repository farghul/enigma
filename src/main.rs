use std::{
    env,
    process::Command,
};
use colored::Colorize;

const HALT: &'static str = " Program halted ";
const PLACE: &'static str = "/Users/byron/Documents/workbench/rust/enigma/";


fn main() {
    let supplied: Vec<String> = env::args().collect();
    if supplied.len() == 1 {
        alert(" No arguments provided, ");
    } else if &supplied[1] == "help" || &supplied[1] == "h" {
        about();
    } else if supplied.len() == 4 {
        let flag: &str= &supplied[1];
        let name = &supplied[2];
        let date = &supplied[3];
        match flag {
        "k" | "key" => genkey(name),
        "n" | "nokey" => gencsr(name, date),
        _ => alert(" Bad flag detected -"),
        }
    } else { 
        warn(" Agrument(s) not recognized -");
    }
}


// Generate a new encryption key
fn genkey(name: &String) {
    message("Generating a Key");
    execute(String::from("openssl"), &["genrsa", "-out", &(name.to_owned() + ".key"), "4096"].to_vec());
}


// Create and verify a Certificate Signing Request
fn gencsr(name: &String, date: &String) {
    message("Creating a CSR");
    execute(String::from("openssl"), &["req", "-new", "-key", &(PLACE.to_owned() + &name + ".key"), "-nodes", "-sha256", "-out", &(PLACE.to_owned() + &name + ".csr"), "-config", &(name.to_owned() + ".cnf")].to_vec());
    message("Verifying the CSR");
    execute(String::from("openssl"), &["req", "-text", "-noout", "-verify", "-in", &(PLACE.to_owned() + &name + "_" + &date + ".csr")].to_vec());
}


// Print an informational message
fn message(content: &str) {
    println!("\n{} {} {}", "**".yellow(), content, "**".yellow());
}


// Print a colourized warning message
fn warn(content: &str) {
    println!("\n{}{}", content.on_yellow(), HALT.on_yellow());
}


// Print a colourized error message
fn alert(content: &str) {
    println!("\n{}{}", content.on_bright_red(), HALT.on_bright_red());
}


// Run standard terminal commands and display the output
fn execute(task: String, additions: &Vec<&str>) {
	let iterator: std::slice::Iter<'_, &str> = additions.iter();
	Command::new(&task)
		.args(iterator)
        .spawn()
        .expect("Command failed to start");
}


// Print help information for using the program
fn about() {
	println!("\n{}", "Usage:".yellow());
	println!("  [program] [flag] [cert name] [mmddyyyy]");
	println!("{}", "\nOptions:".yellow());
	println!("{}      Create CSR with new key", " k,  key".green());
	println!("{}    Create CSR with existing key", " n,  nokey".green());
	println!("{}     Help Information", " h,  help".green());
	println!("{}", "\nExample:".yellow());
	println!("  Pointing at your target/release folder, run:");
	println!("{}", "    enigma k prod_domains 11282023".green());
	println!("{}", "\nHelp:".yellow());
	println!("  For more information go to:");
	println!("{}", "    https://github.com/farghul/enigma.git".green());
    println!();
}