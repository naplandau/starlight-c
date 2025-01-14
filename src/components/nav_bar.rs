use dioxus::prelude::*;
use crate::route::AppRoute;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        // nav {
        //     ul {
        //         li {
        //             Link { to: AppRoute::Home {}, "Home" }
        //         }
        //     }
        // }

        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<AppRoute> {}
    }
}