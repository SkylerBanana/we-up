#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]

enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/settings")]
    Settings {},
}

fn main() {
    // Init logger

    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            "style": "display:flex;   height:100%;",

            Router::<Route> {}
        }

    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Link { to: Route::Blog {id:52}, "Blog" }

        div {


          p{
            "hi"
          }

        }

    }
}

#[component]
fn Nav() -> Element {
    let nav = navigator();
    let handle_click = move |event: MouseEvent| {
        nav.push(Route::Home {});
    };
    rsx! {




        nav {
            "style": "display:flex; flex-direction:column; height:100%; background-color:coral; ",




       a{
           "style": "width:50px; height:50px; background-color:blue;",

          onclick: handle_click,

















       }

       a{
        "style": "width:50px; height:50px; background-color:red;",
        Link { to: Route::Home {},}
      Outlet::<Route> {}
       }


        }
    }
}

fn Settings() -> Element {
    rsx! {

        p{
            "Hi This Is Settings"
        }
    }
}
