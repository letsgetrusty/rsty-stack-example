const { invoke } = window.__TAURI__.tauri;

const BASE_URL = "http://localhost:8080";

function toggle_task(id) {
  fetch(`${BASE_URL}/task/${id}`, { method: "PATCH" })
    .then((response) => response.json())
    .then((res) => {
      console.log(`rows affected: ${res.rows_affected}`);
      const task_item = document.getElementById(id);
      task_item.classList.toggle("completed");
    });
}

function delete_task(id) {
  fetch(`${BASE_URL}/task/${id}`, { method: "DELETE" })
    .then((response) => response.json())
    .then((res) => {
      console.log(`rows affected: ${res.rows_affected}`);
      let elm = document.getElementById(id);
      elm.remove();
    });
}

function add_task(title) {
  console.log(`adding task: ${title}`);
  fetch(`${BASE_URL}/task/${title}`, { method: "POST" })
    .then((response) => response.json())
    .then((task) => {
      const task_list = document.getElementById("task-list");
      task_list.appendChild(construct_task(task));
    });
}

function construct_task(task) {
  var input = document.createElement('input');
  input.type = "checkbox";
  input.checked = task.completed;

  input.onclick = (e) => {
    toggle_task(task.id)
  }

  var label = document.createElement('label');
  label.innerHTML = task.title;

  var button = document.createElement('button');
  button.innerHTML = "Delete";

  button.onclick = (e) => {
    delete_task(task.id)
  }

  var task_item = document.createElement('li');
  task_item.classList.add("center");
  if (task.completed) {
    task_item.classList.add("completed");
  }
  task_item.id = task.id;
  task_item.appendChild(input);
  task_item.appendChild(label);
  task_item.appendChild(button);

  return task_item;
}

window.addEventListener("DOMContentLoaded", () => {
  window.__APP_STATE__ = {
    tasks: []
  };

  let new_task_input = document.getElementById("new-task-input");
  let new_task_btn = document.getElementById("new-task-btn");
  new_task_btn.onclick = (e) => {
    add_task(new_task_input.value)
    new_task_input.value = "";
  }

  const task_list = document.getElementById("task-list");

  fetch(`${BASE_URL}/tasks`)
    .then((response) => response.json())
    .then((tasks) => {
      tasks.forEach((task) => {
        task_list.appendChild(construct_task(task));
      });
      console.log(tasks);
    });
});
