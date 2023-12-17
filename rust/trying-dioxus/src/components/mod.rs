#![allow(non_snake_case)]

use dioxus::prelude::*;

pub struct HeroStruct {
    pub title: String,
    pub by: String,
}

pub fn Hero(cx: Scope, hero: HeroStruct) -> Element {
    let by = hero.by;
    let title = hero.title;

    render! {
        div {
            padding: "0.5rem",
            position: "relative",
            "{title} by {by}"
        }
    }
}
