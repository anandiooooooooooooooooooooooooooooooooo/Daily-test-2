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
// @1754711383 [o3b0qXSTvHFf]
// @1754743815 [Z4oCuyAe5P6t]
// @1754764354 [lYrIc8qxu5NN]
// @1754774045 [DlwzWkisp9Tt]
// @1754817086 [pNtVEXjJ2Ghu]
// @1754830247 [LVHqkECv7yXZ]
// @1754873414 [WNGcSuPicwLZ]
// @1754885038 [wm5HOTY5Tuee]
// @1754894225 [tC9wSWdcXKA0]
// @1754903848 [EvfB0N4hmcZi]
// @1754925728 [ePM7zq3SFtSO]
// @1754937382 [GLtxSNMeeyIW]
// @1754980461 [FRAqsVd5zLbn]
// @1754990030 [Y0w5Rall95MV]
// @1755003185 [fubjVoUn0zF2]
// @1755012089 [cYq9PYFR3ct5]
// @1755023715 [shRipBqregd7]
// @1755033303 [msSePYvLrYyM]
// @1755045872 [SMdmtMa7qH1C]
// @1755057015 [r3ahuHQd5zNA]
// @1755066857 [Zk45tVgihfzi]
// @1755076444 [OLbXOPF0r7Sw]
// @1755098282 [7vbvmDcGhhrc]
// @1755110006 [yxPGquwKt1hN]
// @1755119675 [Hza6Ve2ZvvIX]
// @1755132255 [z2tsYHJvCvR1]
// @1755143450 [nlmKEYUuFzzJ]
// @1755162865 [LIyqP7aB1gzy]
// @1755176087 [idT3n4LNlUH2]
// @1755184854 [1Z5WQoGCbJ69]
// @1755196488 [zo2IWzBmIUOo]
// @1755229873 [ok6ji2JsD0Dq]
// @1755239637 [53g6cnc9A2jM]
// @1755282848 [elBhrvFHSRWW]
// @1755315880 [y3WAzwuUKOUL]
// @1755335463 [QY0FEae8WONZ]
// @1755348526 [SxlIo181o4Yi]
// @1755369012 [jgB1L9lZUfzX]
// @1755378829 [tuCjOrmb7u1m]
// @1755391753 [pyFyGvLgPzvd]
// @1755421841 [EC0EQLdkCIJG]
// @1755455502 [ujnt74TOT1WT]
// @1755478136 [tcraquY06rAf]
// @1755489596 [suGFPXnf0mtx]
// @1755508586 [3sTXJfDwdXIg]
// @1755530513 [Hg3k8tGpfv8W]
// @1755551610 [wHdEFF6OWKXf]
// @1755574859 [5p4sbqiEGUID]
// @1755607745 [WVvCW3Xy83RK]
// @1755661173 [1trHCGKntZHu]
// @1755703230 [WOT3MR8LCLp6]
// @1755724443 [usENeRjTBBis]
// @1755758470 [RwaeQy0njeTn]
// @1755767511 [Xs1nKAN0JqAW]
// @1755810825 [DDB1dU1uMJbQ]
// @1755823294 [JDnUBAsG9nAQ]
// @1755853915 [uMGmpxOmw39b]
// @1755866962 [UUNHZ9TMYCxE]
// @1755875937 [54ag2wc90jzR]
// @1755897205 [mllIzGXxATrW]
// @1755909584 [X9svlzxVoru0]
// @1755920045 [FnX8hTXYv8wW]
// @1755953104 [y3lGYh5FrGm0]
// @1755962188 [3IRwkI48GYLo]
// @1755973723 [pIS9sPGetsZG]
// @1755996415 [BbdjEsdvyOEW]
// @1756007208 [J1GNnLsaRex0]
// @1756039550 [Iw07dtkdnI4B]
// @1756082608 [LhZVhvm7Hvyc]
// @1756093578 [vkknooX9EuoN]
// @1756126304 [MqMUKm3KCIvU]
// @1756135209 [E7VpBH9R7ebi]
// @1756146786 [zy14l3Jitisy]
// @1756156428 [X6XK8mP3ChCm]
// @1756190002 [HJ97uUeQqCWy]
// @1756199529 [m4MnqLU28vlQ]
// @1756212786 [dUYh0TrlyF92]
// @1756242797 [URzPLPfMmcMx]
// @1756255252 [AXZh4tIYQTeR]
// @1756276245 [N1Yr1yLlEtm8]
// @1756285891 [3PHlwUGdt4oh]
// @1756298969 [wmU2EVo6o98E]
// @1756307960 [PSmcGs02pxsD]
// @1756319421 [MAzpqeTPxzPl]
// @1756329229 [TXTD26yeTk6O]
// @1756341616 [etbYafPQZcOv]
// @1756362719 [TjTm2gj2nN8U]
// @1756372294 [pXuLZM9H08Rs]
// @1756385363 [lcGx3mcCKwSo]
// @1756394355 [bj98Zc2VjfOj]
// @1756405895 [xUnjKJAskqcj]
// @1756415608 [ST4aGZ0MgbSs]
// @1756428036 [C4ypPNXNmCH0]
// @1756471710 [ez81DRWRe7jf]
// @1756480587 [kEog0odAjZ9S]
// @1756501983 [wNZGSrwV5bBl]
// @1756514292 [nmGixoFn8FNj]
// @1756524508 [3W2akFLy1FoJ]
// @1756535251 [bBiDqiizAmF5]
// @1756566973 [xZkn9TXViH0z]
// @1756588342 [uvStmE5RsgKp]
// @1756601042 [mqbt97qI1NBD]
// @1756621733 [HsHDLpgAkKX4]
// @1756644212 [6tsO0YB7BP5R]
// @1756698614 [fcWs0QVwjIwX]
// @1756708468 [Zyx8as46pGBy]
// @1756718019 [5BiTWSAtKc1N]
// @1756731061 [QchzwkYV8llH]
// @1756751403 [ihftuhKtpt3B]
// @1756761156 [VBC7sk4Gls9C]
// @1756773656 [HIWbWyu0BELr]
// @1756784258 [zr3S16zpMjPM]
// @1756817440 [WVsgK8DX3cby]
// @1756826377 [FE21PVXCK3cM]
// @1756847562 [GXRgzs3vMD0w]
// @1756859901 [FVQilwz12pmH]
// @1756869977 [dFSB3hYVPYgO]
// @1756880981 [KszN0JLiqtd4]
// @1756890653 [y5cENepRccYO]
// @1756903690 [Iro65NGrQFFA]
// @1756912776 [7XDWudOMssAE]
// @1756924223 [1fv0FFIbLgEf]
// @1756933975 [NP0vtWeTFy6H]
// @1756967414 [wpR2FN6ZIPS2]
// @1756977085 [AUIYEBQkb9OX]
// @1756999130 [6AxxagS40Zss]
// @1757010627 [4L20c057LVnO]
// @1757053839 [Ddd0xfIkC8MR]
// @1757096917 [kUKlZWu47WrI]
// @1757106791 [EdiGgOKZl71f]
// @1757129121 [CXozjUVyeBqO]
// @1757140026 [jwQyZdndrSD8]
// @1757162448 [jF5qbWdubJcQ]
// @1757183195 [AVqPErqWVGP2]
// @1757193100 [8gFM8iIyxrMc]
// @1757226469 [v2VrP6VcZUfY]
// @1757236175 [SmwUsHePuOEO]
// @1757248921 [JXoErqdvjC1N]
// @1757313143 [Y6GUR22nCaLb]
// @1757322808 [ihPK0Qc0Sy4t]
// @1757344685 [rSv7vmBpHbex]
// @1757356318 [uMOiX1H2v6HF]
// @1757366000 [C55cYA98n1PJ]
// @1757378413 [KIk3541qgMxZ]
// @1757409120 [m21HAXShDXmD]
// @1757422297 [KcJlsNXI8oL0]
// @1757431216 [WS42BuQ6NzPS]
// @1757442475 [H2oiGEVPrcLv]
// @1757452353 [h0D8DWBqq05X]
// @1757474777 [7hlMFG1H8K76]
// @1757485845 [5OVeHbHhq8SU]
// @1757529074 [fPNrN7BolZJk]
// @1757538771 [5sC88v3WtJQl]
// @1757551187 [x1vMi8pD69ZP]
// @1757561465 [Ud7Opyxfz8bc]
// @1757572272 [AWA1IlbJxXNb]
// @1757581850 [MeXJawfp4DWF]
