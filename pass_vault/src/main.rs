mod pw_entry;
use crate::pw_entry::prompt;
use crate::pw_entry::read_passwords_from_file;
use crate::pw_entry::ServiceInfo;

fn clr() {
    println!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
     _____         _____ _______      __    _    _ _   _______
    |  __ \ /\    / ____/ ____\ \    / /\  | |  | | | |__   __|
    | |__) /  \  | (___| (___  \ \  / /  \ | |  | | |    | |
    |  ___/ /\ \  \___ \\___ \  \ \/ / /\ \| |  | | |    | |
    | |  / ____ \ ____) |___) |  \  / ____ \ |__| | |____| |
    |_| /_/    \_\_____/_____/    \/_/    \_\____/|______|_|
    "#;

    println!("{}", ascii);
    loop {
        println!("Password Manager Menu:");
        println!("1. Add Password");
        println!("2. List Passwords");
        println!("3. Search Passwords");
        println!("4. Quit Passvault");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords from file: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {},
                         - Username = {}
                         - Password = {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords from file: {}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {},
                             - Username = {}
                             - Password = {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
        println!("\n\n");
    }
}
