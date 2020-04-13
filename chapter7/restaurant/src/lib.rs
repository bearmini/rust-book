#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;


use std::collections::HashMap;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // This is more idiomatic way to `use` paths
    hosting::add_to_waitlist();

    // This does the same task as the above, but less information where the function is defined in
    add_to_waitlist();

    // On the other hand, when bringing in structs, enums, and other items with `use`, it's idiomatic to
    // specify the full path.
    let mut map = HashMap::new();
    map.insert(1, 2);


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::fmt;
use std::io;

// The exception to this idiom is if we're bringing two items with the same name into scope with `use`
// statements, because Rust doesn't allow that.
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    Ok(())
}

fn function4() -> IoResult<()> {
    Ok(())
}


// Combining two paths into one use statement:
// use std::io;
// use std::cmp::Ordering
//   ↓
// use std::{cmp::Ordering, io};

// Combining two paths into one use statement:
// use std::io;
// use std::io::Write
//   ↓
// use std::io::{self, Write};

// The Glob Operator
// use std::collections::*;