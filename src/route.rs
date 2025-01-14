#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use super::components::*;
use super::pages::*;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum AppRoute {
    // The home page is at the / route
    // #[layout(NavBar)]
    //     #[route("/")]
    //     Home {},
    // #[end_layout]

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
