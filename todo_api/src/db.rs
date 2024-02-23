use crate::utils::macros::map;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use surrealdb::{
    dbs::{Response, Session},
    kvs::Datastore,
    sql::{thing, Value},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Task> for Value {
    fn from(val: Task) -> Self {
        match val.id {
            Some(id) => map![
                "id".to_string() => id.into(),
                "title".into() => val.title.into(),
                "completed".into() => val.completed.into(),
            ]
            .into(),
            None => map![
                "title".to_string() => val.title.into(),
                "completed".into() => val.completed.into()
            ]
            .into(),
        }
    }
}

impl Creatable for Task {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

pub trait Creatable: Into<Value> {}

#[derive(Clone)]
pub struct DB {
    pub datastore: Arc<Datastore>,
    pub session: Session,
}

impl DB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, crate::error::Error> {
        let res = self.datastore.execute(query, &self.session, vars).await?;
        Ok(res)
    }

    pub async fn add_task(&self, title: &str) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "CREATE tasks SET title = $title, completed = false, created_at = time::now()";
        let vars: BTreeMap<String, Value> = map!["title".into() => Value::Strand(title.into())];
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        Ok(first_res.result?.into_json())
    }

    pub async fn get_task(&self, id: &str) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let ress = self.execute(sql, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        Ok(first_res.result?.into_json())
    }

    pub async fn get_all_tasks(&self) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "SELECT * FROM tasks ORDER BY created_at ASC;";

        let res = self.execute(sql, None).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        Ok(first_res.result?.into_json())
    }

    pub async fn toggle_task(&self, id: &str) -> Result<AffectedRows, crate::error::Error> {
        let sql = "UPDATE $th SET completed = function() { return !this.completed; }";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }

    pub async fn delete_task(&self, id: &str) -> Result<AffectedRows, crate::error::Error> {
        let sql = "Delete $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}
