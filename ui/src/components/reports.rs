use leptos::*;
use crate::components::Card; // Ensure this import is added

#[component]
pub fn ReportsList() -> impl IntoView {
    view! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">Dashboard</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                <Card title="Total Cases".to_string() count="120".to_string() />
                <Card title="Pending Cases".to_string() count="45".to_string() />
                <Card title="Resolved Cases".to_string() count="75".to_string() />
                <Card title="Upcoming Hearings".to_string() count="12".to_string() />
            </div>
        </div>
    }
}

