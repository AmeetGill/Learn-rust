fn main() {
    println!("Hello, world!");
    
    another_function();
    expressions_function();
    
    let x = five();
    let x = plus_one(x);
    println!("The value of x is {}", x);
    
}

fn another_function(){
    println!("Another function");
}

fn expressions_function(){

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}