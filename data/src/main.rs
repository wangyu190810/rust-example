extern crate data;

use data::english::greetings;
use data::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
//    println!("Goodbye in English: {}", phrases::english::farewells::goodbye());
//
    println!("Hello in Japanese: {}", japanese::greetings::hello());
	println!("goodbye in Japanese: {}", japanese::greetings::goodbye());
//    println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());
}