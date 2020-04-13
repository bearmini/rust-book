fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // Another way to create a string
    let s = String::from("initial contents");

    let hello = String::from("こんにちは");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // If the `push_str()` method took ownership of `s2`, we wouldn't be able to do this. However, this code works as we'd expect!

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s1 = String::from("hello");
    // let h = s1[0]; // Rust strings don't support indexing.

    let len = String::from("Hola").len();
    println!("len == {}", len);

    let len = String::from("Здравствуйте").len();
    println!("len == {}", len);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s == {}", s);

    //let s = &hello[0..1]; // This causes the program to panic

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
