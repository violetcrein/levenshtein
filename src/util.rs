pub fn color(input: f64, levels: (f64, f64, f64)) -> String {
    if input <= levels.0 {
        return format!("\x1b[32;1m{}\x1b[31;0m", input)
    }
    else if input <= levels.1 {
        return format!("\x1b[33;1m{}\x1b[31;0m", input)
    }
    format!("\x1b[31;1m{}\x1b[31;0m", input)
}