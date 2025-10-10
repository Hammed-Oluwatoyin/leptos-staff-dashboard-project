use leptos::prelude::*;
use chrono::NaiveDate;
use uuid::Uuid;
use std::rc::Rc;

use crate::app::{components::{AddPersonModal, PersonRow, Toast, ToastMessage}, models::Person};


// Simulated backend function
pub async fn get_persons() -> Result<Vec<Person>, String>{
      // Static team data (pretend this came from the backend)
   let team = vec![
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Alice Johnson".to_string(),
            title: "Frontend Engineer".to_string(),
            level: "Mid-Level".to_string(),
            compensation: 85_000,
            joined_date: NaiveDate::from_ymd_opt(2022, 3, 10).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Bob Smith".to_string(),
            title: "Frontend Engineer".to_string(),
            level: "Mid-Level".to_string(),
            compensation: 110_000,
            joined_date: NaiveDate::from_ymd_opt(2021, 7, 5).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Carla Adams".to_string(),
            title: "UI/UX Designer".to_string(),
            level: "Junior".to_string(),
            compensation: 65_000,
            joined_date: NaiveDate::from_ymd_opt(2023, 1, 12).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "David Lee".to_string(),
            title: "Frontend Engineer".to_string(),
            level: "Mid-Level".to_string(),
            compensation: 95_000,
            joined_date: NaiveDate::from_ymd_opt(2020, 10, 2).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Emma Brown".to_string(),
            title: "UI/UX Designer".to_string(),
            level: "Junior".to_string(),
            compensation: 120_000,
            joined_date: NaiveDate::from_ymd_opt(2019, 6, 17).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Frank Wilson".to_string(),
            title: "UI/UX Designer".to_string(),
            level: "Junior".to_string(),
            compensation: 92_500,
            joined_date: NaiveDate::from_ymd_opt(2022, 8, 24).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Grace Kim".to_string(),
            title: "QA Engineer".to_string(),
            level: "Junior".to_string(),
            compensation: 68_000,
            joined_date: NaiveDate::from_ymd_opt(2023, 4, 19).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Henry Zhang".to_string(),
            title: "QA Engineer".to_string(),
            level: "Senior".to_string(),
            compensation: 130_000,
            joined_date: NaiveDate::from_ymd_opt(2020, 2, 8).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Ivy Thompson".to_string(),
            title: "QA Engineer".to_string(),
            level: "Lead".to_string(),
            compensation: 145_000,
            joined_date: NaiveDate::from_ymd_opt(2018, 11, 3).unwrap().to_string(),
        },
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Jack Rivera".to_string(),
            title: "QA Engineer".to_string(),
            level: "Mid-Level".to_string(),
            compensation: 75_000,
            joined_date: NaiveDate::from_ymd_opt(2021, 9, 30).unwrap().to_string(),
        },
    ];
    Ok(team)
        
}


#[component]
pub fn TeamPage() -> impl IntoView {
    const ADD_BUTTON_STYLE: &str = "bg-[#7734e7] px-8 py-2 rounded text-white
        transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    let (if_show_modal, set_if_show_modal) = signal(false);
    let (if_show_toast, set_if_show_toast) = signal(false);
    let (toast_message, set_toast_message) = signal(ToastMessage::new());

    // Create the resource
   let get_persons_info = Resource::new(|| (), |_| async move { get_persons().await });

    let on_click = move |_| {
        set_if_show_modal(!if_show_modal());
    };

    view! {
        <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
            <Toast
                toast_message
                if_appear=if_show_toast
                set_if_appear=set_if_show_toast
            />
            <div class="mt-20">
                <div class="text-white flex flex-col w-full mx-auto items-center justify-center z-25">
                    <Show when=move || { if_show_modal() }>
                        <AddPersonModal
                            set_if_show_modal
                            set_if_show_added=set_if_show_toast
                            set_toast_message
                        />
                    </Show>
                    <div class="flex flex-row w-full max-w-[52rem]">
                        <div class="pr-4 mt-4 text-xl">"Members"</div>
                        <hr class="w-full max-w-[48rem] pl-4 pr-4 pt-4 mt-8 mr-4" />
                        <button on:click=on_click class=ADD_BUTTON_STYLE>
                            "Add"
                        </button>
                    </div>
                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        
                          <div class="flex flex-col w-full max-w-[52rem] mt-6">
                            {
                                move || {
                                    get_persons_info.get().map(|persons_data| {
                                             match persons_data {
                                                      Ok(persons) => {
                                                           persons.iter().map(|each_person| view! {
                                                                <PersonRow
                                                                    person=each_person.clone()
                                                                    person_resource=persons.clone()
                                                                    set_if_show_toast
                                                                    set_toast_message
                                                                />
                                                            }).collect_view().into_any()
                                                      },
                                                      Err(_) => {
                                                           view! { <div>"what"</div> }.into_any()
                                                      },
                                                   }
                                      
                                    })
                                }
                            }
                        </div>
                       
                    </Suspense>
                </div>
            </div>
        </div>
    }
}