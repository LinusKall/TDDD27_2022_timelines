use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use weblog::*;
use regex::Regex;

use super::Route;
use super::UserId;

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

#[derive(cynic::FragmentArguments)]
struct CreateUserInput {
    username: String,
    email: String,
    hashed_password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "CreateUserInput"
)]
struct CreateUser {
    #[arguments(input = &args)]
    create_user: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "User",
)]
struct User {
    id: i32,
}

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub set_user_id: Callback<i32>,
}

#[function_component(Signup)]
pub fn signup(props: &Properties) -> Html {
    let username = use_state(String::default);
    let password = use_state(String::default);
    let email = use_state(String::default);
    let valid_email = use_state(bool::default);
    let validate = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,6}$").unwrap();
    let user_id = use_context::<UserId>().expect("No context found.");

    if let Some(_) = *user_id.borrow() {
        return html! { <Redirect<Route> to={Route::ListView} /> };
    }

    let user_id_request = {
        let set_user_id = props.set_user_id.clone();
        let username = (*username).to_owned();
        let hashed_password = (*password).to_owned();
        let email = (*email).to_owned();
        let operation = CreateUser::build(CreateUserInput { username, email, hashed_password });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data
                .unwrap();

            console_log!(format!("{:#?}", &data));
            if let Some(User { id }) = data.create_user {
                set_user_id.emit(id);
                return Ok(id);
            }
            Err("Could not fetch user ID.")
        })
    };

    let oninput = {
        let current_username = username.clone();
        let current_password = password.clone();
        let current_email = email.clone();
        let current_valid_email = valid_email.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.name() == "username" {
                current_username.set(input.value());
            } else if input.name() == "password" {
                current_password.set(input.value());
            } else if input.name() == "email" {
                if validate.is_match(&input.value()) {
                    current_valid_email.set(true);
                } else {
                    current_valid_email.set(false);
                }
                current_email.set(input.value());
            } else {
                console_error!("Should be impossible to get here");
            }
        })
    };

    //let onclick = { Callback::from(move |_| return) };

    let onclick = {
        let user_id_request = user_id_request.clone();
        Callback::from(move |_| {
            user_id_request.run();
        })
    };

    html! {
        <>
            <form>
                <div>
                    <input name="username" oninput = {oninput.clone()} placeholder="Username"/>
                </div>
                <div>
                    <input name="password" oninput = {oninput.clone()} type="password" placeholder="Password"/>
                </div>
                <div>
                    <input name="email" {oninput} type="email" id="email" placeholder="Email"/>
                </div>
                <Link<Route> to={Route::ListView}> <button onclick = {onclick.clone()} disabled={username.len()<4 || password.len()<8 || !*valid_email}>{"Create account"}</button></Link<Route>>
            </form>
        </>
    }
}
