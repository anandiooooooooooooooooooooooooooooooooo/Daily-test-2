use chrono::{DateTime, Utc};

pub fn current_timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}
// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [77owefxR8BLR]
// @1751239674 [a7gAqI0SU02B]
// @1751241408 [siUSWrPofOus]
// @1751241894 [5rlRneZgMTs5]
// @1751253636 [GLFW1W0IeRiq]
// @1751340332 [TYvUMriwXtQY]
// @1751598844 [AzVm9LUaj1AZ]
// @1752031075 [32FI3AxlnGcv]
// @1752117454 [XQIc7ErkpHo0]
// @1752204184 [yLt36IJRe6tk]
// @1752722589 [Gy2QLAWc48lG]
// @1752809081 [9Kd8LBwoS3MR]
// @1752895106 [QjZZcTEnTCns]
// @1753069183 [Ps2ZoNbtQPZA]
// @1753154678 [xCBbCrmWMICI]
// @1753241182 [4ee0tJEHYVk1]
// @1753327480 [YzoLwko7WuHU]
// @1753413881 [BpKZv1WoQqcZ]
// @1753500095 [RS4Gsz8WgYcF]
// @1753641161 [OdhLO9Cp7VaJ]
// @1753650888 [p1BBSBcUJQW3]
