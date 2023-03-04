use crate::{
    state::{TaskAction, TaskState},
    todo_api,
};
use yew::UseReducerHandle;

pub struct TaskController {
    state: UseReducerHandle<TaskState>,
}

impl TaskController {
    pub fn new(state: UseReducerHandle<TaskState>) -> TaskController {
        TaskController { state }
    }

    pub fn init_tasks(&self) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_tasks = todo_api::fetch_tasks().await.unwrap();
            tasks.dispatch(TaskAction::Set(fetched_tasks))
        });
    }

    pub fn create_task(&self, title: String) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = todo_api::create_task(&title).await.unwrap();
            tasks.dispatch(TaskAction::Add(response));
        });
    }

    pub fn toggle_task(&self, id: String) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = todo_api::toggle_task(id.clone()).await.unwrap();
            if response.rows_affected == 1 {
                tasks.dispatch(TaskAction::Toggle(id.clone()));
            }
        });
    }

    pub fn delete_task(&self, id: String) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = todo_api::delete_task(id.clone()).await.unwrap();
            if response.rows_affected == 1 {
                tasks.dispatch(TaskAction::Delete(id.clone()));
            }
        });
    }
}
