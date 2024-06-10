use crate::components::list;
use leptos::*;



#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="flex h-screen">
               Home Component
                <list/>
        </main>

    }
}