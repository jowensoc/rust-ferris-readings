use ferris_says::say;
use std::{io::{BufWriter, stdout}};
use std::fs;

fn main() {
    fn ferris_read_lines(lines: &Vec<&str>, duration : u64) {
        for line in lines {
            ferris_read_line(line, duration);
        }
    } 

    fn ferris_read_line(dialogtext: &str, duration : u64) {
        if duration > 0 {
            std::thread::sleep(std::time::Duration::from_secs(duration));
        }

        let stdout = stdout();
        let message = String::from(dialogtext);

        let width = message.chars().count();

        print!("\x1Bc");
        let mut writer = BufWriter::new(stdout.lock());
        say(&message, width, &mut writer).unwrap();
    }

    fn parse_file(file_path: &str) -> String {
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");  
        return contents;
    }

    let contents = parse_file("rhyme.txt");

    let lines = contents.lines().collect::<Vec<&str>>();
    ferris_read_lines(&lines, 2);
}
