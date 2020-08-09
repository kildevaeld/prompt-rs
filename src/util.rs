pub fn match_char(data: &char) -> bool {
    match *data {
        '\x01'..='\x08' | '\u{10FFFE}'..='\u{10FFFF}' => true,
        _ => false,
    }
}
