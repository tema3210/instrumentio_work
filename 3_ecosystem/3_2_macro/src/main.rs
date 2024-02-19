macro_rules! btreemap {
    ($( $key:expr => $val:expr ),*) => {
        {
            let mut btreemap = std::collections::BTreeMap::new();
            $(
                btreemap.insert($key,$val);
            )*
            btreemap
        }
    };
}

fn main() {
    let map1 = btreemap!(
        "k1" => "v1",
        "k2" => "v2"
    );

    let map2 = proc_crate::btreemap!(
        "k1" => "v1",
        "k2" => "v2"
    );

    println!("map1 {:?} and map2 {:?}", &map1, &map2);
}
