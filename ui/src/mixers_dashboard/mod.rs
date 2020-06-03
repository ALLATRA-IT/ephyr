use dominator::{clone, events, html, Dom};
use futures_signals::{
    signal::{Mutable, SignalExt as _},
    signal_vec::SignalVecExt as _,
};

use crate::state;

pub struct MixersDashboard;

impl MixersDashboard {
    pub fn render(state: &state::State) -> Dom {
        html!("div", {
            .class("mixers_dashboard")
            .class("c-tab__panel")
            .attribute_signal("aria-labeledby", state.active_stream.signal()
                .dedupe()
                .map(|n| format!("stream-{}", n)))
            .attribute("role", "tabpanel")
            .children(&mut [
                Self::render_input_info(state),
                Self::render_mixers(state),
            ])
        })
    }

    pub fn render_input_info(state: &state::State) -> Dom {
        let streams = state.streams.clone();

        html!("div", {
            .class("input_info")
            .text_signal(state.active_stream.signal().dedupe()
                .switch(move |num| {
                    let streams = streams.lock_ref();
                    let stream = streams.as_slice().get(num).unwrap();
                    stream.name.signal_cloned().dedupe_cloned()
                })
                .map(|name| {
                    format!("Input URL: rtmp://127.0.0.1/{}/????", name)
                }))
        })
    }

    pub fn render_mixers(state: &state::State) -> Dom {
        let streams = state.streams.clone();

        html!("form", {
            .class("mixers")
            .children_signal_vec(state.active_stream.signal().dedupe()
                .switch_signal_vec(move |num| {
                    let streams = streams.lock_ref();
                    let stream = streams.as_slice().get(num).unwrap();
                    stream.mixers.signal_vec_cloned()
                })
                .map(Self::render_mixer))
        })
    }

    pub fn render_mixer(mixer: state::Mixer, ) -> Dom {
        html!("fieldset", {
            .class("mixer")
            .text_signal(mixer.name.signal_cloned().dedupe_cloned()
                .map(String::from))
        })
    }
}
