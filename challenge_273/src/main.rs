pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    // Silence
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    // Check for letters
    let letters: Vec<char> = trimmed.chars().filter(|c| c.is_alphabetic()).collect();
    let has_letters = !letters.is_empty();
    let all_letters_upper = has_letters && letters.iter().all(|c| c.is_uppercase());

    // Question?
    let is_question = trimmed.ends_with('?');

    // Yelled question
    if all_letters_upper && is_question {
        return "Calm down, I know what I'm doing!";
    }

    // Yelling
    if all_letters_upper {
        return "Whoa, chill out!";
    }

    // Question
    if is_question {
        return "Sure.";
    }

    // Default
    "Whatever."
}
