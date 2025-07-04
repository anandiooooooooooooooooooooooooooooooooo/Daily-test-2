use crate::app::{handle_add, handle_list};
use crate::infrastructure::fs_storage::FileTaskRepo;

pub fn run_cli() {
    let args: Vec<String> = std::env::args().collect();
    let mut repo = FileTaskRepo::new("tasks.json".to_string());
    repo.load_from_file().unwrap();

    match args.get(1).map(String::as_str) {
        Some("add") => {
            let title = args.get(2).cloned().unwrap_or_else(|| "Untitled".to_string());
            let desc = args.get(3).cloned();
            handle_add(&mut repo, title, desc);
        }
        Some("list") => handle_list(&repo),
        _ => println!("Available commands: add <title> [desc], list"),
    }
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [jwleHZsGkjVm]
// @1751239674 [cZ6gQsSQBD8p]
// @1751241407 [Fh5B4gBZ8Cnr]
// @1751241894 [ojfI1A0hIN6M]
// @1751253636 [6VWFfRnWdDV3]
// @1751340332 [DKJLbYGJ0bfq]
// @1751598844 [4u6giRkePkWR]
