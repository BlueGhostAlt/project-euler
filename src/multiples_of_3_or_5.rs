// We know that:
// - there are ⌊n / d⌋ numbers divisible by d until n
//   let's call that count m
// - d + 2d + 3d + ... + md = d * (1 + 2 + 3 + ... + m)
// - 1 + 2 + 3 + ... + m = (m * m + 1) / 2
//
// Therefore, the sum of all numbers divisible by d until n is:
// d * (m * (m + 1) / 2)
//
// In order to get the sum of all numbers divisible by a and b
// smaller than or equal to n we apply the formula from above
// for a, b and lcm(a, b), adding the first two and subtracting
// the third as to not count their lcm twice.
pub fn solve(n: u64) -> u64 {
    let mut total = 0;

    let m = n / 3;
    total += 3 * m * (m + 1) / 2;
    let m = n / 5;
    total += 5 * m * (m + 1) / 2;
    let m = n / 15;
    total -= 15 * m * (m + 1) / 2;

    total
}
