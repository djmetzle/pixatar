use leptos::prelude::*;

#[derive(Clone, Debug)]
pub enum Background {
    Black,
    White,
}

#[derive(Clone, Debug)]
pub enum Opacity {
    Solid,
    Transparent,
}

#[derive(Clone, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug)]
pub enum Endian {
    Most,
    Least,
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub hue: u32,
    pub bg: Background,
    pub opacity: Opacity,
    pub orient: Orientation,
    pub ordering: Endian,
}

impl Spec {
    pub fn new() -> Self {
        return Self {
            hue: 152u32,
            bg: Background::Black,
            opacity: Opacity::Solid,
            orient: Orientation::Vertical,
            ordering: Endian::Least,
        };
    }

    fn with_bg(&self, bg: Background) -> Self {
        return Self { bg, ..self.clone() };
    }

    fn with_hue(&self, hue: u32) -> Self {
        return Self {
            hue,
            ..self.clone()
        };
    }

    fn with_opacity(&self, opacity: Opacity) -> Self {
        return Self {
            opacity,
            ..self.clone()
        };
    }

    fn with_orient(&self, orient: Orientation) -> Self {
        return Self {
            orient,
            ..self.clone()
        };
    }

    fn with_ordering(&self, ordering: Endian) -> Self {
        return Self {
            ordering,
            ..self.clone()
        };
    }
}

#[component]
pub fn Settings(
    string: ReadSignal<String>,
    set_string: WriteSignal<String>,
    spec: ReadSignal<Spec>,
    set_spec: WriteSignal<Spec>,
) -> impl IntoView {
    view! {
        <div>
            <h3>Enter your username</h3>
            <input id="nameinput" type="text"
                style:font-size="2em"
                bind:value=(string, set_string)
            />
        </div>

        <hr/>

        <div class="control-group">
            <h3>Choose a color</h3>
            <input id="huepicker" type="range"
                on:input:target=move |ev| {
                    let val = ev.target().value().parse().unwrap();
                    set_spec.set(spec.get().with_hue(val));
                }
                min=0
                max=360
            />
            <div style:background-color=move || format!("hsl( {}, 100%, 50%)", spec.get().hue)
                style:width="64px"
                style:height="64px"
            />
        </div>

        <hr/>

        <div class="selectgrid">
            <fieldset>
                <legend>Background color</legend>
                <div>
                    <input type="radio"
                        id="bgWhite"
                        name="bg"
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_bg(Background::White));
                        }
                    />
                    <label for="bgWhite">White</label>
                    <br/>
                    <input type="radio"
                        id="bgBlack"
                        name="bg"
                        checked
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_bg(Background::Black));
                        }
                    />
                    <label for="bgBlack">Black</label>
                </div>
            </fieldset>

            <fieldset>
                <legend>Transparency</legend>
                <div>
                    <input type="radio"
                        id="opacitySolid"
                        name="opacity"
                        checked
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_opacity(Opacity::Solid));
                        }
                    />
                    <label for="opacitySolid">Solid</label>
                    <br/>
                    <input type="radio"
                        id="opacityTransparent"
                        name="opacity"
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_opacity(Opacity::Transparent));
                        }
                    />
                    <label for="opacityTransparent">Transparent</label>
                </div>
            </fieldset>

            <fieldset>
                <legend>Select character layout direction</legend>
                <div>
                    <input type="radio"
                        id="orientationHorizontal"
                        name="orientation"
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_orient(Orientation::Horizontal));
                        }
                    />
                    <label for="orientationHorizontal">Horizontal</label>
                    <br/>
                    <input type="radio"
                        id="orientationVertical"
                        name="orientation"
                        checked
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_orient(Orientation::Vertical));
                        }
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
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_ordering(Endian::Most));
                        }
                    />
                    <label for="bitorderMSB">Most significant first</label>
                    <br/>
                    <input type="radio"
                        id="bitorderLSB"
                        name="bitorder"
                        checked
                        on:change:target=move |_| {
                            set_spec.set(spec.get().with_ordering(Endian::Least));
                        }
                    />
                    <label for="bitorderLSB">Least significant first</label>
                </div>
            </fieldset>
        </div>
    }
}
