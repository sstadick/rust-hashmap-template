extern crate hashmap;
use hashmap::HashMap;

fn random_stat_buff() -> u8 {
    42
}

// type inference lets us omit an explicit type signature (which would be `HashMap<&str, u8>` in
// this example)
fn main() {
    let mut player_stats = HashMap::new();

    // insert a key only if it doesn't exist
    player_stats.entry("health").or_insert(100);

    // insert a key using a function that privides a new value only if it doesn't already exist
    player_stats.entry("defence").or_insert_with(random_stat_buff);

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();
}
