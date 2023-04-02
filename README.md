
# Unofficial Sean Paul's temperature converter

This simple program converts temperatures between Fahrenheit and Celsius as it ouputs "Temperature" lyrics by Sean Paul.

## Origins

It's one of the Rust Book suggested activities in [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) chapter.

### Workflow

Here are some delails about Rust learning curve, for the culture.

#### Getting Started

First and primitive version of the code was written after reading the [Numeric Operations](https://doc.rust-lang.org/book/ch03-02-data-types.html#numeric-operations). The difficulty was to find a way to perfome a numeric operation from a string.

#### Setting up the number parameters and limiting decimals output

Since we can't just perfom numeric operations from strings, we need to convert it to another type of data, in our case, an `f32` scalar, which is a number with a decimal.

The number of decimals is limited to one using this parameter, because no one cares about getting ten decimals of a temperature, tbh:

```rust
 println!("Your temperature is {:.1}° Celsius", converted_temperature);
```

#### Getting input from a new String

In this case, the `parse`method was used to convert a string to another type. This will only work on characters that can logically be converted into numbers. More details can be found in the [Rust Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html?highlight=parse#comparing-the-guess-to-the-secret-number).

Some useful code example can be found [here](https://dev.to/jahwi/a-simple-user-input-collection-validation-and-conversion-library-in-rust-34cj) as well. This post helped to understand I didn't need to create a `celsius` variable and work from the input only and shadow it for simple operations.

## Handling errors

The program handles errors with a simple loop, it just keeps asking for input until a valid one is found.

This is not very satisfying and could be improved, with an explicit message warning against the invalid input, for example.
