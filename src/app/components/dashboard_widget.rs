use leptos::prelude::*;

#[component]
pub fn DashboardWidget<T, V>(title: T, value: V) -> impl IntoView
where
    T: Into<String>,
    V: Into<String>,
{
    view! {
        <div class="flex flex-col h-36 w-full max-w-[21rem] bg-[#283653] rounded
            px-10 py-4 justify-center">

            <div class="text-white text-4xl">{value.into()}</div>
            <div class="text-stone-400">{title.into()}</div>
        </div>
    }
}