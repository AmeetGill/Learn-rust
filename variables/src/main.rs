fn main() {
    let guess: f64 = "-442.8947".parse().expect("Not a number");
    println!("Hello the number is {}",guess);
  
    let heart_eyed_cat = '😻';
    println!("This is the character {}",heart_eyed_cat);
    
    let tup = (300,44.5,3);
    let (_,y,_) = tup;
    println!("This is something {}",y);
    println!("This is how to access in tuple {}",tup.0);
    
    let arr = [3; 5];
    println!("Array Element {}",arr[0]);
}
    
