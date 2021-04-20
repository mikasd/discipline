use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args()
        .nth(2)
        .expect("Please specify an item to act on");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("db init failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("saved succesfully"),
            Err(why) => println!("ERR @ {}", why),
        }
    } else if action == "done" {
        match todo.complete(&item) {
            None => println!("'{}' item not found", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("task updated"),
                Err(why) => println!("error: {}", why),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("err occured: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        //insert new item into map
        self.map.insert(key, true);
    }

    //owns Todo struct implmentation, this prevents posthumous inserts after save
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f,&self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
