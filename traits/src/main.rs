
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(arr: &[T]) -> Result<T,Box<dyn Error>> {
    let mut largest = arr[0];

    for &item in arr {
        if item > largest {
            largest = item;
        }
    }

    Ok(largest)
}

fn largest_ref<T: PartialOrd  >(arr: &[T]) -> Result<&T,Box<dyn Error>> {
    let mut largest = 0;

    for i in 0..arr.len() {
        if arr[i] > arr[largest] {
            largest = i;
        }
    }

    Ok(&arr[largest])
}

fn largest_with_an_aanouncement <'a,T>(
    x : &'a str,
    y : &'a str,
    ann : T
) -> &'a str
where T : Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else{
        y
    }
}

fn largest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}


struct Point<T>{
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            // fkjhk
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub struct Twitter {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}


impl Summary for Twitter {
    fn summarize(&self) -> String {
        format!("{}, by {} ", self.content,  self.username )
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})",self.headline,self.author,self.location)
    }
}

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("notifying {} and {}", item1.summarize(),item2.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("notifying {} and {}", item1.summarize(),item2.summarize());
}

fn some_function<T, U> (t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug   {
        43    
}

impl Point<f32> { // this implemntation block is only for f32 type
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let arr = [23,234,412,123,3312,3,112,31,2];
    let largest = largest(&arr).unwrap();

    let integerPoint = Point{x: 234, y:234};
    let floatPoint = Point{x: 23.4,y:33.4};

    println!("largest value is {} ",largest);

    let tweet1 = Twitter{
        username: String::from("ameetgill"),
        content: String::from("Rust is awesome"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet {} ",tweet1.summarize());

    
}
