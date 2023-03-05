use crate::models::*;
use reqwasm::{http::Request, Error};

const BASE_URL: &str = "http://localhost:8080";

pub async fn fetch_tasks() -> Result<Vec<Task>, Error> {
    Request::get(&format!("{BASE_URL}/tasks"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn create_task(title: &str) -> Result<Task, Error> {
    Request::post(&format!("{BASE_URL}/task/{title}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn toggle_task(id: String) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/task/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn delete_task(id: String) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{BASE_URL}/task/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}
