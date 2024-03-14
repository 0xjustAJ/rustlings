// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer:i32 = square(3);//first step is to assign they type which is i32 in our case
    println!("The square of 3 is {}", answer);
}

fn square(num: i32)-> i32 {
    num * num
    // the solution is to remove the semicolon added at the back of of the num*num
}
