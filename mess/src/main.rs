use std::io;
fn main() {
    let mut name = String::new();
    let mut age = String::new();
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Couldn't read line");
    println!("Enter your age");
    io::stdin().read_line(&mut age).expect("Couldn't read line");
    let age: u32 = age.trim().parse().expect("Enter a valid age!");
    println!("Name: {}\nAge: {}", name.trim(), age)
}
