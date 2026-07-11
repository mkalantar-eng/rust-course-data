// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut c = 1;
    loop {
        println!("{c:?}");
        if c > 3 {
            break;
        }

        c = c + 1;
    }
}
