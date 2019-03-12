use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn apply_color(colored: bool, text: String, color: Color) {
    if colored {
        print_colored(color, text);
    } else {
        print_colored(Color::White, text);
    }
}

pub fn print_colored(color: Color, text: String) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}
