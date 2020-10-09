#[path = "ch5.rs"]
mod chapterx;

use std::io;

#[allow(dead_code)]
fn wait_for_input() {
    println!("\n\nPress any key to exit");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("whatever");
}

fn main() {
    println!("## main.rs ##");

    chapterx::main();

    // wait_for_input();
}
