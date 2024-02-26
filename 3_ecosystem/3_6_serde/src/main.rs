use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,PartialEq,Debug,Clone,Copy)]
#[serde(into = "usize")]
struct Price(usize);

impl From<Price> for usize {
    fn from(value: Price) -> Self {
        value.0
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Gift {
    id: usize,
    price: Price,
    description: String,
}

mod custom {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use std::time::Duration;

    #[derive(Debug, PartialEq)]
    pub struct DurationWrapper(pub Duration);

    impl<'de> Deserialize<'de> for DurationWrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let duration_str: String = Deserialize::deserialize(deserializer)?;
    
            parse_duration_string(duration_str)
                .map(DurationWrapper)
                .map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for DurationWrapper {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let total_seconds = self.0.as_secs();
            let milliseconds = self.0.subsec_millis();
            let nanoseconds = self.0.subsec_nanos() - milliseconds * 1_000_000;

            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;

            let mut res = String::with_capacity(200); // up to 1000 (excl) hours w\o relocate;
            // but no api to append and fmt in the same time =(

            if hours != 0 {
                res = format!("{}{}h",res,hours);
            };
            if minutes != 0 {
                res = format!("{}{}m",res,minutes);
            };
            if seconds != 0 {
                res = format!("{}{}s",res,seconds);
            };
            if milliseconds != 0 {
                res = format!("{}{}ms",res,milliseconds);
            };
            if nanoseconds != 0 {
                res = format!("{}{}ns",res,nanoseconds);
            };

            serializer.serialize_str(&res)
        }
    }


    fn parse_duration_string<S: AsRef<str>>(duration_str: S) -> Result<Duration, String> {
        let mut parsed_duration = Duration::default();
        let mut current_value = 0u64;


        let mut cs = duration_str.as_ref().chars().filter(|ch| !ch.is_whitespace()).peekable();
        loop {
            match cs.next() {
                Some(c) => {
                    if let Some(d) = c.to_digit(10) {
                        current_value = 10*current_value + d as u64;
                    } else {
                        match (c,cs.peek()) {
                            ('s',_) => parsed_duration += Duration::from_secs(current_value),
                            ('m',Some('s')) => {
                                parsed_duration += Duration::from_millis(current_value);
                                let _ = cs.next(); //ignore that 's'
                            },
                            ('m',_) => parsed_duration += Duration::from_secs(current_value * 60),
                            ('h',_) => parsed_duration += Duration::from_secs(current_value * 60 * 60),
                            _ => return Err("Invalid duration unit".into()),
                        }
                        // Reset current_value for the next numeric value
                        current_value = 0;
                    }
                },
                None => break
            }
        }

        // Check if there's a remaining numeric value at the end of the string
        if current_value != 0 {
            parsed_duration += Duration::from_secs(current_value);
        }

        Ok(parsed_duration)
    }

}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Debug {
    duration: custom::DurationWrapper,
    at: chrono::DateTime<chrono::Utc>,       // timestamp
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: usize,
    shard_url: url::Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct PublicTariff {
    id: usize,
    price: Price,
    duration: custom::DurationWrapper, //  ---
    description: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct PrivateTariff {
    client_price: usize,
    duration: custom::DurationWrapper, // ---
    description: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Request {
    #[serde(rename = "type")]
    kind: String,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() {
    let mut args = std::env::args().skip(1);

    let fpath = args.next().expect("Give path to json req");

    let req: Request = serde_json::from_reader(std::fs::File::open(fpath).expect("no such file"))
        .expect("failed to parse");

    println!("yaml:\n {}", serde_yaml::to_string(&req).unwrap());

    println!("toml:\n {}", toml::to_string(&req).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn read_the_req() -> Request {
        let fpath = "./request.json";
        serde_json::from_reader(std::fs::File::open(fpath).expect("no such file"))
            .expect("failed to parse")
    }

    #[test]
    fn test_toml() {
        let req = read_the_req();

        let toml = toml::to_string(&req).unwrap();

        let req2: Request = toml::from_str(&toml).unwrap();

        assert_eq!(req, req2);
    }

    #[test]
    fn test_yaml() {
        let req = read_the_req();

        let yaml = serde_yaml::to_string(&req).unwrap();

        let req2: Request = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(req, req2);
    }
}
