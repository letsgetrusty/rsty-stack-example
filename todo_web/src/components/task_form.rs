use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub on_create_task: Callback<String>,
}

#[function_component(TaskForm)]
pub fn task_form(TaskFormProps { on_create_task }: &TaskFormProps) -> Html {
    let input_node_ref = use_node_ref();

    let on_click = {
        let input_node_ref = input_node_ref.clone();
        let on_create_task = on_create_task.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                on_create_task.emit(input.value());
                input.set_value("");
            }
        })
    };

    html!(
        <div>
            <label for="new-task">
                {"Add Item"}
            </label>
            <div class="center">
                <input ref={input_node_ref} id="new-task" type="text" />
                <button onclick={on_click}>{"Add"}</button>
            </div>
        </div>
    )
}
