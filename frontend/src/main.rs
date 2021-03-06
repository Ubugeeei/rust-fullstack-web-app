use root::todo::repository::{
    complete_todo_mutation, create_todo_mutation, incomplete_todo_mutation, TodoRepository,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod graphql;

pub mod root;
use crate::root::todo::components::todo_card::TodoCard;
use crate::root::todo::repository::get_todos_query::{self, GetTodosQueryGetTodos};

#[function_component(App)]
fn app() -> Html {
    let todos: UseStateHandle<Vec<GetTodosQueryGetTodos>> = use_state(|| vec![]);
    let resync = use_state(|| false);

    // dialog state
    let is_opened_create_dialog = use_state(|| false);
    let open_create_dialog = {
        let is_opened_create_dialog = is_opened_create_dialog.clone();
        Callback::from(move |_| {
            is_opened_create_dialog.set(true);
        })
    };
    let close_create_dialog = {
        let is_opened_create_dialog = is_opened_create_dialog.clone();
        Callback::from(move |_| {
            is_opened_create_dialog.set(false);
        })
    };

    /*
     * create
     */
    // form state
    // title
    let new_title = use_state(|| "".to_string());
    let on_change_new_title = {
        let new_title = new_title.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_title.set(input.value());
        })
    };
    // description
    let new_description = use_state(|| "".to_string());
    let on_change_new_description = {
        let new_description = new_description.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_description.set(input.value());
        })
    };
    // create mutation
    let create_todo = {
        let new_title = new_title.clone();
        let new_description = new_description.clone();
        let is_opened_create_dialog = is_opened_create_dialog.clone();
        let resync = resync.clone();
        #[allow(unused_must_use)]
        Callback::from(move |_| {
            let variables = create_todo_mutation::Variables {
                title: new_title.to_string(),
                description: new_description.to_string(),
            };
            wasm_bindgen_futures::spawn_local(async move {
                TodoRepository::create(variables).await;
            });
            is_opened_create_dialog.set(false);
            resync.set(true);
        })
    };

    /*
     * toggle complete
     */
    let toggle_complete = {
        // let resync = resync.clone();
        // resync.set(true);
        #[allow(unused_must_use)]
        Callback::from(move |todo: GetTodosQueryGetTodos| {
            wasm_bindgen_futures::spawn_local(async move {
                if todo.is_done {
                    TodoRepository::incomplete(incomplete_todo_mutation::Variables { id: todo.id })
                        .await;
                } else {
                    TodoRepository::complete(complete_todo_mutation::Variables { id: todo.id })
                        .await;
                }
            });
        })
    };

    /*
     * fetching todos
     */
    {
        let todos = todos.clone();
        let _resync = resync.clone();
        use_effect_with_deps(
            move |_| {
                let todos = todos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let res = TodoRepository::get(get_todos_query::Variables {})
                        .await
                        .unwrap()
                        .get_todos;
                    todos.set(res);
                    _resync.set(false);
                });
                || ()
            },
            resync,
        );
    }

    html! {
        <div>
        <h1>{ "Todo App !" }</h1>

        <button type="button" onclick={open_create_dialog}>{"new"}</button>
        <dialog open={*is_opened_create_dialog}>
        <h2>{"Create New Todos!"}</h2>
            <div>
                <input
                    placeholder="title"
                    label="title"
                    onchange={on_change_new_title}
                    value={(*new_title).clone()}
                />
                <input
                    placeholder="description"
                    label="description"
                    onchange={on_change_new_description}
                    value={(*new_description).clone()}
                />
                </div>
                <button type="button" onclick={close_create_dialog}>{"cancel"}</button>
                <button type="button" onclick={create_todo}>{"create"}</button>
            </dialog>

            <ul>
            {
                    todos.iter()
                        .map(|todo| html! {
                            <li key={todo.id} style="list-style: none;"><TodoCard todo={todo.clone()} oncheck={toggle_complete.clone()} /></li>
                        })
                        .collect::<Html>()
            }
            </ul>

        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
