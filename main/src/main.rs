// use hello_rust;

// fn main() {
//     println!("{}", hello_rust::hello_rust());
// }

fn main() {
    let v = vec![1, 2, 3, 4];
    for item in v {
        println!("{}", item)
    }
    println!("{:?}", v)
}
