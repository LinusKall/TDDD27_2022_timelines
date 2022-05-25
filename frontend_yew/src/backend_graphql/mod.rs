// use graphql_client::{GraphQLQuery, Response};
// use wasm_bindgen_futures::spawn_local;
// use yew::functional::UseStateHandle;
// use yew::prelude::*;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "../graphql_api/schema.graphql",
//     query_path = "../graphql_api/apiVersion.graphql",
//     response_derives = "Debug"
// )]
// struct Query;

// pub fn full_request(string_handle: &UseStateHandle<String>) {
//     let handle = (*string_handle).clone();
//     use_effect_with_deps(
//         move |_| {
//             let handle = handle.clone();
//             spawn_local(async move {
//                 let request_body = Query::build_query(query::Variables {});

//                 let client = reqwest::Client::new();
//                 let res = client
//                     .post("http://localhost/graphql")
//                     .json(&request_body)
//                     .send()
//                     .await
//                     .expect("Could not send request");

//                 let response_body: Response<query::ResponseData> =
//                     res.json().await.expect("Could not parse response");
//                 handle.set(format!("{:#?}", response_body));
//             });
//             || ()
//         },
//         (),
//     );
// }

// #[function_component(App)]
// pub fn app() -> Html {
//     let resp = use_state_eq(|| String::from("Hello world!"));

//     full_request(&resp);

//     html! {
//         <div styles="background-color: red;">
//             { &(*resp) }
//         </div>
//     }
// }
