use std::collections::HashMap;

fn main() {
    let mut hash_map: HashMap<String, i32> = HashMap::new();

    hash_map.insert("player1".to_string(), 0);
    hash_map.insert("player2".to_string(), 0);

    for (k, y) in &hash_map {
        println!("key: {k}\tvalue: {y}");
    }
    println!("{hash_map:?}");

    hash_map.insert("player1".to_string(), 1);

    let score = hash_map.get(&String::from("player1")).cloned().unwrap_or(0);
    println!("score: {score}");

    hash_map.entry("player1".to_string()).or_insert(42);
    hash_map.entry("player3".to_string()).or_insert(42);
    println!("{hash_map:?}");

    for key in [
        "player1".to_string(),
        "player2".to_string(),
        "player3".to_string(),
    ] {
        let value: &mut i32 = hash_map.entry(key).or_insert(0);
        *value += 1;
    }

    println!("{hash_map:?}");
}
