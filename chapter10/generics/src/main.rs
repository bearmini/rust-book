fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn largest_generic_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);


    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    println!("The largest number is {}", largest_generic_copy(&number_list));
    println!("The largest char is {}", largest_generic_copy(&char_list));
    let string_list = vec!["abc", "xyz", "aaa", "ab"];
    println!("The largest string is {}", largest_generic_copy(&string_list));

    println!("The largest number is {}", largest_generic_clone(&number_list));
    println!("The largest char is {}", largest_generic_clone(&char_list));
    println!("The largest string is {}", largest_generic_clone(&string_list));

    println!("The largest number is {}", largest_generic_ref(&number_list));
    println!("The largest char is {}", largest_generic_ref(&char_list));
    println!("The largest string is {}", largest_generic_ref(&string_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wont_work = Point { x: 5, y: 4.0 };
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p = Point { x: 1.0, y: 4.0 };
    println!("Distance from the origin = {}", p.distance_from_origin());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
