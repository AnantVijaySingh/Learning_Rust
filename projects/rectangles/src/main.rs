
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels", area_1(width, height));


    let rect1:(u32, u32) = (30, 50);
    println!("The area of the rectangle is {} square pixels", area_2(rect1));


    let rectangle1 = Rectangle {
        width: 30,
        height: 50
    };
    dbg!(&rectangle1);
    println!("The area of the rectangle is {} square pixels", area_3(&rectangle1));
}

fn area_1 (width: i32, height: i32) -> i32 {
    width * height
}

fn area_2 (dimenssions: (u32, u32)) -> u32 {
    dimenssions.0 * dimenssions.1
}

fn area_3 (dimenssions: &Rectangle) -> u32 {
    dimenssions.width * dimenssions.height
}

