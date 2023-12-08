fn main() {
    let value = 0b1111_0101u8; /* NOT, AND, OR, XOR, SHIFT */
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value;
    println!("value is {:08b}", value);

    value = value & 0b1111_0101;
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value);

    value = value << 4;
    println!("value is {:08b}", value);

    value = value >> 2;
    println!("value is {:08b}", value);
}
