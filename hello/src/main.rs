fn main() {
    println!("Hello, world!");
    let mut bunnies = 4;
    // let bunnies = 5;
    bunnies += 3;
    println!("{}", bunnies);
    {
        let bunnies = 2;
        println!("{}", bunnies);
    }
    println!("{}", bunnies);

    if bunnies > 3 {
        println!(">3")
    }
    else {
        println!("<3")
    }

    println!("{}", do_stuff(3., 4.));
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}