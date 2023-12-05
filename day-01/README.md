# Day 1: Trebuchet?!

- [Challenge Description](https://adventofcode.com/2023/day/1)

## Table of Contents

- [Part 1](#part-1)
  - [The Problem Statement](#the-problem-statement)
  - [Tests](#tests)
  - [Iterators](#iterators)
  - [What's wrong?](#whats-wrong)
- [Part 2](#part-2)
  - [Refactoring](#refactoring)
  - [Macros](#macros)

## Part 1

### The Problem Statement

Our input, called calibration document, looks like this:

```txt
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

Every line in the document consists of a sequence of letters and numbers.
Our task is to find the **first digit** and the **last digit** in each line and combine them into a two-digit number.
Then we have to sum up all these numbers together, which is our final result.

In this example, the result would be `12 + 38 + 15 + 77 = 142`.

### Tests

Since our input and output has a very clear relationship, it's quite suitable for test-driven development.

We will write a function called `process` that takes a `&str` and returns a `u32`.
Then we will write a test case that calls this function with the sample input and checks that the result is `142`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENT: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_process() {
        let answer = process(DOCUMENT);
        assert_eq!(answer, 142);
    }
}
```

Since we haven't implemented the `process` function yet, this test won't even compile.
Let's fix that by adding a dummy implementation.

```rust
fn process(input: &str) -> u32 {
    0
}
```

Now, the test compiles and fails with the following error message:

```txt
thread 'tests::test_process' panicked at 'assertion failed: `(left == right)`
  left: `0`,
 right: `142`', day-01/src/main.rs:21:9
```

Let's start implementing the solution for this problem.

### Iterators

In Rust, we have a very powerful tool called **iterators**.
They can be used to iterate over a collection of items, along with a lot of convenient methods to transform the items or manipulate the sequence like `map`, `fold` or `filter`.

In our case, we want to first iterate over all lines in the document.
Then we want to iterate over all characters in each line.
Finally, we want to get the first and last character of each line and combine them into a two-digit number.

Let's start with the first step: iterating over all lines in the document.

```rust
fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    println!("The answer is {}", answer);
}

fn process(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}
```

> **Note**:
>
> The `include_str` macro is a convenient way to include the contents of a file as a string literal.
> It expects the file exists in the same directory as the source file.
> In our case, we have to create a file called `input.txt` just next to `main.rs` and put the input document into that file.

Here, we use the `str::lines` method to split the document into lines.
This is a better approach than splitting the document by `\n` because it also works on Windows where the line separator is `\r\n`.

The `lines` method returns an iterator over all lines in the document.
Then we use the `Iterator::map` method with our `process_line` function to transform each line into a two-digit number.
Finally, we use the `sum` method to sum up all these numbers.

Notice that there's no semicolon after the `sum` method.
This is because Rust is an expression-based language, and the last expression in a function automatically becomes the return value of the function.

Now, let's implement the `process_line` function.

```rust
fn process_line(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().expect("Line must contain at least one digit");
    let last = digits.last().expect("Line must contain at least one digit");
    first * 10 + last
}
```

We use the `str::chars` method to get an iterator over all characters in the line.
Then we use `Iterator::filter_map` in combination with the `char::to_digit` method to filter out all characters that cannot be converted to a decimal digit.
With inlay hints, we can see that `rust-analyzer` tells us the type of the `digit` variable is `impl Iterator<Item = u32>`.

#### `Iterator::filter_map` and `char::to_digit`

Why is the type of `digits` an iterator over `u32`?

Let's take a look at the signature of the `to_digit` method:

```rust
pub fn to_digit(self, radix: u32) -> Option<u32>
```

The `to_digit` method takes a `char` and a radix and returns an `Option<u32>`.
If the character cannot be converted to a digit, it returns `None`.
Otherwise, it returns `Some(digit)`.

This is exactly what we need for `Iterator::filter_map`!

```rust
pub fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> Option<B>,
```

Even though the signature looks a bit complicated, it's actually quite simple:

The `filter_map` method takes a function `f` that takes an item of the iterator and returns an `Option<B>`.
When the `f(item)` returns `Some(b)`, there will be a new item `b` in the resulting iterator.
Conversely, when `f(item)` returns `None`, the item will be filtered out.

#### `Iterator::next` and `Iterator::last`

Now that we have an iterator over all digits in the line, we can use the `Iterator::next` and `Iterator::last` methods to get the first and last digit.

```rust
let first = digits.next().expect("Line must contain at least one digit");
let last = digits.last().expect("Line must contain at least one digit");
```

The `next` and `last` methods return an `Option<u32>`, so we use the `expect` method to panic if the iterator is empty.

Finally, we can combine the first and last digit into a two-digit number and return it.

After this change, our test should pass... wait, it doesn't!

```txt
thread 'tests::test_process' panicked at 'Line must contain at least one digit', day-01/src/main.rs:12:30
```

### What's wrong?

Let's carefully inspect the input document again:

```txt
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

As you can see, the last line contains only one digit `7`, and we should return `77`.

However, our code expects that every line contains at least two digits. This is because the `Iterator::next` consumes the first digit, and the `Iterator::last` consumes the whole iterator and returns the last digit in the iterator.

We can fix this by cloning the iterator and calling `next` on the clone:

```rust
let digits = line.chars().filter_map(|c| c.to_digit(10));
let first = digits.clone().next().expect("Line must contain at least one digit");
let last = digits.last().expect("Line must contain at least one digit");
```

Note that we don't have to specify `digits` as `mut` because `Iterator::clone` provides a new iterator to the `Iterator::next` method, and `Iterator::last` takes the ownership of the iterator instead of borrowing it.

Now, our test passes!

If we run our program with the real input, we get the following result (your answer may be different):

```txt
The answer is 54239
```

## Part 2

In the second part of the challenge, the rules are similar to the first part, but with a twist: now `one`, `two`, `three`, ..., `nine` are also valid digits. That is, in this example:

```txt
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

The result would be `29 + 83 + 13 + 24 + 42 + 14 + 76 = 281`.

Let's write a test case for this:

```rust
const DOCUMENT2: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn test_process_part2() {
    let answer = process_part2(DOCUMENT2);
    assert_eq!(answer, 281);
}
```

Now, we have to implement the `process_part2` function. Since the rules are similar to the first part, we can just copy the `process` function and change the function we pass to `Iterator::map`.

```rust
fn process_part2(input: &str) -> u32 {
    input.lines().map(process_line_part2).sum()
}
```

The problem statement isn't very clear about what to do with overlapping digits, and none of the examples contain overlapping digits.
Let me tell you the answer before you waste your time trying to figure it out: overlapping digits are counted multiple times.

For example, in this line:

```txt
four3twone
```

The digits would be `4`, `3`, `2`, and `1`.

With this in mind, we add test cases for both non-overlapping and overlapping digits. Here we're going to use [`rstest`](https://github.com/la10736/rstest) to write parameterized tests:

```rust
use rstest::rstest;

#[rstest]
#[case("two1nine", 29)]
#[case("eightwothree", 83)]
#[case("abcone2threexyz", 13)]
#[case("xtwone3four", 24)]
#[case("4nineeightseven2", 42)]
#[case("zoneight234", 14)]
#[case("7pqrstsixteen", 76)]
/// This checks we count overlapping digits multiple times.
#[case("four3twone", 41)]
#[case("oneight", 18)]
fn test_process_line_part2(#[case] line: &str, #[case] expected: u32) {
    let actual = process_line_part2(line);
    assert_eq!(actual, expected);
}
```

Now, we can implement the `process_line_part2` function:

```rust
fn process_line_part2(line: &str) -> u32 {
    let first = find_first_digit(line).expect("Line must contain at least one digit");
    let last = find_last_digit(line).expect("Line must contain at least one digit");
    first * 10 + last
}

fn find_first_digit(line: &str) -> Option<u32> {
    let mut chars = line.chars();
    for idx in 0..line.len() {
        if let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
            return Some(digit);
        }
        let sliced = &line[..=idx];
        if sliced.ends_with("one") {
            return Some(1);
        } else if sliced.ends_with("two") {
            return Some(2);
        } else if sliced.ends_with("three") {
            return Some(3);
        } else if sliced.ends_with("four") {
            return Some(4);
        } else if sliced.ends_with("five") {
            return Some(5);
        } else if sliced.ends_with("six") {
            return Some(6);
        } else if sliced.ends_with("seven") {
            return Some(7);
        } else if sliced.ends_with("eight") {
            return Some(8);
        } else if sliced.ends_with("nine") {
            return Some(9);
        }
    }
    None
}

fn find_last_digit(line: &str) -> Option<u32> {
    let mut chars = line.chars().rev();
    for idx in (0..line.len()).rev() {
        if let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
            return Some(digit);
        }
        let sliced = &line[idx..];
        if sliced.starts_with("one") {
            return Some(1);
        } else if sliced.starts_with("two") {
            return Some(2);
        } else if sliced.starts_with("three") {
            return Some(3);
        } else if sliced.starts_with("four") {
            return Some(4);
        } else if sliced.starts_with("five") {
            return Some(5);
        } else if sliced.starts_with("six") {
            return Some(6);
        } else if sliced.starts_with("seven") {
            return Some(7);
        } else if sliced.starts_with("eight") {
            return Some(8);
        } else if sliced.starts_with("nine") {
            return Some(9);
        }
    }
    None
}
```

The `find_first_digit` and `find_last_digit` functions are very similar, so let's just focus on the `find_first_digit` function.

1. First, we create an iterator over all characters.
2. Then we iterate over all indices in the line.
3. For each index, we first check if the character at that index is a digit.
   If it is, we return it.
4. Otherwise, we check if the line ends with `one`, `two`, `three`, ..., `nine`.
   If it does, we return the corresponding digit.
5. If we reach the end of the loop, we return `None`. (This should never happen in this challenge, otherwise it's the problem of the input document.)

The `find_last_digit` function is similar, but we iterate over the indices in reverse order and check if the line starts with `one`, `two`, `three`, ..., `nine`.

In this way, we only find the first and last digit in each line.
We don't have to worry about overlapping digits because we find the digits from both ends of the line, it should be the same as counting the overlapping digits multiple times.

### Refactoring

I know, I know, the code is ugly.
But let's verify that it works before we refactor it by running the test:

```txt
test tests::test_process_part2 ... ok
```

Now, we can refactor the code by writing some **macros** 🪄.

```rust
macro_rules! match_suffix {
    ($sliced:expr, $($suffix:expr => $value:expr),* $(,)?) => {
        $(if $sliced.ends_with($suffix) {
            return Some($value);
        })*
    };
}

macro_rules! match_prefix {
    ($sliced:expr, $($prefix:expr => $value:expr),* $(,)?) => {
        $(if $sliced.starts_with($prefix) {
            return Some($value);
        })*
    };
}
```

Then we can use these macros to simplify the `find_first_digit` and `find_last_digit` functions:

```rust
fn find_first_digit(line: &str) -> Option<u32> {
    let mut chars = line.chars();
    for idx in 0..line.len() {
        if let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
            return digit;
        }
        let sliced = &line[..=idx];
        match_suffix! {
            sliced,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
        };
    }
    None
}

fn find_last_digit(line: &str) -> Option<u32> {
    let mut chars = line.chars().rev();
    for idx in (0..line.len()).rev() {
        if let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
            return digit;
        }
        let sliced = &line[idx..];
        match_suffix! {
            sliced,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
        };
    }
    None
}
```

Now, we can run the test again to verify that our refactoring didn't break anything:

```txt
test tests::test_process_part2 ... ok
```

### Macros

Macros are pretty amazing, aren't they? 😎
Let's take a look at the `match_suffix` macro:
(`match_prefix` should be similar.)

```rust
macro_rules! match_suffix {
    ($sliced:expr, $($suffix:expr => $value:expr),* $(,)?) => {
        $(if $sliced.ends_with($suffix) {
            return Some($value);
        })*
    };
}
```

The macro takes an expression `$sliced` followed by zero or more of patterns `$suffix => $value`. The `$suffix` is the suffix we want to match, and the `$value` is the value we want to return if the suffix matches.

We used the repetition syntax `$(...)*` to mark the input pattern can be repeated zero or more times. Check out the [Rust reference](https://doc.rust-lang.org/reference/macros-by-example.html#repetition) for more information about repetition.
The final `$(,)?` is used to make the comma after the last pattern optional.

For example, we can use the macro like this:

```rust
match_suffix! {
    sliced,
    "hello" => 1,
    "world" => 2,
};
```

After macro expansion, this is equivalent to:

```rust
if sliced.ends_with("hello") {
    return Some(1);
}
if sliced.ends_with("world") {
    return Some(2);
}
```

Finally, we can run our program with the real input:

```txt
The answer is 55343
```

That's it for today, see you tomorrow! 👋

[Next day](../day-02/README.md)
