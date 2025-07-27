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
// @1751340332 [IHAeh7pjUGdu]
// @1751598844 [Rfeh24BMU49D]
// @1752031075 [nbcxbSyyvPp4]
// @1752117454 [WlkcQ9A8AeJk]
// @1752204184 [l38hXIz89t5E]
// @1752722589 [gnXIItauuTYl]
// @1752809081 [thkPk6HzWj7l]
// @1752895106 [BFD5KRx9XEPr]
// @1753069183 [0jAsDG8Aq96d]
// @1753154678 [6l0inzVvppXn]
// @1753241182 [73Ngsl3AH73Y]
// @1753327480 [uedrgHd8AJZH]
// @1753413881 [rKwPIzQr0omf]
// @1753500095 [cxXi1vngW9gt]
// @1753641161 [mQ4w7S4Nf3Vj]
