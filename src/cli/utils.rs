use rand::Rng;

pub fn print_in_box(title: &str, messages: &[&str]) {
    let terminal_width = term_size::dimensions().map_or(80, |(w, _)| w);
    let max_message_length = terminal_width - 4; // Subtracting box borders and padding

    let title_length = title.len();
    let total_border_length = terminal_width - title_length - 2; // Subtracting title and box corners
    let left_border_length = total_border_length / 2;
    let right_border_length = total_border_length - left_border_length;

    // Use different characters based on the OS
    let (newline, horizontal_line, vertical_line, left_top_corner, right_top_corner, right_bottom_corner, left_bottom_corner) = if cfg!(target_os = "windows") {
        ("\r", "-", "|", "+", "+", "+", "+") // ASCII characters for Windows
    } else {
        ("\n", "═", "║", "╔", "╗", "╝", "╚") // Unicode characters for Unix-like systems
    };    

    let left_border = horizontal_line.repeat(left_border_length);
    let right_border = horizontal_line.repeat(right_border_length - 2);

    print!("\n{}{} {} {}{}", left_top_corner, left_border, title, right_border, right_top_corner);
    print!("{}", newline);
    for &message in messages {
        for line in wrap_text(message, max_message_length).iter() {
            let padding = " ".repeat(max_message_length - line.len());
            print!("{} {}{} {}", vertical_line, line, padding, vertical_line);
            print!("{}", newline);
        }
    }
    print!("{}{}{}", left_bottom_corner, horizontal_line.repeat(terminal_width - 2), right_bottom_corner);
    print!("{}", newline);
}

fn wrap_text(text: &str, max_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if !line.is_empty() && line.len() + word.len() + 1 > max_length {
                lines.push(line.clone());
                line.clear();
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    lines
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


