use leptos::prelude::*;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::app::components::{DashboardChart, DashboardHeader};
use crate::app::models::Person;





// Simulated backend function
pub async fn get_persons() -> Result<Vec<Person>, String> {
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
pub fn HomePage() -> impl IntoView {

       let get_persons_info = Resource::new(|| (), |_| async move { get_persons().await });

    view! {
        <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
            <DashboardHeader />
            <Suspense fallback=move || {
                view! { <p>"Loading data..."</p> }
            }>
                {
                    move || {
                        get_persons_info.get().map(|data| {
                            match data {
                                Ok(persons_data) => {
                                    view! {
                                        <DashboardChart persons_data />
                                    }.into_any()
                                },
                                Err(_) => view! {
                                    <div>"No Data Found"</div>
                                }.into_any()
                            }
                        })
                    }
                }
            </Suspense>
        </div>
    }
}