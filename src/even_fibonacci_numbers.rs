use project_euler::fib::Fibonacci;

pub fn solve(n: u64) -> u64 {
    Fibonacci::<u64>::new()
        .take_while(|&x| x < n)
        .filter(|&x| x % 2 == 0)
        .sum()
}
