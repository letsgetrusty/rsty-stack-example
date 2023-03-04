use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

#[derive(Deserialize)]
pub struct RowId {
    pub id: String,
}
