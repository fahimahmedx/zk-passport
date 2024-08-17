//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use fibonacci_lib::{check_passport, Passport};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let number = sp1_zkvm::io::read::<String>();
    let country = sp1_zkvm::io::read::<String>();
    let passport = Passport {
        number: number.clone(),
        country: country.clone(),
    };

    let is_real_passport = check_passport(&passport);

    sp1_zkvm::io::commit(&number);
    sp1_zkvm::io::commit(&country);
    sp1_zkvm::io::commit(&is_real_passport);
}
