fn use_value (_val: Demo) {

}

struct Demo {
    a: i32,
}

fn main () {
    let demo = Demo { a: 123 };
    use_value(demo);
    println!("{}", demo.a);
}