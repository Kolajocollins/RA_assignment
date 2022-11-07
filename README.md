# RA_assignment

Question:
> Write a code that calculates the Modular multiplicative inverse of a number using the Euclidean Algorithm?

# Theory
The modular multiplicative inverse of an integer a is an integer x such that the product ax is congruent to 1 with respect to the modulus m. In the standard notation of modular arithmetic this congruence is written as

ax $\equiv$ 1 ${\pmod {m}}$  (Source: Wikipedia)

I did some reading about the theorem here: https://www.math.utah.edu/~fguevara/ACCESS2013/Euclid.pdf

After trying it on paper and also looking through a library implementation in rust, I came up with this implementation.

## How to compile and run

You need to have rust installed to compile

Installation instructions here: https://www.rust-lang.org/tools/install

You can run it using cargo run a m where a and m is the number and its modulo for which you want to find the inverse respectively

Some tests you may run include

>cargo run 12345 5431

The expected output is "The inverse is 5160"

> cargo run 123 27

The expected output is "The inverse does not exist". 
This is because some inverses dont exist for certain modulos.

## How to run 

You can also directly run the binaries that can be found in target/release

This is to be done from a terminal environment.

> ./RA_assignment a m.

The only dependency in this case should be gcc