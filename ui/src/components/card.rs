use leptos::*;

#[component]
pub fn Card(title: String, count: String) -> impl IntoView {
    view! {
        <div class="bg-white p-4 shadow rounded-lg">
            <h2 class="text-xl font-bold">{title}</h2>
            <p class="text-3xl mt-2">{count}</p>
        </div>
    }
}
