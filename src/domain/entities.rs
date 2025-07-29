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
