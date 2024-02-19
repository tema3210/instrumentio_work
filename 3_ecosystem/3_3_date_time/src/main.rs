use chrono::Datelike;

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

struct User(chrono::NaiveDate);

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        Self(chrono::NaiveDate::from_ymd_opt(year, month, day).expect("bad date"))
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let now: chrono::NaiveDate = NOW.parse().unwrap();

        let age = now.years_since(self.0).unwrap_or(0) as u16;
        // dbg!(self.0,now,age);

        age
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for (i,((y, m, d), expected)) in vec![
            ((2032, 6, 25), 0),
            ((2026, 6, 27), 0), // -_-
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ].into_iter().enumerate() {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected,"at {}", i );
        }
    }
}
