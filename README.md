
# Fahrenheit to Celsius temperatures converter

This simple program converts Fahrenheit temperatures to Celsius. It's one of the Rust Book suggested activities in [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) chapter.

## Workflow

Here are some delails about Rust learning curve.

### Primitive code

First and primitive version of the code looked like this :

`/src/main.rs

```rust

fn main() {

    let fahrenheit:i32 = -5;

    let celsius: i32 = fahrenheit - 17;

    println!("When the Fahrenheit temperature is {fahrenheit} the Celsius temperature is {celsius}");

}

```

This was written after reading the [Numeric Operations](https://doc.rust-lang.org/book/ch03-02-data-types.html#numeric-operations). The difficulty was to find a way to perfome a numeric operation from a string.

## Converting a String

Since we can't just perfom numeric operations from strings, we need to convert it to another type of data, in our case, an `ì32` integrer (which is a number without a fractional component).

In this case, the `parse`method was used to convert a string to another type. This will only work on characters that can logically be converted into numbers. More details can be found in the [Rust Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html?highlight=parse#comparing-the-guess-to-the-secret-number).

Some useful code example can be found [here](https://dev.to/jahwi/a-simple-user-input-collection-validation-and-conversion-library-in-rust-34cj) as well. This post helped to understand I didn't need to create a `celsius` variable and work from the input only and shadow it.

## Handling errors

The program handles errors with a simple loop, it just keeps asking for input until a valid one is found.

This is not very satisfying and could be improved, with an explicit message warning against the invalid input, for example.
