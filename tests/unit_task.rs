use taskforge::core::task::Task;

#[test]
fn it_creates_new_task() {
    let t = Task::new(1, "Write tests".to_string(), "2025-06-29T12:00:00Z".into());
    assert_eq!(t.title, "Write tests");
    assert!(!t.completed);
}
// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [2WOhYjMupNC3]
// @1751239674 [3RGqzyYi0h5K]
// @1751241408 [lAh0HsAHgkMn]
// @1751241894 [DvA61J0iuCKG]
// @1751253636 [mMEuQKZTXoFP]
// @1751340332 [R3bglViuDlYu]
// @1751598845 [wkkA99Fensjn]
