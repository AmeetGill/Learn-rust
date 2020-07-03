
struct RectanlgeTuple(u32,u32);

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
    name: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height*self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn clear(self) {
        println!("Rectangle cleared");
    }
}

impl Rectangle{
    fn generate_square(x: u32) -> Rectangle {
        Rectangle {
            width: x,
            height: x,
            name: String::from("temp_name")
        }
    }
}

fn main() {
    let x = 23;
    let y = 43;
    
    let rect_name = String::from("rect1");
    
    
    let rect1 = Rectangle {
        width: 23,
        height: 43,
        name: rect_name
    };

    let rect2 = Rectangle {
        width: 23,
        height: 43,
        name: String::from("rect2")
    };
    
    println!("print the array {:#?} ",rect1);
    
    let area_calualted = area_individual(x,y);
    
    let rectangle = RectanlgeTuple(23,43);
    
    let area_calualted_new = area_tuple(rectangle);
    
    let area_calculated = area(&rect1); 
    
    println!("Calculated Area: {} \n New area {} \n Best method area {} ", area_calualted, area_calualted_new,area_calculated);
    
    println!("Rectangle is {:?}", rect1);
     println!("Rectangle is {:#?}", rect1);

    println!("Printing area using method {}",rect1.area());
    rect1.clear();
    let rect1 = Rectangle::generate_square(43);
    println!("can hold {} ", rect1.can_hold(&rect2));

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width*rectangle.height
}

fn area_individual(x: u32, y: u32) -> u32 {
    x*y    
}

fn area_tuple(tup: RectanlgeTuple) -> u32 {
    tup.0*tup.1
}

