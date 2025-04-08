use leptos::prelude::*;

mod settings;

use crate::settings::Settings;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>Pixatar</h1>
        <h3>Generate pixel art avatar images</h3>
        <hr/>
        <Settings />
        <hr/>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
