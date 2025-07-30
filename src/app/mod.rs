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
// @1751340332 [z4BAUpwa5L4E]
// @1751598844 [AzQOnquO2LzA]
// @1752031075 [COi9iScuSwRt]
// @1752117454 [PyeFkPQbJaet]
// @1752204184 [45OtrYzEWQgE]
// @1752722589 [DlDsNx6hPS6Q]
// @1752809081 [pOSxj1F1EF2R]
// @1752895106 [6LQPeSnjuct6]
// @1753069183 [3hpI1yXNZzKz]
// @1753154678 [KSowGoVDvBQC]
// @1753241182 [aIbhoHFNSrxl]
// @1753327480 [z6v1UVWIUnlR]
// @1753413881 [IHUB9gp9gNkv]
// @1753500095 [Q5ox7JPgn1Ca]
// @1753641161 [GzsJVL77uOTU]
// @1753650888 [qWXVflzVOmaw]
// @1753663908 [GJf9WoARV5Jt]
// @1753684652 [k4AH4AeCpppn]
// @1753694335 [Hm346xu4yMnA]
// @1753716179 [lmupw9aBDBq3]
// @1753727830 [50qC00BMHlSh]
// @1753750373 [MyQdWCSShi0X]
// @1753761987 [hDCFnNd87oIE]
// @1753770963 [wkSxVGPYBpg9]
// @1753780620 [D8YUT9k0XbC3]
// @1753802641 [hmkVivIx29Ig]
// @1753823749 [e3WySshajZ8o]
// @1753836470 [kETtgdNuyGKJ]
// @1753848215 [xMCYxS7JC6GW]
// @1753857430 [g3AwaHOLQuZb]
// @1753867112 [bgswzkRPyW1f]
