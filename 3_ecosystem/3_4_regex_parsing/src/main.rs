#![feature(if_let_guard)]
#![feature(let_chains)]
use regex::Regex;

fn main() {
    println!("Implement me!");
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}


/// we take in: format_spec := [[fill]align][sign]['#']['0'][width]['.' precision]type
/// we give back: sign,width,precision
fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {

    const REGEX2: &'static str = r"
        ^.?
        [<>^]?
        ([+-])?
        [#]?
        [0]?
        (\d+)?
        [(\.\d+|$\d+)]?
        [a-zA-Z]+
        /g
    ";

    let the_regex = Regex::new(REGEX2).unwrap(); //should have this in lazy static

    if let Some(captures) = the_regex.captures(input) {
        let sign = captures.get(1).map_or("", |m| m.as_str());
        let width = captures.get(2).map_or("", |m| m.as_str());
        let precision = captures.get(3).map_or("", |m| m.as_str());


        let sign = match sign {
            "+" => Some(Sign::Plus),
            "-" => Some(Sign::Plus),
            "" => None,
            _ => return (None,None,None)
        };
        let width = match width {
            w if let Ok(width) = w.parse::<usize>() => Some(width),
            "" => None,
            _ => return (None,None,None)
        };
        let precision = match precision {
            i if let Ok(uint) = i.parse::<usize>() => Some(Precision::Integer(uint)),
            a if let Ok(arg) = a[1..].parse() && (a.starts_with('$') || a.starts_with('.')) => Some(Precision::Argument(arg)),
            "*" => Some(Precision::Asterisk),
            "" => None,
            _ => return (None,None,None)
        };

        (sign,width,precision)
    } else {
        (None,None,None)
    }

}

#[cfg(test)]
mod spec {
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
            assert_eq!(sign, expected,"on: {}",input);
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
            assert_eq!(width, expected,"on: {}",input);
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
            assert_eq!(precision, expected,"on: {}",input);
        }
    }
}
