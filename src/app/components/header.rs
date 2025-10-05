use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str = "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    // Use the location hook to get the current pathname
    let location = use_location();
    let pathname = move || location.pathname.get();

    view! {
        <div class="flex mx-auto align-center items-center w-full h-12 pt-8 px-20 top-0 fixed">
            <nav class="flex flex-row w-full max-w-[52rem] h-12">
                <a href="/" class=move || get_style_from_url(&pathname(), "/")>
                    "Dashboard"
                </a>
                <a href="/team" class=move || get_style_from_url(&pathname(), "/team")>
                    "Team"
                </a>
            </nav>
        </div>
    }
}

fn get_style_from_url(url: &str, match_url: &str) -> &'static str {
    if url == match_url {
        INPUT_STYLE_SELECTED
    } else {
        INPUT_STYLE
    }
}