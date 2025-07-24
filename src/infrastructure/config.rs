use std::env;

pub fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [uDBhhYn1K4Ui]
// @1751239674 [BHsvZbZ2QeYk]
// @1751241408 [HON8ah9puJHx]
// @1751241894 [H4IJT5xMFaWZ]
// @1751253636 [jP1zVoWTkyu6]
// @1751340332 [xIidPp65pQhE]
// @1751598844 [hh04EMpCF7vx]
// @1752031075 [0imwl8zShhSk]
// @1752117454 [pWWmWKe4LQpu]
// @1752204184 [k1XSogwZy2Uc]
// @1752722590 [sr4ctOFrDgUP]
// @1752809081 [fDVwri7OeIU4]
// @1752895106 [bacReAxkvBES]
// @1753069183 [YJaKdKRQc89i]
// @1753154678 [7H4NANFNPT3U]
// @1753241182 [bB7dsR0LGGwq]
// @1753327480 [lcP5IxH8guKU]
