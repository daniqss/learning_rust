use std::io;

fn main () {
    let mut message = String::from("Wenas me llamo ");

    get_user(&mut message);
    println!("{}", message);
}

fn get_user (message: &mut String) {
    let mut user = String::new();

    print!("Introduce tu nombre -> ");
    io::stdin().read_line(&mut user).expect("Failed to read line");
    message.push_str(&user);
}