use std::{collections::HashMap, hash::Hash};

fn main() {
    just_print_it("Hello");
    just_print_it(3);
    just_print_it(3.556);

    just_print_it_2("Using Where");

    let mut my_bucket = MyHashMapBucket::new();
    my_bucket.insert("weekday", "Monday");
    my_bucket.insert("weekday", "Tuesday");
    my_bucket.insert("weekday", "Wednesday");
    my_bucket.insert("weekday", "Thursday");

    my_bucket.insert("chillday", "Friday");

    my_bucket.insert("weekend", "Saturday");
    my_bucket.insert("weekend", "Sunday");

    println!("{my_bucket:?}")
}

fn just_print_it<T: ToString>(something: T) {
    println!("{}", something.to_string())
}

fn just_print_it_2<T>(something: T)
where
    T: ToString,
{
    println!("{}", something.to_string())
}

#[derive(Debug)]
struct MyHashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> MyHashMapBucket<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}
