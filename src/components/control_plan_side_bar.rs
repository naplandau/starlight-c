use dioxus::prelude::*;
use crate::route::AppRoute;

#[component]
pub fn ControlPlanSideBar() -> Element {
    rsx! {
        div {
            class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<AppRoute> {}
    }
}