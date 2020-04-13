use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    let f = File::open("hello.txt");
    // If we give a type annotation that we know is _not_ the return type of the function
    // and then try to compile the code, the compiler tell us that the types don't match.
    // The error message will then tell us what the type of f is. Let's try it!
    // let f: u32 = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("problem opening the file: {:?}", other_error),
        },
    };

    // A more seasoned Rustacean might write this code isntead of the above
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let s = read_username_from_file().unwrap();
    println!("{}", s);

    let s = read_username_from_file2().unwrap();
    println!("{}", s);

    let s = read_username_from_file3().unwrap();
    println!("{}", s);

    let s = read_username_from_file4().unwrap();
    println!("{}", s);

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}