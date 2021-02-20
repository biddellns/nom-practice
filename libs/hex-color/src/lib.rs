use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    IResult,
    sequence::tuple,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq)] pub struct ColorHex {
    pub red: String,
    pub green: String,
    pub blue: String,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    return c.is_digit(16);
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex,
    )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn hex_number(input: &str) -> IResult<&str, ColorHex> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_raw, hex_raw, hex_raw))(input)?;

    Ok((input, ColorHex { red, green, blue }))
}

fn hex_raw(input: &str) -> IResult<&str, String> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        // absolutely gross but wanted to see if I could do a closure
        |val: &str| -> Result<String, std::string::ParseError> { Ok(val.to_string()) }
    )(input)
}

#[cfg(test)]
mod test {
    use crate::{Color, hex_color, hex_number, ColorHex};

    #[test]
    fn parse_color() {
        assert_eq!(hex_color("#2F14DF"), Ok(("", Color {
            red: 47,
            green: 20,
            blue: 223,
        })))
    }

    #[test]
    fn parse_color_hex_strings() {
        assert_eq!(hex_number("#2F14DF"), Ok(("", ColorHex{
            red: String::from("2F"),
            green: String::from("14"),
            blue: String::from("DF"),
        })))
    }
}
