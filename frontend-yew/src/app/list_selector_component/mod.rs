use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;


#[function_component(ListSelectorComponent)]
pub fn list_selector_component() -> Html {
    let timelines = use_state(|| vec!["List1".to_owned(), "List2".to_owned(), "List3".to_owned()]);
    let timelines_clone = timelines.clone();
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter"{
            let timelines = timelines_clone.clone();
            let mut list = (*timelines).clone();
            let input: InputElement = e.target_unchecked_into();
            if input.value() != "" {
                let value = input.value();
                input.set_value("");
                list.push(value);
                timelines.set(list);
            } else { 
            }
        } else {
        }
    });
    html! {
        <div class="list_selector">
            <input
                type="new_list"
                placeholder="Add a new timeline"
                {onkeypress}
            />
            {for (*timelines).clone().iter().map(|list| html! {<button>{list}</button>})}
        </div>
    }
}