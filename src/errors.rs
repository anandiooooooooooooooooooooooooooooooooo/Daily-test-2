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
// @1753650888 [zqpUxx0QShpL]
// @1753663908 [z16wFzCFy88D]
// @1753684652 [WD4jTxt2fVZN]
// @1753694335 [MMD3JS8yZhtI]
// @1753716179 [H2CcaxC5iVE0]
// @1753727830 [JNp8XzG1H0Jk]
// @1753750373 [jkKBR5PiwApc]
// @1753761987 [l5V3KKvUxeJQ]
// @1753770963 [sRzDYAISsrbH]
// @1753780620 [sWZES11YOhTL]
// @1753802641 [ov5oEu9JT2sM]
// @1753823749 [FktlccNl9iGc]
// @1753836470 [LlA04lUMx7dB]
// @1753848216 [pfgaANcVDgR5]
// @1753857430 [hmop1Z3EbTwA]
// @1753867112 [ktqLceU00eEP]
// @1753880219 [Z9WS3DevdlUN]
// @1753888947 [bTXTZb2xHb9B]
// @1753900586 [nVZackKzBjSS]
// @1753910155 [gHAWYaheIJie]
// @1753922854 [68vVOoOFLC22]
// @1753934558 [P2JptMr5iPcQ]
// @1753943757 [YltAftli4JEu]
// @1753953468 [O0CTS5bPrR3H]
// @1753975340 [LU2q1ywVQC7J]
// @1753986994 [7JoFMYPlFRqg]
// @1753996549 [RJt0hIwIIW4V]
// @1754030229 [bX6DV0uJ0QB3]
// @1754039812 [thNiL727gnlY]
// @1754073342 [oOHI7DtDbl9f]
// @1754095490 [3B5GjpgdaP7m]
// @1754106914 [0xzo8mH9vpmm]
// @1754116332 [8JEcARgWQEDU]
// @1754139067 [LJKTXq7hoytt]
// @1754147971 [1lfBs4fiTnzl]
// @1754159609 [qUysrgfSAdv0]
