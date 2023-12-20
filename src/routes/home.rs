use yew::{function_component, Html, Properties};

static PRESENTATION: &str = include_str!("../../static/html/presentation.html");

#[function_component(Home)]
pub fn home() -> Html {
    /*let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(markdown::to_html("## Hello, *world*!"));

    Html::VRef(div.into())*/

    Html::from_html_unchecked(PRESENTATION.into())
}