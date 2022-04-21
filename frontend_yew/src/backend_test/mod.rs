use reqwasm::http::Request;
use yew::prelude::*;
use yew::functional::UseStateHandle;

// https://yew.rs/docs/tutorial#fetching-data-using-external-rest-api

pub fn read_request(string_handle: &UseStateHandle<String>) {
    let resp = (*string_handle).clone();
        use_effect_with_deps(move |_| {
            let resp = resp.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched: String = Request::get("/test")
                    .send()
                    .await
                    .expect("Could not fetech from server")
                    .text()
                    .await
                    .expect("Could not translate response to text");
                resp.set(fetched);
            });
            || ()
        }, ());
}


#[function_component(App)]
pub fn app() -> Html {
    let resp = use_state_eq(|| String::from("Hello world!"));
    
    read_request(&resp);

    html! {
        <div styles="background-color: red;">
            { &(*resp) }
        </div>
    }
    
}
