use yew::prelude::*;
pub mod task_list_component;
use task_list_component::*;
pub mod list_selector_component;
use list_selector_component::*;

pub struct App;

pub enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <ListSelector/>
                <TaskListComponent/>
            </div>
        }
    }
}