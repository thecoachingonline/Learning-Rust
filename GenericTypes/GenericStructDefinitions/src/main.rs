#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    heigth: U
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        heigth: 3u16
    };
    println!("rect is {:?}", rect);
}
