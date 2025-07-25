use crate::domain::entities::Task;
use std::error::Error;

pub trait TaskRepository {
    fn all(&self) -> Result<Vec<Task>, Box<dyn Error>>;
    fn add(&mut self, task: Task) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, task: Task) -> Result<(), Box<dyn Error>>;
    fn remove(&mut self, task_id: u64) -> Result<(), Box<dyn Error>>;
    fn find_by_id(&self, task_id: u64) -> Result<Option<Task>, Box<dyn Error>>;
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [i7sOK1JYirZ0]
// @1751239674 [9FnfATFL8kGS]
// @1751241407 [9WroWHrGxyA9]
// @1751241894 [zJuqJgLLJc3E]
// @1751253636 [YCeo4TWo9XmV]
// @1751340332 [U6uKsWbjirDS]
// @1751598844 [mFaFYkMlGP66]
// @1752031075 [IgVrFTGSbPem]
// @1752117454 [dSm0DbREIDKs]
// @1752204184 [TTSpqx6nPZH6]
// @1752722589 [UFL27HLmeDuj]
// @1752809081 [u4q5vgJc1xu1]
// @1752895106 [bgnBbefiXcIB]
// @1753069183 [A3gawv0J9wWn]
// @1753154678 [J8OsvSk9mPFX]
// @1753241182 [esI8YtQTp6z1]
// @1753327479 [Zt2DGGvjN6Ia]
// @1753413881 [TuMr95HAicfU]
// @1753500095 [pYWzEcpRTAYJ]
