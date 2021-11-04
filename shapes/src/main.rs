pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > self.width && self.height > other.height
    }
}


fn main() {
    println!("Hello, world!");

    let r1 = Rectangle {width: 80, height: 70};

    let r2 = Rectangle {width: 20, height: 30};

    let r3 = Rectangle {width: 90, height: 60};


    println!("area {}", r1.area() );

    println!("{}", r1.can_hold(&r2) );
}
