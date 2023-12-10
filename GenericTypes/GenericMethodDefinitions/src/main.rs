#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    heigth: U
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.heigth
    }
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        heigth: 3u8
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
}
