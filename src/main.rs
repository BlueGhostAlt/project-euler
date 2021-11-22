pub mod even_fibonacci_numbers;
pub mod multiples_of_3_or_5;

fn main() {
    assert_eq!(multiples_of_3_or_5::solve(999), 233168);
    assert_eq!(even_fibonacci_numbers::solve(4_000_000), 4613732);
}
