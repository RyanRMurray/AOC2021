mod utils;

fn main() {
    println!("Hello, world!");

    let mut x = utils::Answer::default();

    x.record(&"test");

    x.record(&444);

    print!("{}",x);
}
