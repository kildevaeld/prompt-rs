use prompt::canvas::Canvas;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = prompt::Theme::new();
    {
        let mut stdout = std::io::stdout().into_raw_mode()?;
        let (x, y) = stdout.cursor_pos()?;
        println!("START {} {}", x, y);
        let mut c = Canvas::new(&mut stdout, &theme, (10, y + 1));

        // c.print("Jesper");
        // std::thread::sleep_ms(500);
        c.print_lines(&["Hello", "World"])?.println("after")?;
        ///println!("after");
        std::thread::sleep_ms(1000);
        c.clear()?;
        //c.print("Hello")?;
        c.print_lines(&["Hello 2", "World 2", "Test Mig"])?
            .println("after 2")?;
        std::thread::sleep_ms(1000);
        c.clear()?;
        c.print_lines(&["Hello 3"]);
        std::thread::sleep_ms(500);
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
