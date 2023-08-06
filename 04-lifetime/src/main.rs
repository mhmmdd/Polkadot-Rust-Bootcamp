///
/// 1. If you have a reference in a struct, you need to specify a lifetime for that reference.
///
// struct Person {
//     name: &str,
// }

///
/// 2. If your function returns a reference, you need to specify a lifetime for that reference.
///
// fn min(x: &str, y: &str) -> &str {
//     if x < y {
//         x
//     } else {
//         y
//     }
// }

///
/// When you use a reference in a struct, you need to specify a lifetime for that reference.
///
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    // example 1
    let name = String::from("Alice");
    let person = Person { name: &name };
    person.greet();


    // example 2
    let person;
    {
        let name = String::from("Alice");
        person = Person { name: &name };
    }
    // name is dropped here
    // println!("Name: {}", person.name); // Error!
}


