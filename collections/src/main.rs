use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut map = HashMap::new();
    map.insert(String::from("Hello"),10);
    map.insert(String::from("Hello1"),50);
    map.entry(String::from("Hello2")).or_insert(34);
    let v2 = vec![1,2,3];

    v.push(5);
    v.push(6);
    v.push(7);

    println!("Vectors found are {:?} {:?} {:?}", v,v2,map);

    let third: &i32 = &v[2];
    println!("THe third element is {}",third);

    match v.get(2) {
        Some(third) => println!("Third elem {} ",third),
        None => println!("There is no third element"),
    }
    
    let mut v = vec![1, 2, 3, 4, 5];
    
    consume(&v);

    // v.push(6);
    
    // let first = &v[0];

    println!("The first element is: {:?}", v);

}

fn consume(vect: &Vec<i32>) {
    println!("Consumed value, now iterating");
    for val in vect {
        println!("value is {:?}",val);
    }
}
