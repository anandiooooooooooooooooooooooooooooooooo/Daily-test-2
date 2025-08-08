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
// @1753641161 [7aqQWKxv8yVN]
// @1753650888 [tN6IkuYoB9hl]
// @1753663908 [OwXFzinf0RDq]
// @1753684652 [RoFeruOgGE9T]
// @1753694335 [wkEPixGwTTxk]
// @1753716179 [WpSc4WMXXMgP]
// @1753727830 [4y7TG2x6pgzt]
// @1753750373 [TlByEgwBOjFK]
// @1753761987 [kZBPqcSUWa7P]
// @1753770963 [mEW7c8s2Bqye]
// @1753780620 [W9LVNY5HYNJo]
// @1753802641 [9gcBuV4DKTZA]
// @1753823749 [Fprlc6lJ8HkN]
// @1753836470 [lcPCXanJ2FGr]
// @1753848216 [9qOozbKEAOKJ]
// @1753857430 [DwJSVcSfDlIk]
// @1753867112 [GSNQWidHXmec]
// @1753880219 [vmvIgKooUHbd]
// @1753888947 [Ow5SvpmaiPjf]
// @1753900586 [K3rJgKTrjteD]
// @1753910155 [dCe1tHegWX2B]
// @1753922854 [sL5TA0B8iaPc]
// @1753934558 [qU9Dpn1bwWas]
// @1753943757 [o158AqqE5OMA]
// @1753953468 [QI9UUf44WmW6]
// @1753975340 [HvIGQDvzoKkj]
// @1753986994 [2uJGmT8YMzCs]
// @1753996549 [vydAanEDlBQg]
// @1754030229 [xnHBbwMWNMlR]
// @1754039812 [eg73sWpLp8WG]
// @1754073342 [IlBwaQtEJeTs]
// @1754095490 [YVo9FZ0Kdc9A]
// @1754106914 [wyld8XOJob47]
// @1754116332 [668nZcurtqjP]
// @1754139067 [QilnjzbpczMZ]
// @1754147971 [MLqHe5XjbelU]
// @1754159609 [2hz6C0kX0Edv]
// @1754182424 [C6lUyq72GxBP]
// @1754212283 [ffLbu1f6pT9e]
// @1754225644 [ToRdfgYyNP78]
// @1754268798 [GkGlrAaBFUQB]
// @1754280866 [iazQmxtXPdq2]
// @1754289696 [JGbOBnpMjRZz]
// @1754321013 [loY27oUwHLNX]
// @1754332652 [zyYjXPjYNw6U]
// @1754354966 [VccsosANJxNf]
// @1754375852 [Y7qavAjwpWYh]
// @1754385513 [HLXZqrSBtyjt]
// @1754398697 [vcd4o1sSLPCx]
// @1754419126 [jrjUlrAqK7dH]
// @1754428575 [4qY86OhwVeTd]
// @1754441302 [qBbx8xT6ZVE1]
// @1754462258 [YbqMgfnwaCmy]
// @1754471936 [JkeJHHerXBd6]
// @1754493826 [mOFyBVI5JC05]
// @1754505359 [kxIWcsln6J4a]
// @1754548651 [fDXCHUGXr6dh]
// @1754571453 [9Lk4X6RkIr3W]
// @1754591881 [rq7M6FGAxRSV]
// @1754601270 [kE287z5kPFJW]
// @1754614104 [Ku6lyqvl1ZYy]
// @1754625849 [YqzGiqWuyWOf]
// @1754644661 [eKaOGUqD6HhB]
// @1754657790 [isC89JGH1NrP]
// @1754678027 [yQ5MlbEDiI8F]
