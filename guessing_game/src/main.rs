extern crate rand; // Use extern crate to import libraries

use std::io; // `io` is from std; kinda equivalent to `import io from std` of python
use std::cmp::Ordering;
use rand::Rng;

fn main() { // every program starts with a main function in a main.rs file

    // anything with an `some_func!()` is a macro. If there is no `!`, it is a normal function.
    println!("Guess the number!"); // notice the `!`

    // let to declare variable bindings and will be immutable by default
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // creates an infinite loop
    loop {
        println!("Please input your guess."); // eq to puts

        // declare a var called `guess` which is mutable and is a string
        // `String::new();` creates an instance of a String
        let mut guess = String::new();

        // io::stdin() can also be written as std::io::stdin() removing the use from earlier
        // read_line eq to gets. it takes only mutable string and hence the need to specify `&mut guess`
        // `&mut guess` creates a reference to guess
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line."); // handle the error with `expect()`. It prints the arg when program fails

        // Since guess is a string, we can't compare with a number. Hence, we need to make the string into a number.
        // u32 is unsigned 32 bit number which is safe to use for small numbers.
        // `guess` string can be method-chained.
        // trim eq chomp
        // parse eq to to_i. acts on a string
        // match is to compare the result by parse and will be passed into the block.
        // If it matches `Ok()`, `guess` holds the num.
        // If it matches to `Err()`, it ignores.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} is the placeholder for print statements.
        // If you have multiple variables, you place multiple holders `{}`
        println!("You guessed: {}", guess);

        // cmp acts on a number, takes a reference as an argument to compare and returns an enum
        // this enum contains three variants: less, greater and equal
        // `=>` are called arms coz they connect
        // These arms come from `match` which can be used on the variants of the enum to do some stuff
        // since `loop` is infinite, we insert a `break` to come out of loop once target is achieved
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
