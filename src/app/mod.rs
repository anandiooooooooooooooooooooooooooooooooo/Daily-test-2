use crate::domain::entities::Task;
use crate::domain::repository::TaskRepository;
use chrono::Utc;

pub fn handle_add<R: TaskRepository>(repo: &mut R, title: String, description: Option<String>) {
    let id = Utc::now().timestamp_millis() as u64;
    let task = Task::new(id, title, description);
    repo.add(task).expect("Failed to add task");
}

pub fn handle_list<R: TaskRepository>(repo: &R) {
    let tasks = repo.all().expect("Failed to fetch tasks");
    for task in tasks {
        println!("[{}] {} - {}", task.id, task.title, if task.completed { "Done" } else { "Pending" });
    }
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [Rg05AQNXQ3PT]
// @1751239674 [9XTopIHMTqs8]
// @1751241407 [o4JdBTLsaE3d]
// @1751241894 [MFbkB4IUo8qv]
// @1751253636 [2192HEUh653s]
