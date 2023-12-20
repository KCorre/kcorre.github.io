use yew::prelude::*;

static_toml::static_toml! {
    static TOML = include_toml!("static/toml/contacts.toml");
}

pub struct ContactEntry {
    title: &'static str,
    name: &'static str,
    street: &'static str,
    postcode: &'static str,
    city: &'static str,
    country: &'static str,
    email: &'static str,
}


lazy_static! {static ref CONTACTS: Vec<ContactEntry> = vec![
    ContactEntry { title: TOML.work.title, name:TOML.work.name, street: TOML.work.street, postcode: TOML.work.postcode, city: TOML.work.city, country: TOML.work.country, email: TOML.work.email }
];
}

#[function_component(Contact)]
pub fn contact() -> Html {

    html! {
        <div>
        <div class="contact">
            <h2> {"Online"} </h2>
            <table>
                <thead>
                    <tr>
                    <th align="left"></th>
                    <th align="left"></th>
                    </tr>
                </thead>
                <tbody>
                <tr>
                <td align="left"><strong>{TOML.online.github.title}</strong></td>
                <td align="left"><a href={TOML.online.github.url}>{TOML.online.github.value}</a></td>
                </tr>
                <tr>
                <td align="left"><strong>{TOML.online.gitlab.title}</strong></td>
                <td align="left"><a href={TOML.online.gitlab.url}>{TOML.online.gitlab.value}</a></td>
                </tr>
                <tr>
                <td align="left"><strong>{TOML.online.linkedin.title}</strong></td>
                <td align="left"><a href={TOML.online.linkedin.url}>{TOML.online.linkedin.value}</a></td>
                </tr>
                </tbody>
            </table>
        </div>
        {
            CONTACTS.iter().map(|entry| html! {
            <div class="contact">
                <h2> {entry.title} </h2>
                <table>
                    <thead>
                        <tr>
                        <th align="left"></th>
                        <th align="left"></th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                        <td align="left"><strong>{"Address:"}</strong></td>
                        <td align="left">{entry.title}</td>
                        </tr>
                        <tr>
                        <td align="left"></td>
                        <td align="left">{entry.street}</td>
                        </tr>
                        <tr>
                        <td align="left"></td>
                        <td align="left">{format!("{} {} - {}", entry.postcode, entry.city, entry.country)}</td>
                        </tr>
                        <tr>
                        <td align="left"><strong>{"Email:"}</strong></td>
                        <td align="left"><a href={format!("mailto:{}", entry.email)}>{entry.email}</a></td>
                        </tr>
                    </tbody>
                </table>
            </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}
