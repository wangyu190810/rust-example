extern crate phraese;

fn main() {
    println!("Hello in English: {}", phraese::english::greetings::hello());
    println!("Goodbye in English: {}", phraese::english::farewell::goodbye());

    //println!("Hello in Japanese: {}", phraese::japanese::greetings::hello());
    //println!("Goodbye in Japanese: {}", phraese::japanese::farewells::goodbye());
}
