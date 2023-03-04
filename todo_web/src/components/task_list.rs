use yew::{function_component, html, Callback, Html, Properties};

use super::TaskItem;
use crate::models::Task;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
    pub on_delete_task: Callback<String>,
    pub on_toggle_task: Callback<String>,
}

#[function_component(TaskList)]
pub fn task_list(
    TaskListProps {
        tasks,
        on_delete_task,
        on_toggle_task,
    }: &TaskListProps,
) -> Html {
    let tasks: Html = tasks
        .iter()
        .map(|task| html!( <TaskItem task={task.clone()} on_delete_task={on_delete_task} on_toggle_task={on_toggle_task} /> ))
        .collect();

    html!(
        <ul id="task-list">
            {tasks}
        </ul>
    )
}
