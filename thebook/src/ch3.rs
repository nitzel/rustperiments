// Try building programs to do the following:
/// Convert temperatures between Fahrenheit and Celsius.
fn fahrenheit2celsius(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}
fn celsius2fahrenheit(c: f64) -> f64 {
    c * 9. / 5. + 32.
}
// Generate the nth Fibonacci number.
fn fibo(n: u32) -> u128 {
    let mut f1 = 0;
    let mut f2 = 1;
    for _ in 0..n {
        let old_f1 = f1;
        f1 = f2;
        f2 += old_f1;
    }
    f1
}

/// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn get_carol_row(n: usize) -> String {
    let data = vec![
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "tgree French hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five gold rings"),
        ("sixth", "six geese a laying"),
        ("seventh", "seven swans a swimming"),
        ("eighth", "eight maids a milking"),
        ("ninth", "nine ladies dancing"),
        ("tenth", "ten lords a leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelth", "twelve drummers drumming"),
    ];
    let mut s: String =
        String::new() + "On the " + data[n].0 + " day of Christmas my true love gave to me\n";

    for i in (0..(n + 1)).rev() {
        let row = data[i].1;
        s += (match i {
            0 => {
                if n == 0 {
                    format!(" {}\n", row)
                } else {
                    format!(" and {}\n", &row)
                }
            }
            _ => format!(" {}\n", &row),
        })
        .as_str()
    }
    s
}

/// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
pub fn get_carol() -> String {
    let mut s = String::new();
    for i in 0..12 {
        s += get_carol_row(i).as_str();
    }
    s
}

pub fn main() {
    println!("32F = {}°C", fahrenheit2celsius(32.0));
    println!("100°C = {}F", celsius2fahrenheit(100.0));
    for i in 0..10 {
        println!("Fibo #{} is {}", i, fibo(i));
    }

    println!("{}", get_carol());
}

#[cfg(test)]
#[macro_use]
pub mod tests {
    use crate::chapterx::celsius2fahrenheit;
    use crate::chapterx::fahrenheit2celsius;
    use crate::chapterx::fibo;

    #[allow(unused_macros)]
    macro_rules! assert_delta {
        ($x:expr, $y:expr) => {
            let diff = $x - $y;
            let relative_diff = (diff / (if $x == 0. { 1. } else { $x })).abs();
            if relative_diff > 0.001 {
                panic!(
                    "Expected {}, but was {}, difference={}, relative={}",
                    $x, $y, diff, relative_diff
                );
            }
        };
    }
    #[test]
    fn fahrenheit2celsius_test() {
        assert_delta!(fahrenheit2celsius(32.), 0.);
        assert_delta!(fahrenheit2celsius(86.), 30.);
        assert_delta!(fahrenheit2celsius(212.), 100.);
    }

    #[test]
    fn celsius2fahrenheit_test() {
        assert_delta!(celsius2fahrenheit(0.), 32.);
        assert_delta!(celsius2fahrenheit(30.), 86.);
        assert_delta!(celsius2fahrenheit(100.), 212.);
    }

    #[test]
    fn fibo_test() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(99), 218922995834555169026);
    }
}
