use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;


pub enum TaskListComponentMsg {
    AddTask(String)
}

pub struct TaskListComponent {
    tasks: Vec<String>,
}

impl Component for TaskListComponent {
    type Message = TaskListComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { tasks: Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, task_list_msg: Self::Message) -> bool {
        match task_list_msg {
            TaskListComponentMsg::AddTask(task) => {self.tasks.push(task); true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter"{
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    Some(Self::Message::AddTask(value))
                } else {
                    None
                }
            } else {
                None
            }
        });
        html! {
            <div class="task_list">
                <input
                    type="new_todo"
                    placeholder="What needs to be done?"
                    {onkeypress}
                />
                <ul class="item_list">
                    {for self.tasks.iter().map(|task| html! {<li>{task}</li>})}
                </ul>
            </div>
        }
    }
}