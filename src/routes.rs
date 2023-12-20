use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use self::contact::Contact;
use self::publications::PublicationList;
use self::home::Home;

//pub mod blog;
mod contact;
mod publications;
mod home;
//pub mod dashboard;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/publications")]
    PublicationList,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Navigation)]
fn secure() -> Html {
    let navigator = use_navigator().expect("Navigator not found");

    let onclick_root_callback = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };
    let onclick_contact_callback = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Contact))
    };
    let onclick_publications_callback = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::PublicationList))
    };
    html! {
        <nav class="nav-container">
            <p class="header">{"Kevin Corre"}</p>
            <ul class="nav flex-column">
                <li class="nav-item">
                    {"cd"}
                </li>
                <li class="nav-item px-3">
                    <a class="nav-link active" onclick={ onclick_root_callback }>
                        {"/"}
                    </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" onclick={ onclick_contact_callback }>
                        {"/contact"}
                    </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" onclick={ onclick_publications_callback }>
                        {"/publications"}
                    </a>
                </li>
                <li class="nav-item">
                    <a target="_blank" href="https://github.com/sparika/" class="nav-link">
                        {"github/sparika"}
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_callback = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(BaseRouter)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <BrowserRouter>
                <Navigation/>
                <div class="content">
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home />
        },
        Route::PublicationList => html! {
            <PublicationList />
        },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { { "404" } },
    }
}
