#[path = "ch2.rs"]
mod chapterx;

use std::io;

fn main() {
    println!("## main.rs ##");

    chapterx::main();

    println!("\n\nPress any key to exit");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("whatever");
}
