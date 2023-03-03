/// main is a simple function
fn main() {
    println!("{}", my_adder(3, 4));
}

// Example with rustdoc

/// ```
/// /// Some documentation for this function.
/// my_adder(1, 2)
///
/// use std::io;
/// let mut input = String::new();
/// io:stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
fn my_adder(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
#[should_panic(expected = "InvalidDigit")]
fn bad_string() {
    "twenty".parse::<i32>().unwrap();
}

// Example testing one function
#[cfg(test)]
mod test {
    use super::my_adder;

    #[test]
    fn test_my_adder() {
        assert_eq!(my_adder(2, 4), 6);
    }
}
