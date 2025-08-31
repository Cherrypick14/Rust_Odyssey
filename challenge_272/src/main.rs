pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None; // Collatz only defined for positive integers
    }

    let mut steps = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            // checked math to avoid overflow: 3n + 1
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        steps += 1;
    }

    Some(steps)
}
