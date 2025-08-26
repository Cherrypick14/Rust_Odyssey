pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();

    let mut result = Vec::new();

    for r in 0..rows {
        let mut row = String::new();
        for c in 0..cols {
            if garden[r].as_bytes()[c] == b'*' {
                // If it's a flower, keep it
                row.push('*');
            } else {
                // Otherwise, count adjacent flowers
                let mut count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0
                            && nr < rows as isize
                            && nc >= 0
                            && nc < cols as isize
                            && garden[nr as usize].as_bytes()[nc as usize] == b'*'
                        {
                            count += 1;
                        }
                    }
                }
                if count == 0 {
                    row.push(' ');
                } else {
                    row.push(char::from_digit(count, 10).unwrap());
                }
            }
        }
        result.push(row);
    }

    result
}
