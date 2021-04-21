// use todo::Todo;
use chrono::{DateTime, Local};
use std::io;

fn main() {
    // let action = std::env::args().nth(1).expect("Please specify an action");
    // let item = std::env::args()
    //     .nth(2)
    //     .expect("Please specify an item to act on");

    // println!("{:?}, {:?}", action, item);

    // let mut todo = Todo::new().expect("db init failed");

    // if action == "add" {
    //     todo.insert(item);
    //     match todo.save() {
    //         Ok(_) => println!("saved succesfully"),
    //         Err(why) => println!("ERR @ {}", why),
    //     }
    // } else if action == "done" {
    //     match todo.complete(&item) {
    //         None => println!("'{}' item not found", item),
    //         Some(_) => match todo.save() {
    //             Ok(_) => println!("task updated"),
    //             Err(why) => println!("error: {}", why),
    //         },
    //     }
    // }

    let now: DateTime<Local> = Local::now();

    // testing datetime lib
    println!("datetime is: {}", now);

    let mut three_wins: Vec<&str> = vec![];

    println!("Document three wins you had yesterday.");
    println!("Win 1 => ");
    let mut input1 = String::new();
    match io::stdin().read_line(&mut input1) {
        Err(error) => println!("error: {}", error),
        Ok(n) => {
            println!("{} bytes read", n);
        }
    }

    three_wins.push(&input1);
    println!("Win 2 => ");

    let mut input2 = String::new();
    match io::stdin().read_line(&mut input2) {
        Err(error) => println!("error: {}", error),
        Ok(n) => {
            // placeholder for logger logic 
            println!("{} bytes read", n);
        }
    }

    three_wins.push(&input2);
    println!("Win 3 => ");

    let mut input3 = String::new();
    match io::stdin().read_line(&mut input3) {
        Err(error) => println!("error: {}", error),
        Ok(n) => {
            // placeholder for logger logic             
            println!("{} bytes read", n);
        }
    }

    three_wins.push(&input3);

    for win in three_wins.iter() {
        println!("{}", win);
    }
}
