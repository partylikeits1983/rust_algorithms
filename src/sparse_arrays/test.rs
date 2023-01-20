use std::collections::HashMap;

// type inference lets us omit an explicit type signature (which
// would be `HashMap<&str, u8>` in this example).

fn main() {

    let mut map = HashMap::new();
    let mut map_counter = HashMap::new();

    map.insert(1, "a");
    map_counter.insert(1, 1);

    assert_eq!(map.get(&1), Some(&"a")); // key exists
    assert_eq!(map.get(&2), None);

    let key: i32 = 1;
    let val = "a";

    if map.get(&key) == Some(&val) {
        println!("true");

        let value = map_counter[&key] + 1;
        println!("number: {}", value);

        map_counter.insert(key, value);
    };


    map.insert(1, "a");

    for (key, value) in &map {
        println!("{} {}", key, value);
    }

    println!("{}", map[&1]);
}


