use yew::prelude::*;
use chrono::Utc;
use chrono::prelude::*;
use chrono::offset::LocalResult;
use gloo::console::log;

#[function_component(Calender)]
pub fn calender() -> Html {
    let time = Utc::now();
    let local: DateTime<Local> = Local::now();

    html! {
        <div>
            <div>{time.day().to_rfc3339()}</div>
        </div>
    }
}