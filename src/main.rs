use leptos::prelude::*;

mod generator;
mod settings;

use crate::generator::Generator;
use crate::settings::Settings;
use crate::settings::Spec;

#[component]
fn App() -> impl IntoView {
    let (str, set_str) = signal("pixatar".to_string());
    let (spec, set_spec) = signal(Spec::new());

    view! {
        <h1>Pixatar</h1>
        <h3>Generate pixel art avatar images</h3>
        <hr/>
        <Settings string=str set_string=set_str spec=spec set_spec=set_spec />
        <hr/>
        <Generator string=str spec=spec />
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
