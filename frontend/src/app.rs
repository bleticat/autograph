use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

async fn tauri_invoke<T: serde::de::DeserializeOwned>(cmd: &str, args: &impl Serialize) -> T {
    let args = serde_wasm_bindgen::to_value(args).unwrap();
    let result = JsFuture::from(invoke(cmd, args)).await.unwrap();
    serde_wasm_bindgen::from_value(result).unwrap()
}

#[component]
pub fn App() -> impl IntoView {
    let todos = RwSignal::new(Vec::<Todo>::new());
    let input_value = RwSignal::new(String::new());

    // Load todos on mount
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let loaded: Vec<Todo> = tauri_invoke("get_todos", &serde_json::json!({})).await;
            todos.set(loaded);
        });
    });

    let do_add = move || {
        let title = input_value.get_untracked().trim().to_string();
        if title.is_empty() {
            return;
        }
        input_value.set(String::new());
        leptos::task::spawn_local(async move {
            let new_todos: Vec<Todo> =
                tauri_invoke("add_todo", &serde_json::json!({ "title": title })).await;
            todos.set(new_todos);
        });
    };

    let toggle_todo = move |id: i64| {
        leptos::task::spawn_local(async move {
            let new_todos: Vec<Todo> =
                tauri_invoke("toggle_todo", &serde_json::json!({ "id": id })).await;
            todos.set(new_todos);
        });
    };

    let delete_todo = move |id: i64| {
        leptos::task::spawn_local(async move {
            let new_todos: Vec<Todo> =
                tauri_invoke("delete_todo", &serde_json::json!({ "id": id })).await;
            todos.set(new_todos);
        });
    };

    view! {
        <main>
            <h1>"Autograph Todo"</h1>
            <div class="input-row">
                <input
                    type="text"
                    placeholder="What needs to be done?"
                    prop:value=move || input_value.get()
                    on:input=move |ev| input_value.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            do_add();
                        }
                    }
                />
                <button on:click=move |_| do_add()>"Add"</button>
            </div>
            <ul class="todo-list">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    let:todo
                >
                    {
                        let id = todo.id;
                        let completed = todo.completed;
                        view! {
                            <li class:completed=completed>
                                <input
                                    type="checkbox"
                                    prop:checked=completed
                                    on:change=move |_| toggle_todo(id)
                                />
                                <span>{todo.title.clone()}</span>
                                <button class="delete" on:click=move |_| delete_todo(id)>
                                    "\u{00d7}"
                                </button>
                            </li>
                        }
                    }
                </For>
            </ul>
        </main>
    }
}
