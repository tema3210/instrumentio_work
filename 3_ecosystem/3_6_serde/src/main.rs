use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,PartialEq,Debug)]
struct Gift {
    id: usize,
    price: usize,
    description: String
}

#[derive(Deserialize,Serialize,PartialEq,Debug)]
struct Debug {
    duration: String, //should really be a Duration
    at: String // timestamp
}

#[derive(Deserialize,Serialize,PartialEq,Debug)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: usize,
    shard_url: url::Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Deserialize,Serialize,PartialEq,Debug)]
struct PublicTariff {
    id: usize,
    price: usize,
    duration: String, //  ---
    description: String
}

#[derive(Deserialize,Serialize,PartialEq,Debug)]
struct PrivateTariff {
    client_price: usize,
    duration: String, // ---
    description: String
}

#[derive(Deserialize,Serialize,PartialEq,Debug)]
struct Request {
    r#type: String,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug
}


fn main() {
    let mut args = std::env::args().skip(1);

    let fpath = args.next().expect("Give path to json req");

    let req: Request = serde_json::from_reader(std::fs::File::open(fpath).expect("no such file")).expect("failed to parse");

    println!("yaml:\n {}", serde_yaml::to_string(&req).unwrap());

    println!("toml:\n {}", toml::to_string(&req).unwrap());
}


#[cfg(test)]
mod tests {
    use crate::*;

    fn read_the_req() -> Request {
        let fpath = "./request.json";
        serde_json::from_reader(std::fs::File::open(fpath).expect("no such file")).expect("failed to parse")
    }

    #[test]
    fn test_toml() {
        let req = read_the_req();

        let toml = toml::to_string(&req).unwrap();

        let req2: Request = toml::from_str(&toml).unwrap();

        assert_eq!(req,req2);
    }

    #[test]
    fn test_yaml() {
        let req = read_the_req();

        let yaml = serde_yaml::to_string(&req).unwrap();

        let req2: Request = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(req,req2);
    }

}