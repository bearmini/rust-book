use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    /*
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r); // x does not live long enough
    */

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("abcd");
    {
        let string4 = "xyz";
        let result = longest(string3.as_str(), string4);
        println!("The longest string is {}", result);
    }

    /*
    let string5 = String::from("abcd");
    let result;
    {
        let string6 = String::from("xyz");
        result = longest(string5.as_str(), string6.as_str());
    }
    println!("The longest string is {}", result); // string6 does not live long enough
    */

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    let part = i.announce_and_return_part("ladies and gentlemen!");
    println!("{}", part);

    let s: &'static str = "I have a static lifetime.";
}
