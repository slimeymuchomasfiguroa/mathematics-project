
// Function to calculate the factorial of a given number
fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

// Test the function with different inputs
#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
}
