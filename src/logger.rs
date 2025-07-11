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
