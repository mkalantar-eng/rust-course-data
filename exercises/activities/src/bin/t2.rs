fn main() {
    let i = &7;
    let b: &i32;
    b = i;
    println!("{:?}", b);
    println!("{:?}", i);
}
