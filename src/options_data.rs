use std::{string::String, f32::consts::E};
use nom::{
    IResult,
    bytes::complete::{tag, take_while},
    character::complete::{space0, space1, newline},
};

#[derive(Debug)]
pub struct OptionsData<'a> {
    title: Option<String>, 
    rsi: Option<String>,    // restart input file or folder
    rso: Option<String>,    // restart output file or folder 
    output: &'a str,        // processor print file (text output)
    save: &'a str,          // file or csr folder for binary data
    redb: Option<String>,   // file/folder for Rel. Engr. database
    mline: isize,           // number of printed lines per page
    spelloff: bool,         // logic spell checker flag
    endchar: &'a str,       // custom in-line comment character
    model: Option<String>,  // model name
}

impl Default for OptionsData<'_> {
    fn default() -> Self {
        OptionsData {
            title: None,
            rsi: None,
            rso: None,
            output: "case0.out",
            save: "case0.sav",
            redb: None,
            mline: 10000,
            spelloff: false,
            endchar: "$",
            model: None,
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    // slice single line
    let (input, line) = take_while(|c: char| c != '\n')(input)?;
    let (input, _) = newline(input)?;
    // parse "variable = value"
    let (line, variable) = take_while(|c: char| c != '=')(line)?;
    let (value, _) = tag("=")(line)?;
    Ok((input, (variable.trim(), value.trim())))
}

fn parse_title(input: &str) -> IResult<&str, &str> {
    // the line starts with "TITLE"
    let (input, _) = tag("TITLE")(input)?;
    let (input, _) = space1(input)?;
    let (input, title_value) = take_while(|c: char| c != '\n')(input)?;
    let (input, _) = newline(input)?;
    Ok((input, title_value.trim()))
}

pub fn parse_options_data(input: &str) -> IResult<&str, OptionsData> {
    let (input, _) = tag("HEADER OPTIONS DATA")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = newline(input)?;

    let mut header = OptionsData::default();
    let mut remaining_input = input;

    while !remaining_input.trim().is_empty() {
        if let Ok((input, (variable, value))) = parse_line(remaining_input) {
            match variable {
                "MODEL" => header.model = Some(value.to_string()),
                "RSI" => header.rsi = Some(value.to_string()),
                "RSO" => header.rso = Some(value.to_string()),
                "OUTPUT" => header.output = value,
                "SAVE" => header.save = value,
                "REDB" => header.redb = Some(value.to_string()),
                "MLINE" => header.mline = value.parse::<isize>().unwrap(),
                "SPELLOFF" => header.spelloff = value.parse::<bool>().unwrap(),
                "ENDCHAR" => header.endchar = value,
                _ => break,
            }
            remaining_input = input;
        } else if let Ok((input, title_value)) = parse_title(remaining_input) {
            header.title = Some(title_value.to_string());
            remaining_input = input;
        } else {
            break;
        }
    }

    Ok((remaining_input, header))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_options_data() {
        let input = 
r#"HEADER OPTIONS DATA
    OUTPUT = BAR.OUT
    MODEL = TEST
TITLE HEATED BAR SAMPLE PROBLEM
    SAVE = BAR.SAV
"#;

        let (remaining_input, data) = parse_options_data(input).unwrap();
        println!("{:?}", data);
        assert_eq!(data.title.unwrap(), "HEATED BAR SAMPLE PROBLEM");
        assert_eq!(data.output, "BAR.OUT");
        assert_eq!(data.save, "BAR.SAV");
        assert_eq!(data.model.unwrap(), "TEST");
    }
}