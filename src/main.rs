use std::io::stdin;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut name = String::from("Rust");
    println!("Welcome to Rust! https://www.rust-lang.org");
    println!("Type \".help\" for more information");
    loop {
        print_c!("", Color::White);
        eprint!("{} > ", name);
        let mut x = String::new();
        stdin().read_line(&mut x).unwrap();

        match &x.trim().to_lowercase() as &str {
            ".help" => {
                println!();
                print_c!("---Command---", Color::Green);
                print_c!(".help       Print this help Message", Color::Cyan);
                print_c!(".exit       Exit the REPL", Color::Cyan);
                print_c!(
                    ".editor     Enter editor mode (cannot be modified)",
                    Color::Cyan
                );
                print_c!(".name       Set Name", Color::Cyan);
                print_c!("---Comment---", Color::Green);
                print_c!("// exit     Exit editor mode", Color::Cyan);
                continue;
            }
            ".exit" => std::process::exit(0),
            ".name" => {
                let mut n = String::new();
                print_c!("", Color::Cyan);
                eprint!("Enter Name: ");
                stdin().read_line(&mut n).unwrap();
                name = n.trim().parse().unwrap();
                print_c!("", Color::White);
            },
            ".editor" => {
                let mut code = String::new();

                'editorloop: loop {
                    let mut editor_read = String::new();
                    stdin().read_line(&mut editor_read).unwrap();
                    match &editor_read.trim().to_lowercase() as &str {
                        "// exit" | "//exit" => break 'editorloop,
                        _ => code.push_str(&editor_read as &str),
                    }
                }
                run(code.as_str());
            }
            _ => {
                run(x.as_str());
            }
        }
    }
}

#[macro_export]
macro_rules! print_c {
    ($t:expr,$color:expr) => {{
        print_color_e($t, $color, (false, false, false));
    }};
    ($t:expr,$color:expr,$tx:expr) => {{
        print_color_e($t, $color, $tx);
    }};
}

#[allow(unused_must_use)]
fn print_color_e(t: &str, color: Color, tx: (bool, bool, bool)) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(color))
            .set_bold(tx.0)
            .set_italic(tx.1)
            .set_underline(tx.2),
    );
    writeln!(&mut stdout, "{}", t);
    stdout.reset();
}

fn run(code: &str) {
    print_c!(
        match rsrun::eval(code) {
            Ok(e) => e,
            Err(e) => {
                print_c!(format!("{}", e).as_str(), Color::Rgb(255, 77, 77));
                "".to_string()
            }
        }
        .as_str(),
        Color::Yellow
    );
}
