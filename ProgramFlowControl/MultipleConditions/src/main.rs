fn main() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater then y");
    } else {
        if x < y {
            println!("x is less then y");
        } else {
            println!("x is equal to y");
        }
    }

    if x > y {
        println!("x is greater then y");
    } else if x < y {
        println!("x is less then y");
    } else {
        println!("x is equal to y");
    }
}
