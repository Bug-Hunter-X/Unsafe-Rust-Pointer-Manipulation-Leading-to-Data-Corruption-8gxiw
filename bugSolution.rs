fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe and preferred way to modify the vector element
    println!("The first element is: {}", v[0]);
}
