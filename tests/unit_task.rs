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
// @1752031075 [Nar6AyiRhiDJ]
// @1752117454 [BJudvz3xl6LN]
// @1752204184 [dJI2Poxitwor]
// @1752722590 [9kyK2axzUxy9]
// @1752809081 [W2Kygh3ziTDe]
// @1752895106 [KpiPmmno2wvu]
// @1753069183 [p4bsdHVwdYrM]
// @1753154678 [KNCNVUSrnnSI]
// @1753241182 [EmruZdRC9hGq]
// @1753327480 [LxXjVNwmhcdO]
// @1753413881 [iOmuUDkXBwVD]
// @1753500095 [UPUilysDJVqH]
// @1753641161 [XnQe696EXNwN]
// @1753650888 [Fuo9CzJv2PvE]
// @1753663908 [GBG4U76qPp7t]
// @1753684652 [ZToni1GNj5AA]
// @1753694335 [m1br1cfCuXRF]
// @1753716179 [ggu8ZjpwNlF8]
// @1753727830 [R4Q1XR0Omc8x]
// @1753750373 [gwj9EUx9UKhl]
// @1753761987 [2XT8TdM5A2JQ]
// @1753770963 [YH0lRg3CjdXi]
// @1753780620 [XaaxFMGkAmfQ]
// @1753802641 [sotoVWEeieN0]
// @1753823749 [BUhZnFy70eFQ]
