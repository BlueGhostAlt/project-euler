pub fn sum_divisible_by_until(n: u64, d: u64) -> u64 {
    let count = n / d;

    d * count * (count + 1) / 2
}

pub fn solve(n: u64) -> u64 {
    sum_divisible_by_until(n, 3) + sum_divisible_by_until(n, 5) - sum_divisible_by_until(n, 15)
}
