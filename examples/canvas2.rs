use prompt::line::Canvas2;
use std::io::Write;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = prompt::Theme::new();
    {
        let mut stdout = std::io::stdout().into_raw_mode()?;
        // let (x, y) = stdout.cursor_pos()?;
        //println!("START {} {} \r", x, y);
        let mut c = Canvas2::new(&mut stdout).expect("canvas");

        {
            let mut line = c.create_line();
            write!(line, "Hello, World!");
        }
        {
            let mut line = c.create_line();
            write!(line, "Hello, World!");
        }
        c.render();
        // c.print("Jesper");
        std::thread::sleep_ms(500);

        c.line_mut(0).unwrap().clear().push("Mio mendo");

        //c.clear();
        c.render().unwrap();
        std::thread::sleep_ms(1000);
        c.line_mut(1).unwrap().clear().push("Mio Test0");
        c.render();
        //c.render_line(1).unwrap();
        std::thread::sleep_ms(1000);

        // c.print_lines(&["Hello", "World"])?.println("after")?;
        // ///println!("after");
        // std::thread::sleep_ms(1000);
        // c.clear()?;
        // //c.print("Hello")?;
        // c.print_lines(&["Hello 2", "World 2", "Test Mig"])?
        //     .println("after 2")?;
        // std::thread::sleep_ms(1000);
        // c.clear()?;
        // c.print_lines(&["Hello 3"]);
        // std::thread::sleep_ms(500);
    }
    println!("HEY");
    //c.print("Jesper");
    //std::thread::sleep_ms(2000);
    // c.ensure_lines();
    // //c.print_line("Hello!");
    // c.print_validation("Hello, World");
    // c.print_line("Hello!");
    // c.clear_lines();

    Ok(())
}
