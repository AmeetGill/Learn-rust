fn main() {
    let number = 3;

    if number < 5{
        println!("Codition was true");
    } else {
        println!("Condition was false");
    }
    
    loop_value();
    for_loop();
    for_loop_iter();
}


fn loop_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    };

    println!("The result is {}", result);
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_loop_iter() {
     for number in (1..4).rev() {
        println!("{}!", number);
    }
     for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}