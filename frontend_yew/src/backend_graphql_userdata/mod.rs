use cynic::{http::SurfExt, impl_scalar, QueryBuilder};
use wasm_bindgen_futures::spawn_local;
use weblog::console_log;
use yew::functional::UseStateHandle;
use yew::prelude::*;

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

type DateTime = chrono::naive::NaiveDateTime;
impl_scalar!(DateTime, schema::DateTime);

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
struct Timeline {
    id: i32,
    title: String,
    public: bool,
    created_at: DateTime,
    updated_at: DateTime,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Query")]
struct Query {
    get_timelines: Vec<Timeline>,
}

pub fn full_request(string_handle: &UseStateHandle<String>) {
    let handle = (*string_handle).clone();
    use_effect_with_deps(
        move |_| {
            let handle = handle.clone();
            spawn_local(async move {
                let operation = Query::build(());

                let res = surf::post("http://localhost/api/graphql")
                    .run_graphql(operation)
                    .await
                    .expect("Could not send request");

                let data: Query = res.data.unwrap();
                console_log!(format!("{:#?}", &data));
                handle.set(format!("{:?}", &data));
            });
            || ()
        },
        (),
    );
}

#[function_component(App)]
pub fn app() -> Html {
    let resp = use_state_eq(|| String::from("Hello world!"));

    full_request(&resp);

    html! {
        <div>
            { &(*resp) }
        </div>
    }
}
