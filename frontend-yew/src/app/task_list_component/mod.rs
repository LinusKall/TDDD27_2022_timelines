use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use yew::Callback;

#[function_component(TaskListComponent)]
pub fn task_list_component() -> Html {
    let tasks = use_state(|| Vec::new());
    let tasks_clone = tasks.clone();
    let title = use_state(|| "Test".to_owned());
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter"{
            let tasks = tasks_clone.clone();
            let mut list = (*tasks).clone();
            let input: InputElement = e.target_unchecked_into();
            if input.value() != "" {
                let value = input.value();
                input.set_value("");
                list.push(value.to_owned());
                tasks.set(list);
            } else { 
            }
        } else {
        }
    });
    html! {
        <div class="task_list">
            <h2>{&*title}</h2> 
            <input
                type="new_todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
            <ul class="item_list">
                {for (*tasks).iter().map(|task| html! {<li>{task}</li>})}
            </ul>
        </div>
    }
}