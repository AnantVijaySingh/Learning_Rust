
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    //methods
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(self: &Self) -> bool {
        self.width > 0
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated functions
    fn square(dimensions: u32 ) -> Self {
        Self {
            width: dimensions,
            height: dimensions
        }
    }
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

    println!("The area of the rectangle is {} square pixels calculated using method instead of a function", rectangle1.area());


    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    let sq = Rectangle::square(10);

    println!("The area of the rectangle with equal width and height is {} square pixels", sq.area());

}

fn area_1 (width: i32, height: i32) -> i32 {
    width * height
}

fn area_2 (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3 (dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

