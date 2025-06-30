use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppError: {}", self.message)
    }
}
// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [mGd5RYTAHNhp]
// @1751239674 [ZgqnIqPaWREZ]
// @1751241407 [d9XkllnMeXw3]
// @1751241894 [dVujhwlIAlIn]
// @1751253636 [1zSYaQEwv36I]
