use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use super::task_list_component::TaskListComponent;

pub struct ListSelector {
    timelines: Vec<String>
}

pub enum Msg {
    AddList(String),
    OpenList(String),
}

impl Component for ListSelector {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { timelines: vec!["list1".to_string(), "list2".to_string(), "list3".to_string()] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddList(list) => {self.timelines.push(list); true},
            Msg::OpenList(list) => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter"{
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    Some(Self::Message::AddList(value))
                } else {
                    None
                }
            } else {
                None
            }
        });
        html! {
            <div class="list_selector">
                <input
                    type="new_list"
                    placeholder="Add a new timeline"
                    {onkeypress}
                />
                {for self.timelines.iter().map(|list| html! {<button onclick={ctx.link().callback(|_| Msg::OpenList)}>{list}</button>})}
            </div>
        }
    }
}