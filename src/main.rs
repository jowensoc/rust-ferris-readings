use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {

    fn set_dialog(dialogtext: &str, delay : u64) {
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

    set_dialog("One", 1);
    set_dialog("Two", 1);
    set_dialog("Three", 1);

    println!("END_PROGRAM");
}
