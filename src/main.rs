use std::collections::HashMap;



fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item to act on");

    println!("{:?}, {:?}", action, item);
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        //insert new item into map
        self.map.insert(key, true);
    }
}

