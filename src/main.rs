use ferris_says::say;
use std::{io::{BufWriter, stdout}};

fn main() {
    fn ferris_read_lines(lines: &[&str], delay : u64) {
        for line in lines {
            ferris_read_line(line, delay);
        }
    } 

    fn ferris_read_line(dialogtext: &str, delay : u64) {
        if delay > 0 {
            std::thread::sleep(std::time::Duration::from_secs(delay));
        }

        let stdout = stdout();
        let message = String::from(dialogtext);

        let width = message.chars().count();

        print!("\x1Bc");
        let mut writer = BufWriter::new(stdout.lock());
        say(&message, width, &mut writer).unwrap();
    }

    let array = &["one", "two", "three"];
    ferris_read_lines(array, 2);

    println!("End of reading");
}
