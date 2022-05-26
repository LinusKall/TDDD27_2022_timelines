pub mod account_info;
pub mod gql;
pub mod list_selector;
pub mod list_view;
pub mod login;
pub mod navigation_bar;
pub mod signup;
pub mod task_info;
pub mod task_item;
pub mod task_list;

use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use std::cell::RefCell;
use std::rc::Rc;
use weblog::*;
use yew::prelude::*;
use yew_router::prelude::*;

use account_info::*;
use list_view::*;
use login::*;
use navigation_bar::*;
use signup::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    ListView,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/account-information")]
    AccountInfo,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type UserId = Rc<RefCell<Option<i32>>>;
pub const USER_ID_KEY: &'static str = "timelines_user_id";

pub const LOCALHOST: &'static str = "http://localhost:8000";

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| Rc::new(RefCell::new(None)));

    let set_user_id = {
        let context = ctx.clone();
        Callback::from(move |id: i32| {
            let user_id = (*context).clone();
            *user_id.borrow_mut() = Some(id);
            LocalStorage::set(USER_ID_KEY, id).unwrap();
            context.set(user_id);
        })
    };

    let switch = Switch::render(move |route: &Route| match route {
        Route::Login => html! { <Login set_user_id={set_user_id.clone()} /> },
        Route::Signup => html! { <Signup set_user_id={set_user_id.clone()} /> },
        Route::ListView => html! { <ListView/> },
        Route::AccountInfo => html! { <AccountInfo/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    });

    html! {
        <ContextProvider<UserId> context={(*ctx).clone()}>
            <BrowserRouter>
                <NavigationBar/>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UserId>>
    }
}
