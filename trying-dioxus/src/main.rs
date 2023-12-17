#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::components::{Hero, HeroStruct};

mod components;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let hero_data = HeroStruct {
        title: "Connecting African Developers".to_owned(),
        by: "AfriDev Network".to_string(),
    };
    cx.render(rsx! {
        h1 { "Welcome to AfriDev Network " }
      Hero(cx, hero_data) {

        }
    })
}
