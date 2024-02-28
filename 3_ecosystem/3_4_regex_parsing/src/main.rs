#![feature(if_let_guard)]
#![feature(let_chains)]
use regex::RegexBuilder;
use pest_derive::Parser;

fn main() {
    println!("Implement me!");
}

#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

type ToGet = (Option<Sign>, Option<usize>, Option<Precision>);

const NOMATCH: ToGet = (None,None,None);

pub fn parse_hand_version(input: &str) -> ToGet {

    #[derive(Parser)]
    #[grammar = "grammars/fmt.pest"]
    struct Parser;

}

/// we take in: format_spec := [[fill]align][sign]['#']['0'][width]['.' precision]type
/// we give back: sign,width,precision
pub fn parse(input: &str) -> ToGet {

    const REGEX_SIGN: &str = r#"
        (.?[<^>])?(?P<sign>[+\-])?[#]?.*
    "#;
    
    const REGEX_WIDTH: &str = r#"
        [0]?(?P<width>[0-9]+)?.*
    "#;

    const REGEX_PRECISION: &str = r#"
        (\.(?P<precision>[0-9]+ | \* )  )?.*
    "#;

    let caps = |s: &str| {
        let the_regex = RegexBuilder::new(s).build().unwrap(); //should have this in lazy static

        let the_captures = the_regex.captures(input);

        the_captures
    };

    let sign = {
        let c = caps(REGEX_SIGN);

        c.and_then(
            |c| match c.name("sign").map(|m| m.as_str()).unwrap_or("") {
                "+" => Some(Sign::Plus),
                "-" => Some(Sign::Plus),
                _ => None
            }
        )
    };

    let width = {
        let c = caps(REGEX_WIDTH);

        c.and_then(
            |c| match c.name("width").map(|m| m.as_str()).unwrap_or("") {
                w if let Ok(width) = w.parse::<usize>() => Some(width),
                _ => unreachable!(),
            }
        )
    };

    let precision = {
        let c = caps(REGEX_PRECISION);

        c.and_then(
            |c| match c.name("precision").map(|m| m.as_str()).unwrap_or("") {
                i if let Ok(uint) = i.parse::<usize>() => Some(Precision::Integer(uint)),
                a if let Ok(arg) = a[1..].parse()
                    && (a.starts_with('$') || a.starts_with('.')) =>
                {
                    Some(Precision::Argument(arg))
                }
                "*" => Some(Precision::Asterisk),
                _ => None,
            }
        )
    };

    (sign,width,precision)

}

#[cfg(test)]
mod spec_hand {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse_hand_version(input);
            assert_eq!(expected, sign, "on: {}", input);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse_hand_version(input);
            assert_eq!(expected, width, "on: {}", input);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse_hand_version(input);
            assert_eq!(expected, precision, "on: {}", input);
        }
    }
}

#[cfg(test)]
mod spec_regex {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(expected, sign, "on: {}", input);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(expected, width, "on: {}", input);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(expected, precision, "on: {}", input);
        }
    }
}
