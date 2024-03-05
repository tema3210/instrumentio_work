#![allow(clippy::never_loop)]
#![feature(if_let_guard)]
#![feature(let_chains)]
use pest::Parser;
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
    ArgumentStr(String),
    Asterisk,
}

type ToGet = (Option<Sign>, Option<usize>, Option<Precision>);

const NOMATCH: ToGet = (None,None,None);

#[derive(Parser)]
#[grammar = "grammars/fmt.pest"]
struct FmtParser;

pub fn parse_hand_version(input: &str) -> ToGet {
    match FmtParser::parse(Rule::format_spec,input) {
        Ok(pairs) => {
            let mut sign = None;
            let mut width = None;
            let mut precision = None;

            let pair = pairs.take(1).last().unwrap(); //this one doesn't crash because pairs have at least one item inside

            for sub in pair.into_inner() {
                match sub.as_rule() {
                    Rule::sign => {
                        match sub.as_str() {
                            "+" => sign = Some(Sign::Plus),
                            "-" => sign = Some(Sign::Minus),
                            _ => continue
                        }
                    },
                    Rule::width => {
                        match sub.as_str() {
                            i if let Ok(i) = i.parse::<usize>() => {
                                width = Some(i);
                            },
                            // we are not supposed to parse parameters here
                            _ => continue
                        }
                    },
                    Rule::precision => {
                        match sub.as_str() {
                            a if let Ok(arg) = a[1..].parse()
                                && a.starts_with('$') =>
                            {
                                println!("the precision: {}", sub.as_str());
                                precision = Some(Precision::Argument(arg))
                            },
                            a if a.starts_with('$') => {
                                precision = Some(Precision::ArgumentStr(a[1..].into()))
                            },
                            i if let Ok(uint) = i.parse::<usize>() => precision = Some(Precision::Integer(uint)),
                            "*" => {
                                precision = Some(Precision::Asterisk)
                            },
                            _ => unreachable!("This")
                        }
                    },
                    _ => continue,
                }
            };
            (sign,width,precision) 
        },
        Err(_) => NOMATCH,
    }
}

const REGEX_SIGN: &str = r#"([\+-])"#;
    
const REGEX_WIDTH: &str = r#"([0-9]+)"#;

const REGEX_PRECISION: &str = r#"(\.([0-9]+|\*|\$\w))"#;

fn matcher(regex: &str) -> impl Fn(&str,usize) -> Option<(&str,usize)> {
    let the_regex = RegexBuilder::new(regex).build().unwrap(); //should have this in lazy static

    move |input,key: usize| {
        let captures = the_regex.captures(input)?;

        let res = captures.get(key).map(|m| m.as_str());

        Some((res?,captures.len()))
    }
}

/// We take in: format_spec := [[fill]align][sign]['#']['0'][width]['.' precision]type
/// We give back: sign,width,precision
pub fn parse(input: &str) -> ToGet {

    let sign = {
        let c = matcher(REGEX_SIGN);

        c(input,1).and_then(
            |(c,_)| match c {
                "+" => Some(Sign::Plus),
                "-" => Some(Sign::Minus),
                _ => None
            }
        )
    };

    let width_matcher = matcher(REGEX_WIDTH);

    let precision_matcher = matcher(REGEX_PRECISION);


    let width = width_matcher(input,1).and_then(
        |(c,_)| match c {
            w if let Ok(width) = w.parse::<usize>() => {
                Some(width)
            },
            "" => None,
            _ => unreachable!(),
        }
    );

    let precision = precision_matcher(input,2).and_then(
        |(c,_)| match c {
            "*" => Some(Precision::Asterisk),
            i if let Ok(uint) = i.parse::<usize>() => {
                // what if we have two integers...
                Some(Precision::Integer(uint))
            },
            a if a.starts_with('$') && let Ok(arg) = a[1..].parse() => {
                Some(Precision::Argument(arg))
            },
            a if a.starts_with('$') => {
                Some(Precision::ArgumentStr(a[1..].into()))
            },
            _ => None,
        }
    );

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
            // used to be "-.1$x" but according to grammar that's illegal
            ("-.$x", Some(Precision::ArgumentStr("x".into()))),
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
            // used to be "-.1$x" but according to grammar that's illegal
            ("-.$x", Some(Sign::Minus)),
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
            // used to be "-.1$x" but according to grammar that's illegal
            ("-.$x", None),
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
            // used to be "-.1$x" but according to grammar that's illegal
            ("-.$x", Some(Precision::ArgumentStr("x".into()))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(expected, precision, "on: {}", input);
        }
    }
}
