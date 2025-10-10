use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use crate::app::models::{EditPersonRequest, Person};
use leptos::prelude::*;
use std::rc::Rc;
use validator::Validate;

// style for each field, including subtle animations for animating the
// placeholder

// when focused upon
const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white
    mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000
    ease-in-out";

const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded
    text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

const UPDATE_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded
    text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8
    border-[#7734e7] px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed
    top-20 z-50";

const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8
    border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed
    top-20 z-50";

#[component]
pub fn EditPersonModal(
    person: Rc<Person>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    person_resource:  Vec<Person>
) -> impl IntoView {
    let (person_name, _set_person_name) = signal(person.name.clone());
    let (person_title, set_person_title) = signal(person.title.clone());
    let (person_level, set_person_level) = signal(person.level.clone());
    let (compensation, set_compensation) = signal(format!("{}", person.compensation));

    // for error messages
    let (error_message, set_error_message) = signal(String::new());
    let (if_error, set_if_error) = signal(false);

    // to close the modal
    let on_close = move |_| {
        set_if_show_modal(false);
    };

   

    view! {

        <div class="flex flex-col w-full h-full z-50 mx-auto items-center
            align-center">

            <div class={ move || {
                if if_error() { ERROR_STYLE }
                else { NO_ERROR_STYLE }
            }}>

                <Show when=move || { if_error() }>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5
                        py-3 transition-all duration-750 ease-in-out">
                        { error_message() }
                    </p>
                </Show>
                <p class="text-white pt-5 text-4xl mb-10">{person_name}</p>

                <input type="text" placeholder="Title" class=INPUT_STYLE
                    value=person_title
                    on:input=move |event| {
                        set_person_title(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Level" class=INPUT_STYLE
                    value=person_level
                    on:input=move |event| {
                        set_person_level(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Compensation" class=INPUT_STYLE
                    value=compensation
                    on:input=move |event| {
                        set_compensation(event_target_value(&event));
                    }
                />

                <div class="flex flex-row w-full items-right justify-right mt-3">

                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button class=UPDATE_BUTTON_STYLE>
                        "Update"
                    </button>
                </div>
            </div>
        </div>
    }
}