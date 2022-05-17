use yew::prelude::*;
pub mod list_selector;
pub mod list_view;
pub mod task;
pub mod task_info;
pub mod task_list;
pub mod login;

use list_view::*;
use login::*;

//------------------------------------Routing-------------
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/listview")]
    ListView,
    #[at("/timelines")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}


// History function compoonent
#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! {<Login/>},
        Route::ListView => html! {<ListView/>},
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


use std::cell::RefCell;
use std::rc::Rc;

pub type User = Rc<UserInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
    pub password: RefCell<String>,
}

//------------------------------------Routing-------------

#[function_component(App)]
pub fn app() -> Html {

    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
            password: RefCell::new("initial".into()),
        })
    });

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        </ContextProvider<User>>
    }
}
