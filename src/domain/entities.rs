use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u64, title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description,
            created_at: now,
            updated_at: now,
            completed: false,
        }
    }

    pub fn toggle_completion(&mut self) {
        self.completed = !self.completed;
        self.updated_at = Utc::now();
    }

    pub fn update(&mut self, title: Option<String>, description: Option<String>) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(d) = description {
            self.description = Some(d);
        }
        self.updated_at = Utc::now();
    }
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [xvm2PlRitC7a]
// @1751239674 [OC5Tau5YnMbb]
// @1751241407 [4EUMqQLtmDhs]
// @1751241894 [e7aOQyUAkA9B]
// @1751253636 [CxT7nTiqsoup]
// @1751340332 [bCYTl0xpUXsp]
// @1751598844 [9nqFGFRIRxgZ]
// @1752031075 [z0YOXzJiVfQy]
// @1752117454 [fP3nQOSXwSv0]
// @1752204184 [mK5qM0IVErHm]
// @1752722589 [lvC4WqhTDeAT]
// @1752809081 [ZzUPvd5B45hs]
// @1752895106 [8wEgt3K2tB97]
// @1753069183 [g73tnoSyV2GG]
// @1753154678 [A1W90u1IO0Bk]
// @1753241182 [9qYdu16DEHRT]
// @1753327479 [YQFmpFqL7maP]
// @1753413881 [e2zkeVn1LTlV]
// @1753500095 [XUAKfcIHqclH]
// @1753641161 [1T6ly6SbuFo5]
// @1753650888 [f1QUM8r57Mi5]
// @1753663908 [mainQmaIYeOO]
// @1753684652 [9v9iFhshWTTH]
// @1753694335 [j74KZv0CAUTA]
// @1753716179 [dDEzMfJVDHDH]
// @1753727830 [CDfQ4EeCTLo4]
// @1753750373 [nUKzoZZGr915]
// @1753761987 [G8R3EXoTGqMr]
// @1753770963 [okqOWduZ3HEq]
// @1753780620 [fs79n3ddpujC]
// @1753802641 [tlOsjWCcY3Bw]
// @1753823749 [AHTvfNhozpSw]
// @1753836470 [lg182QOR6ms0]
// @1753848215 [3749AWa9N3kI]
// @1753857430 [0WOOQjmrPyrk]
// @1753867112 [QjlEe0xYo4pY]
// @1753880219 [TEUuvX2ruQwE]
// @1753888947 [4nEbwycYJ4sv]
// @1753900586 [nlYtx6MeNSEC]
// @1753910154 [ojC4u9UnM4vh]
// @1753922854 [WFvYK7W5y6As]
// @1753934557 [MRfifVcPHSvh]
// @1753943757 [s5sw9xWln5PC]
// @1753953468 [JTValYNCq0UW]
// @1753975340 [JGTXpr8gB39R]
// @1753986994 [8J9UUoXondCW]
// @1753996549 [rvV6UiQFI5Nx]
// @1754030228 [WQ7x2GInteJ6]
// @1754039812 [zIsX2TkeuUT9]
// @1754073342 [hMdOooAtvZ9e]
// @1754095490 [LVR8Vr74RkYg]
// @1754106914 [h5lpDXc4FoL2]
// @1754116331 [78q181cOkRQZ]
// @1754139067 [s0FXXdSO3q5Y]
// @1754147971 [sWPYmMrYwSVA]
// @1754159608 [vmeHMmCu9wZO]
// @1754182424 [a3Xk0f8GdX7w]
// @1754212282 [Zw5YanjHQ9yP]
// @1754225643 [86keDMdmhNqx]
// @1754268798 [YymnEr5ilK9Q]
// @1754280866 [KUdAMlAvVFtR]
// @1754289696 [WWa0GlyKj95g]
// @1754321013 [cJwmUYmpazPE]
// @1754332652 [KUWI5rWM0Oj7]
// @1754354966 [w8dds111fTXj]
// @1754375852 [pt9DVyvDEINh]
// @1754385513 [ZZNkq4z8o2sU]
// @1754398697 [CMTELqPbI6AD]
// @1754419126 [OqTDR3OEowpk]
// @1754428575 [t49C0WzmXomr]
// @1754441302 [VtA7mEujP6AO]
// @1754462258 [FAbIEhIUNAFU]
// @1754471936 [VIwCOsACLmNw]
// @1754493826 [STvZNPn3T62v]
// @1754505359 [up9RT69xiXOx]
// @1754548651 [ny6Kd8x0bjoa]
// @1754571453 [qsl6cWKCGj3e]
// @1754591881 [ar0aE9GnChXJ]
// @1754601269 [T9Dsp2v3kIBH]
// @1754614104 [sEdSbbXrFv7V]
// @1754625849 [i9fr18SCeFxm]
// @1754644661 [2TtLFRKqRToK]
// @1754657789 [gn2tbScnqsEH]
// @1754678026 [EkX5D5z3geJQ]
// @1754711383 [jZtGwLQ4ElPg]
// @1754743815 [4tCxnWoMarYN]
// @1754764355 [P5ygxnMZZNfz]
// @1754774045 [1o42hZVugELM]
// @1754817086 [fAOz9plc7OkA]
// @1754830247 [qWcyd6lPY12y]
// @1754873414 [UsOpU9PHZBoP]
// @1754885038 [N5m5xoHzjMiU]
// @1754894225 [ylluF3ARIq2o]
// @1754903848 [I880haf8XchH]
// @1754925728 [r1tmJ4qGLysX]
// @1754937382 [LzcL9aKUrKfw]
