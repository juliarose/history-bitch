
type RGB = (u8, u8, u8);

fn parse_hex_color(color_str: String) -> Result<RGB8, hex::FromHexError> {
    if color_str.len() != 6 {
        return Err(hex::FromHexError::InvalidStringLength);
    }
    hex::decode(&color_str).map(
        |bytes| RGB8::new(bytes[0], bytes[1], bytes[2])
    )
}
