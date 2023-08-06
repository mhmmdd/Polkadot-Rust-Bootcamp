///
/// Box pointer is a smart pointer that points to data on the heap.
/// It is used to store data on the heap rather than on the stack.
/// Stack is limited in size and cannot grow dynamically.
///
fn main() {
    let mut b = Box::new(5);
    println!("b = {}", b);

    {
        let temp = Box::new(4);
        println!("temp = {}", temp);
    }
    // println!("temp = {}", temp); // error[E0425]: cannot find value `temp` in this scope

    // change the value of b
    *b = 10;
    println!("b = {}", b);
}


