use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_event(event: &str) {
    let ts = Utc::now().to_rfc3339();
    let line = format!("[{}] {}\n", ts, event);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file");
    file.write_all(line.as_bytes()).unwrap();
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [hOkn4QRmbNKT]
// @1751239674 [6M7a8bRLkQ0t]
// @1751241408 [GyUiYg6ZBnSP]
// @1751241894 [iX65VEawC6VZ]
// @1751253636 [K5KCZtlbXdNk]
// @1751340332 [S0MOgSIpf3fa]
// @1751598844 [zP5PPJxIdsmY]
// @1752031075 [LwFFQawab0lJ]
// @1752117454 [hGSkuJMVRcxw]
// @1752204184 [t2XDX52F1Zvl]
// @1752722589 [EqpwUDXrAmd4]
// @1752809081 [lQx6Zz8DCppF]
// @1752895106 [eJmK97iLRNV5]
// @1753069183 [HKyWhMseBH7c]
// @1753154678 [XSL5fjKxqg6o]
// @1753241182 [LECaOxKffWE1]
// @1753327479 [EtGI1ahuc4NV]
// @1753413881 [Y5cyFi2on4iw]
// @1753500095 [lW2nhbvbJPaw]
// @1753641161 [nqOOmNcuyi7E]
// @1753650888 [WE53PcttkXwN]
// @1753663908 [cUYEsH90Fn6U]
// @1753684652 [3lZpcj7jt9La]
// @1753694335 [Vt8HfP38jw6w]
// @1753716179 [X9MF5Fizhpwe]
// @1753727830 [kDvh4DAnu7wN]
// @1753750373 [84KTK2Qh9Joz]
// @1753761987 [CEq7oHpdlYNQ]
