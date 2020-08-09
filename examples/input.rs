use prompt::canvas::Canvas;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = prompt::Input2::new(false, Some("*"));
    println!("Echo");
    let mut output = std::io::stdout().into_raw_mode()?;

    let pos = output.cursor_pos()?;
    let mut canvas = Canvas::new(&mut output, (pos.0, pos.1 + 2));

    let echo = input.run(std::io::stdin(), &mut canvas).unwrap();
    canvas.clear();
    println!("ECHO '{}'", echo);
    Ok(())
}
