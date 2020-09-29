
fn print_message() {
    println!("I am running in a different task!"); 
}

fn main() {
    spawn(print_message);
}
