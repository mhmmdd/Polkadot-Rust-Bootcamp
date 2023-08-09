fn main() {
    // define a closure that takes an integer and returns a boolean

    let is_even = |x: i32| -> bool { x % 2 == 0 };
    // let is_even = |x: i32| x % 2 == 0;

    // call the closure
    println!("Is 7 even? {}", is_even(7));


    // define a closure that prints some text after receiving data
    let print_data = |data: &str| {
        println!("Received data: {}", data);
    };

    // call the closure
    download_data("https://www.rust-lang.org/", print_data);
}

///
/// Fn, FnMut, and FnOnce traits are implemented for closures
/// Fn: the closure captures by reference (&T)
/// FnMut: the closure captures by mutable reference (&mut T)
/// FnOnce: the closure captures by value (T)
///
fn download_data(url: &str, callback: impl FnOnce(&str) -> ()) {
    // download the data here
    let data = format!("Some data from {}", url);
    callback(&data);
}


