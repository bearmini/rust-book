fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("({}, {}, {})", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let index = 1;
    let second = a[index];
    println!("a = {:?}, first = {}, second = {}", a, first, second);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("months = {:?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);

    let a = [3; 5];
    println!("a = {:?}", a);

}
