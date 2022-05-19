pub mod list_selector;
pub mod list_view;
pub mod login;
pub mod signup;
pub mod task;
pub mod task_info;
pub mod task_list;

use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

use list_view::*;
use login::*;
use signup::*;

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| Rc::new(RefCell::new(None)));

    html! {
        <ContextProvider<UserId> context={(*ctx).clone()}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<UserId>>
    }
}

//------------------------------------Routing-------------

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    ListView,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! {<Login/>},
        Route::Signup => html! {<Signup/>},
        Route::ListView => html! {<ListView/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

pub type UserId = Rc<RefCell<Option<i32>>>;

// pub type User = Rc<UserInner>;

// #[derive(Debug, PartialEq)]
// pub struct UserInner {
//     pub username: RefCell<String>,
//     pub password: RefCell<String>,
//     pub email: RefCell<String>,
// }

//------------------------------------Routing-------------
