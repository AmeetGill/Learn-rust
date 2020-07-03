use std::fs::File;
use std::io::ErrorKind;

fn main() {
    
}

fn old_read_function() {
    let f = File::open("hello1233.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(new_file) => new_file,
                        Err(err) => panic!("Not able to create file the direcotry")
                    }
                },
                other_error => {
                    panic!("Not able to open the file specified")
                }
            }
        }
    };
}

fn better_file_read_function() {
    let f = File::open("hello1.txt");
    let f = f.unwrap_or_else(|error| {
        if error.kind == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Not able to creat new file")
            });
        } else {
            panic!("Not able to open the specified file");
        }
    });
}

fn best_file_read_function(fileName: str) -> Result<String,io::Error> {

    let s = String::new();

    let f = File::open(fileName)?;
    
}