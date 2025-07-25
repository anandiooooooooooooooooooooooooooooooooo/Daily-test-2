use crate::domain::entities::Task;
use crate::domain::repository::TaskRepository;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;

pub struct FileTaskRepo {
    pub path: String,
    pub cache: HashMap<u64, Task>,
}

impl FileTaskRepo {
    pub fn new(path: String) -> Self {
        Self {
            path,
            cache: HashMap::new(),
        }
    }

    fn sync_to_file(&self) -> Result<(), Box<dyn Error>> {
        let tasks: Vec<&Task> = self.cache.values().collect();
        let data = serde_json::to_string_pretty(&tasks)?;
        fs::write(&self.path, data)?;
        Ok(())
    }

    fn load_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        if Path::new(&self.path).exists() {
            let data = fs::read_to_string(&self.path)?;
            let tasks: Vec<Task> = serde_json::from_str(&data)?;
            self.cache = tasks.into_iter().map(|t| (t.id, t)).collect();
        }
        Ok(())
    }
}

impl TaskRepository for FileTaskRepo {
    fn all(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        Ok(self.cache.values().cloned().collect())
    }

    fn add(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        self.cache.insert(task.id, task);
        self.sync_to_file()
    }

    fn update(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        self.cache.insert(task.id, task);
        self.sync_to_file()
    }

    fn remove(&mut self, task_id: u64) -> Result<(), Box<dyn Error>> {
        self.cache.remove(&task_id);
        self.sync_to_file()
    }

    fn find_by_id(&self, task_id: u64) -> Result<Option<Task>, Box<dyn Error>> {
        Ok(self.cache.get(&task_id).cloned())
    }
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [QvdL55PdWe9l]
// @1751239674 [IFG2Ku3Z6FBC]
// @1751241408 [lCxyD07aLwBD]
// @1751241894 [yHY66gMnujWH]
// @1751253636 [d8veb69Y29ZG]
// @1751340332 [v74zXmpIq8tW]
// @1751598844 [r7evmvd7TdbR]
// @1752031075 [NvpGceZvvq8U]
// @1752117454 [WMUQwnrIvNsE]
// @1752204184 [o8rjMdTDShqs]
// @1752722590 [zzZYv86GXiow]
// @1752809081 [G8Zz284IJL4t]
// @1752895106 [laWfDdGDLAPg]
// @1753069183 [YH3WzMRIKeg3]
// @1753154678 [ywAgYklDN2Jt]
// @1753241182 [ksV4gxjj3QJD]
// @1753327480 [sjSShKzABvMe]
// @1753413881 [BMvEgumAnDQu]
// @1753500095 [NwxTy7dj057U]
