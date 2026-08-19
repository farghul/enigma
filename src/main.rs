use colored::Colorize;
use dirs::home_dir;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const HALT: &'static str = " Program halted ";

fn home() -> PathBuf {
    home_dir().expect("Could not find home directory")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        alert(" No arguments provided, ");
    } else if &args[1] == "--help" || &args[1] == "-h" {
        about();
    } else if args.len() == 4 {
        let flag: &str = &args[1];
        let name = &args[2];
        let date = &args[3];
        match flag {
            "-k" | "--key" => gen_key(name),
            "-n" | "--nokey" => gen_csr(name, date),
            _ => alert(" Bad flag detected -"),
        }
    } else {
        warn(" Argument(s) not recognized -");
    }
}

fn key_path(name: &str) -> PathBuf {
    home().join("Documents/gitea/enigma/key").join(format!("{}.key", name))
}

fn csr_path(name: &str, date: &str) -> PathBuf {
    home()
        .join("Documents/gitea/enigma/csr")
        .join(format!("{}_{}.csr", name, date))
}

fn ssl_dir() -> PathBuf {
    home().join("Documents/bitbucket/vm-nginx-conf/ssl")
}

fn cnf_path(name: &str) -> PathBuf {
    let ssl = ssl_dir();
    for entry in fs::read_dir(&ssl).expect("Could not read ssl directory") {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        if path.is_dir() {
            for sub in fs::read_dir(&path).expect("Could not read subdirectory") {
                let sub = sub.expect("Could not read entry");
                let sub_path = sub.path();
                if sub_path.extension().map_or(false, |e| e == "cnf") {
                    let stem = sub_path.file_stem().unwrap().to_string_lossy();
                    if stem.contains(name) {
                        return sub_path;
                    }
                }
            }
        }
    }
    panic!("No .cnf file found matching '{}'", name);
}

// Generate a new encryption key
fn gen_key(name: &String) {
    message("Generating a Key");
    execute(
        String::from("openssl"),
        &[
            "genrsa",
            "-out",
            key_path(name).to_str().unwrap(),
            "4096",
        ]
        .to_vec(),
    );
}

// Create and verify a Certificate Signing Request
fn gen_csr(name: &String, date: &String) {
    message("Creating a CSR");
    execute(
        String::from("openssl"),
        &[
            "req",
            "-new",
            "-key",
            key_path(name).to_str().unwrap(),
            "-nodes",
            "-sha256",
            "-out",
            csr_path(name, date).to_str().unwrap(),
            "-config",
            cnf_path(name).to_str().unwrap(),
        ]
        .to_vec(),
    );
    message("Verifying the CSR");
    execute(
        String::from("openssl"),
        &[
            "req",
            "-text",
            "-noout",
            "-verify",
            "-in",
            csr_path(name, date).to_str().unwrap(),
        ]
        .to_vec(),
    );
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
    let output = Command::new(&task)
        .args(additions)
        .output()
        .expect("Command failed to start");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{} {}", "Error:".red(), stderr.trim());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        print!("{}", stdout);
    }
}

// Print help information for using the program
fn about() {
    println!("\n{}", "Usage:".yellow());
    println!("  [program] [flag] [cert name] [mmddyyyy]");
    println!("{}", "\nOptions:".yellow());
    println!("{}      Create CSR with a new key", " -k,  --key".green());
    println!(
        "{}    Create CSR with an existing key",
        " -n,  --nokey".green()
    );
    println!("{}     Help Information", " -h,  --help".green());
    println!("{}", "\nExample:".yellow());
    println!("  Pointing at your target/release folder, run:");
    println!("{}", "    enigma -k prod_domains 11282023".green());
    println!("{}", "\nHelp:".yellow());
    println!("  For more information go to:");
    println!("{}", "    https://gitea.com:bstuike/enigma.git".green());
    println!();
}
