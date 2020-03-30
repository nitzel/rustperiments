use hello::greet;
use rand::Rng;

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

    greet();

    let rnd = rand::thread_rng().gen_range(1,5);
    println!("rand = {}", rnd);

    // characters and string slicing
    let c = 'ä';
    let x = "Dead meat";
    let z = &x[1..2];
    println!("char = {} / {}", c, z);


    // tuples
    //   max-arity: 12, then limited functionality
    let my_tuple = (1, 3.3, "str", 'c');
    println!("tuple {}, {}, {}", my_tuple.0, my_tuple.1, my_tuple.2);

    let (a, b, c, d) = my_tuple;
    println!("deconstructed tuple {}{}{}{}", a, b, c, d);

    // array
    //  max length: 32, then limited functionality
    //  use vectors or slices insted
    let arr: [i32; 5] = [9; 5]; // initialise 5 elements with value '9'
    println!("array1 {} {}", arr[0], arr[4]);
    let arr: [i32; 5] = [1,2,3,4, 5]; // list initialiser
    println!("array2 {} {}", arr[0], arr[4]);

    // control flow: if
    //  no parenthesis required
    let x = 5;
    let y = if x == 5 {
        let a = "five";
        a
    } else if x == 4 {
        "four"
    } else {
        "other"
    };

    println!("if expressions: y = {}", y);

    // unconditional loops and breaking them
    'outer: loop {
        loop {
            break 'outer;
        }
    }

    // while loop
    let mut x = 0;
    while x < 5 {
        println!("{}", x);
        x += 1;
    }

    // looping over iterables
    for num in [7, 8, 9].iter() {
        println!("{}", num);
    }

    for (x, y) in [(1,2), (3,4)].iter() {
        println!("xy {}/{}", x, y);
    }

    print!("exclusive for loop [0,5): ");
    for num in 0..5 {
        print!("{}, ", num);
    }
    print!("\ninclusve for loop [0,5]: ");
    for num in 0..=5 {
        print!("{}, ", num);
    }
    println!(""); // new line

    // Strings
    //  str (pointer to bytes and length)
    //  String(pointer to bytes, length and capacity)
    //  str is often a subset of a String instance
    //    Can't us str[n] to get a character because indexing
    //    should guarantee constant access time, but reading utf-8
    //    isn't doable in constant time. So either get str.bytes()[n] for
    //    the corresponding byte OR iterate the str.chars()
    //     iterators can be 'indexed' via .nth(n)
    println!("Nth char in unicode str: {}", "AsdFgh".chars().nth(3).unwrap());


    let mut x: i32 = 5;
    {
        let y = &x;
        println!("x={} y={}", x, y);
    }
    {
        let y = &mut x;
        (*y) += 1;
        print!("y={}", y);
    }
    println!(" x={}", x);

    //////////
    /// Structs and Traits
    struct Robot {
        name: String
    }

    impl Robot {
        fn new(count: i32) -> Self {
            Self {
                name: format!("Name#{}", count)
            }
        }
        fn to_string(&self) -> String {
            format!("[Robot] '{}'", self.name)
        }
    }

    println!("1: {}", Robot::new(1).to_string()); // 1: [Robot] Name#1

    trait Printable {
        fn to_string(&self) -> String;
    }

    impl Printable for Robot {
        fn to_string(&self) -> String {
            format!("[Trait]{}", Robot::to_string(self))
        }
    }
    println!("2: {}", Robot::new(2).to_string());  // 1: [Robot] Name#2
    println!("3: {}", Printable::to_string(&Robot::new(3)))  // stack overflow
}


fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}