use rand::Rng;

pub fn print_in_box(title: &str, messages: &[&str]) {
    let max_length = messages.iter().map(|s| s.len()).max().unwrap_or(0);
    let total_length = max_length + 4; // Adding space for padding and box sides
    let title_length = title.len() + 2; // Adding space for padding around the title
    let border_length = total_length - title_length;
    let left_border = "═".repeat(border_length / 2);
    let right_border = "═".repeat(border_length - border_length / 2);

    println!("\n╔{} {} {}╗", left_border, title, right_border);
    for &message in messages {
        let padding = " ".repeat(max_length - message.len() + 2);
        println!("║ {}{} ║", message, padding);
    }
    println!("╚{}╝\n", "═".repeat(total_length));
}

pub fn operations(a: i32, b: i32) {
    let messages = [
        format!("Passed i32 numbers: {}, {}", a, b),
        format!("Subtraction: a - b = {}", a - b),
        format!("Multiplication: a * b = {}", a * b),
        if b != 0 {
            format!("Division: a / b = {}, -a / b = {}", a / b, -a / b)
        } else {
            "Division: Cannot divide by zero".to_string()
        },
        if b != 0 {
            format!("Remainder: a % b = {}, b % a = {}", a % b, b % a)
        } else {
            "Remainder: Cannot compute remainder with zero".to_string()
        },
    ];
    let message_slices: Vec<&str> = messages.iter().map(AsRef::as_ref).collect();
    print_in_box("operations", &message_slices);
}

pub fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..=100);
    let message = format!("Here is the random number: {}", random);
    print_in_box("randomizer", &[&message]);
    random
}


