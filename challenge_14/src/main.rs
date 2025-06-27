pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            // Push opening brackets to the stack
            '(' | '[' | '{' => stack.push(c),
            // For closing brackets, check if the top of the stack matches
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            // Ignore all other characters
            _ => continue,
        }
    }

    // If the stack is empty, all brackets were matched
    stack.is_empty()
}
