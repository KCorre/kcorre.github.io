use yew::prelude::*;

static_toml::static_toml! {
    static PUB_TOML = include_toml!("static/toml/publications.toml");
}

pub struct Publication {
    title: &'static str,
    authors: &'static str,
    published: &'static str,
    link: &'static str
}

lazy_static! {static ref PUBLICATIONS: Vec<Publication> = vec![
        Publication { title: PUB_TOML.these.title, authors: PUB_TOML.these.authors, published: PUB_TOML.these.published, link: PUB_TOML.these.title },
        Publication { title: PUB_TOML.popets_2017.title, authors: PUB_TOML.popets_2017.authors, published: PUB_TOML.popets_2017.published, link: PUB_TOML.popets_2017.title },
        Publication { title: PUB_TOML.icwe_2017.title, authors: PUB_TOML.icwe_2017.authors, published: PUB_TOML.icwe_2017.published, link: PUB_TOML.icwe_2017.title }
    ];
}

lazy_static! {static ref TECHNICALS: Vec<Publication> = vec![
        Publication { title: PUB_TOML.draft_corre.title, authors: PUB_TOML.draft_corre.authors, published: PUB_TOML.draft_corre.published, link: PUB_TOML.draft_corre.title },
        Publication { title: PUB_TOML.brevet_exfo.title, authors: PUB_TOML.brevet_exfo.authors, published: PUB_TOML.brevet_exfo.published, link: PUB_TOML.brevet_exfo.title },
        Publication { title: PUB_TOML.draft_copeland.title, authors: PUB_TOML.draft_copeland.authors, published: PUB_TOML.draft_copeland.published, link: PUB_TOML.draft_copeland.title },
        Publication { title: PUB_TOML.brevet_orange.title, authors: PUB_TOML.brevet_orange.authors, published: PUB_TOML.brevet_orange.published, link: PUB_TOML.brevet_orange.title }
    ];
}

#[function_component(PublicationList)]
pub fn publications() -> Html {
        html! {
            <div class="publications">
            <h2> {"Conferences"} </h2>
            <ul>  
            {PUBLICATIONS.iter().map(|publi| html! {
                <li>
                    <a href={publi.link} class="pub-title">{publi.title}</a> {format!(", {}", publi.authors)}<br/>
                    <a class="pub-published"> {publi.published} </a>
                </li>
            }).collect::<Vec<_>>()}
            </ul>
            <h2> {"Technicals"} </h2>
            <ul>  
            {TECHNICALS.iter().map(|publi| html! {
                <li>
                    <a href={publi.link} class="pub-title">{publi.title}</a> {format!(", {}", publi.authors)}<br/>
                    <a class="pub-published"> {publi.published} </a>
                </li>
            }).collect::<Vec<_>>()}
            </ul>
            </div>
        }
    }