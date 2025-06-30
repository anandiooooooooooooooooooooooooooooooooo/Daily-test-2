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
