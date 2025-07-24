use std::collections::HashMap;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
extern crate chrono;

use chrono::{Local, NaiveDate, NaiveTime, TimeZone};

struct TodoItem {
    name: String,
    due_time: Instant,
}

impl TodoItem {
    fn new(name: &str, due_time: Instant) -> TodoItem {
        TodoItem {
            name: String::from(name),
            due_time,
        }
    }

    fn time_until_due(&self) -> Duration {
        self.due_time.saturating_duration_since(Instant::now())
    }

    fn is_due_soon(&self) -> bool {
        let one_hour = Duration::from_secs(60 * 60);
        let zero_duration = Duration::from_secs(0);
        let time_until_due = self.time_until_due();
        time_until_due <= one_hour && time_until_due > zero_duration
    }

    fn is_overdue(&self) -> bool {
        self.time_until_due() <= Duration::from_secs(0)
    }
}

struct TodoList {
    list: Arc<Mutex<HashMap<String, TodoItem>>>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_item(&self, name: &str, due_time: Instant) {
        let mut list = self.list.lock().unwrap();
        let item = TodoItem::new(name, due_time);
        list.insert(name.to_string(), item);
    }

    fn remove_item(&self, name: &str) {
        let mut list = self.list.lock().unwrap();
        list.remove(name);
    }

    fn get_items(&self) -> Vec<String> {
        let list = self.list.lock().unwrap();
        let mut items = Vec::new();

        for (name, item) in list.iter() {
            let status = if item.is_overdue() {
                "OVERDUE"
            } else if item.is_due_soon() {
                "DUE SOON"
            } else {
                ""
            };

            let time_until_due = item.time_until_due();
            let minutes_until_due = time_until_due.as_secs() / 60;

            let message = format!(
                "{}{} (due in {} minutes)",
                name,
                if status.is_empty() { "" } else { " " },
                minutes_until_due
            );

            items.push(message);
        }

        items
    }

    fn check_due_items(&self) {
        loop {
            let items = self.get_items();
            let due_items: Vec<&str> = items
                .iter()
                .filter(|item| item.contains("DUE SOON"))
                .map(|item| item.as_str())
                .collect();

            for item in due_items {
                println!("Due soon: {}", item);
            }

            thread::sleep(Duration::from_secs(5 * 60));
        }
    }

    fn set_item_time(&self, name: &str, due_time: Instant) -> Result<(), String> {
        let mut list = self.list.lock().unwrap();
        if let Some(item) = list.get_mut(name) {
            item.due_time = due_time;
            Ok(())
        } else {
            Err("Item not found.".to_string())
        }
    }
}

fn main() {
    let todo_list = Arc::new(Mutex::new(TodoList::new()));
    let todo_list_clone = todo_list.clone();

    let due_items_thread = thread::spawn(move || {
        todo_list_clone.lock().unwrap().check_due_items();
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.get(0).unwrap_or(&"") == &"add" {
            if tokens.len() < 3 {
                println!("Invalid command. Usage: add <name> <date> <time>");
                continue;
            }

            let name = tokens[1];
            let date_string = tokens[2];
            let time_string = tokens.get(3).unwrap_or(&"00:00");

            let date = match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    println!("Invalid date format. Please use YYYY-MM-DD.");
                    continue;
                }
            };

            let time = match NaiveTime::parse_from_str(&time_string, "%H:%M") {
                Ok(time) => time,
                Err(_) => {
                    println!("Invalid time format. Please use HH:MM.");
                    continue;
                }
            };

            let naive = date.and_time(time).expect("Invalid date/time");
            let datetime = Local.from_local_datetime(&naive).unwrap().with_timezone(&chrono::Utc);

            let duration = datetime.signed_duration_since(chrono::Utc::now()).to_std().unwrap_or(Duration::from_secs(0));
            let due_time = Instant::now() + duration;

            todo_list.lock().unwrap().add_item(name, due_time);
            println!("Item added.");
        }

        if tokens.get(0).unwrap_or(&"") == &"remove" {
            if tokens.len() < 2 {
                println!("Invalid command. Usage: remove <name>");
                continue;
            }

            let name = tokens[1];
            todo_list.lock().unwrap().remove_item(name);
            println!("Item removed.");
        }

        if tokens.get(0).unwrap_or(&"") == &"list" {
            let items = todo_list.lock().unwrap().get_items();
            if items.is_empty() {
                println!("No items.");
            } else {
                for item in items {
                    println!("{}", item);
                }
            }
        }

        if tokens.get(0).unwrap_or(&"") == &"settime" {
            if tokens.len() < 3 {
                println!("Invalid command. Usage: settime <name> <HH:MM>");
                continue;
            }

            let name = tokens[1];
            let time_string = tokens[2];

            let time = match NaiveTime::parse_from_str(&time_string, "%H:%M") {
                Ok(time) => time,
                Err(_) => {
                    println!("Invalid time format. Please use HH:MM.");
                    continue;
                }
            };

            let now = Local::now();
            let naive = now.date_naive().and_time(time).expect("Invalid time");
            let datetime = Local.from_local_datetime(&naive).unwrap().with_timezone(&chrono::Utc);
            let duration = datetime.signed_duration_since(chrono::Utc::now()).to_std().unwrap_or(Duration::from_secs(0));
            let due_time = Instant::now() + duration;

            match todo_list.lock().unwrap().set_item_time(name, due_time) {
                Ok(_) => println!("Due time updated."),
                Err(e) => println!("{}", e),
            }
        }

        if tokens.get(0).unwrap_or(&"") == &"exit" {
            break;
        }

        if !["add", "remove", "list", "settime", "exit"].contains(&tokens.get(0).unwrap_or(&"")) {
            println!("Invalid command.");
        }
    }

    println!("Exiting program...");
    due_items_thread.join().unwrap();
}
