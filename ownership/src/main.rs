fn main() {
   let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("{}, {} world!", s1,s2);
    
    let s2 = String::from("Hello");
    let s1 = new_function(s2);
    let s3 = 2;
    let s4 = s3;
    let s5 = String::from("Hello, bye");
    let s6 = s5;
    println!("value from function {} {} {} {}",s1,s3,s4,s6);

    let s1 = String::from("hello");
    let mut s2 = String::from("hello2");
    let s3 = String::from("hello");

    
    let len = calculate_length(&s1);
    let len2 = calculate_length(&s1);
    
    let r1 = &mut s2;
    
    change(r1);
    change(&mut s2);
    
    
    let len3 = calculate_length(&s2);
    
    change(&mut s2);
    // change(r1);      // this will result in error

    
    let len4 = calculate_length(&s2);
    
    // let reference_to_nothing = dangle();
    
    let first_word_index = first_word(&s3);
    
    println!("The length of '{}' is {}. {} {} {} {} {} ", s1, len, len2,s2,len3,len4,first_word_index);
   
}

fn new_function(mut s: String) -> String {
    s.push_str(" append It");
    s   
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    str.len()
}