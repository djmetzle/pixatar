use leptos::prelude::*;

#[component]
pub fn Settings(string: ReadSignal<String>, set_string: WriteSignal<String>) -> impl IntoView {
    let (hue, set_hue) = signal(1u32);
    let (orient, set_orient) = signal(1i32);
    let (bitorder, set_bitorder) = signal(1i32);

    view! {
        <div>
        <h3>Enter your username</h3>
        <input id="nameinput" type="text"
            bind:value=(string, set_string)
        />
        </div>
        <br/>
        <hr/>

        <div>
        <h3>Choose a color</h3>
        <input id="huepicker" type="range"
            value={ move || hue.get() }
            on:input:target=move |ev| set_hue.set(ev.target().value().parse().unwrap())
            min=0
            max=360
        />
            <div style:background-color=move || format!("hsl( {}, 100%, 50%)", hue.get())
                style:width="64px"
                style:height="64px"
            >
            </div>
        </div>

        <fieldset>
            <legend>Background color</legend>
            <div>
                <input type="radio"
                    id="bgBlack"
                    name="bg"
                    checked
                    on:change:target=move |_| {
                        set_bg.set(1);
                    }
                    prop:value=move || bg.get().to_string()
                />
                <label for="bgBlack">Black</label>
                <input type="radio"
                    id="bgWhite"
                    name="bg"
                    on:change:target=move |_| {
                        set_bg.set(2);
                    }
                    prop:value=move || bg.get().to_string()
                />
                <label for="bgWhite">White</label>
            </div>
        </fieldset>

        <br/>
        <hr/>

        <fieldset>
            <legend>Select character layout direction</legend>
            <div>
                <input type="radio"
                    id="orientationHorizontal"
                    name="orientation"
                    checked
                    on:change:target=move |_| {
                        set_orient.set(1);
                    }
                    prop:value=move || orient.get().to_string()
                />
                <label for="orientationHorizontal">Horizontal</label>
                <input type="radio"
                    id="orientationVertical"
                    name="orientation"
                    on:change:target=move |_| {
                        set_orient.set(2);
                    }
                    prop:value=move || orient.get().to_string()
                />
                <label for="orientationVertical">Vertical</label>
            </div>
        </fieldset>

        <fieldset>
            <legend>Select bit ordering</legend>
            <div>
                <input type="radio"
                    id="bitorderMSB"
                    name="bitorder"
                    checked
                    on:change:target=move |_| {
                        set_bitorder.set(1);
                    }
                    prop:value=move || bitorder.get().to_string()
                />
                <label for="bitorderMSB">Most significant first</label>
                <input type="radio"
                    id="bitorderLSB"
                    name="bitorder"
                    on:change:target=move |_| {
                        set_bitorder.set(2);
                    }
                    prop:value=move || bitorder.get().to_string()
                />
                <label for="bitorderLSB">Least significant first</label>
            </div>
        </fieldset>

        <div>
            <p>Settings:</p>
            <pre>string: {string}</pre>
            <pre>orientation: {orient}</pre>
            <pre>bitorder: {bitorder}</pre>
        </div>
    }
}
