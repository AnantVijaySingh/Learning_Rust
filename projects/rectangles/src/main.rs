
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    //methods
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated functions
    fn square(dimensions: u32 ) -> Self {
        Self {
            width: dimensions,
            height: dimensions,
            name: "Square".to_string(),
        }
    }

    fn max (self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);

        Rectangle {
            width: w,
            height: h,
            name: String::from("max"),
        }
    }

    // fn set_to_max (&mut self, other: Rectangle) {
    //     *self = self.max(other);
    // }
}

fn main() {
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels", area_1(width, height));


    let rect1:(u32, u32) = (30, 50);
    println!("The area of the rectangle is {} square pixels", area_2(rect1));


    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
        name: "rectangle1".to_string(),
    };
    dbg!(&rectangle1);
    println!("The area of the rectangle is {} square pixels", area_3(&rectangle1));

    println!("The area of the rectangle is {} square pixels calculated using method instead of a function", rectangle1.area());


    let rect2 = Rectangle {
        width: 30,
        height: 50,
        name: "rect2".to_string(),
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
        name: "rect3".to_string(),
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
        name: "rect4".to_string(),
    };

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    let sq = Rectangle::square(10);

    println!("The area of the rectangle with equal width and height is {} square pixels", sq.area());

    let mut r1 = Rectangle {
        width: 9,
        height: 9,
        name: String::from("r1"),
    };

    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2"),
    };

    // r1.set_to_max(r2);
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

