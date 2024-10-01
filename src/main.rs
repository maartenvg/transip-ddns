mod find_current_ip;

fn main() {
    match find_current_ip::run() {
        Ok(result) => println!("IP address is {}", result),
        Err(e) => println!("Error occurred: {}", e),
    };
}
