// Function to count how many eggs (1s) are in the binary display
pub fn egg_count(display_value: u32) -> usize {
    display_value.count_ones() as usize
}

fn main() {
    // Example display values to test
    let display_values = [0, 1, 5, 13, 255]; // Binary: 0b0, 0b1, 0b101, 0b1101, 0b11111111

    for &value in &display_values {
        let eggs = egg_count(value);
        println!(
            "Display shows {} (binary: {:b}) → 🥚 {} egg(s)",
            value, value, eggs
        );
    }
}
