use crate::api::api::api_test;

use crate::api::api::ForecastPeriod;

use crate::Route;

use dioxus_free_icons::icons::{self, fa_brands_icons};
use dioxus_free_icons::Icon;

use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    let API_info: Resource<Result<Vec<ForecastPeriod>, reqwest::Error>> = use_resource(move || {
        let future = async { api_test().await };
        Box::pin(future)
    });

    let handle_click = move |_event: MouseEvent| {
        if let Some(Ok(periods)) = &*API_info.read() {
            for period in periods.iter() {
                println!(
                    "Time: {}, Temperature: {}, Forecast: {}",
                    period.startTime,
                    period.temperature.unwrap_or_default(),
                    period.shortForecast
                );
            }
        } else {
            println!("No data available or error fetching data.");
        }

        navigator().push(Route::Home {});
    };

    rsx! {
        nav {
            "style": "display:flex; flex-direction:column; height:100%; background-color:coral;  ",
            div {
                "style": "width:50px; height:50px; background-color:blue;",
                onclick: handle_click,
            }
            div {
                "style": "width:50px; height:50px; background-color:red; display:flex; align-items:center; justify-content:center;",



                Icon {
                    width: 30,
                    height: 30,
                    fill: "black",
                    icon: icons::fa_brands_icons::FaGithub,
                }


            }


        }
        Outlet::<Route> {}
    }
}
