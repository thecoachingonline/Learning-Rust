use std::fs;

fn main() {
    let contents = fs::read_to_string("answer_to_the_ultimate_question.txt").unwrap();
    println!("contents is: {:?}", contents);
}
