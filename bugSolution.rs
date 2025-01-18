fn main() {
    let mut x = 5;
    { // Using a scope to limit the mutable borrow
        let y = &mut x;
        *y += 1;
    }
    { // another mutable borrow
        let z = &mut x;
        *z += 1; 
    }
    println!("x = {}", x);
}
//Alternative solution using clone:
// fn main() {
//     let mut x = 5;
//     let mut y = x;
//     let mut z = x;
//     y += 1;
//     z += 1;
//     x = y + z - 10; //Note that you need to refactor the logic
//     println!("x = {}", x);
// }