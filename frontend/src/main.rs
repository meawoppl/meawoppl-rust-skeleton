use gloo_net::http::Request;
use shared::HealthResponse;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let health = use_state(|| None::<String>);

    {
        let health = health.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get("/api/health").send().await {
                    Ok(resp) => {
                        if let Ok(data) = resp.json::<HealthResponse>().await {
                            health.set(Some(data.status));
                        }
                    }
                    Err(e) => health.set(Some(format!("Error: {}", e))),
                }
            });
        });
    }

    html! {
        <div>
            <h1>{ "App" }</h1>
            <div class="status">
                { match (*health).as_ref() {
                    Some(s) => format!("Backend: {}", s),
                    None => "Checking backend...".to_string(),
                }}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
