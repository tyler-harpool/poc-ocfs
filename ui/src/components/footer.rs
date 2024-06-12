use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-auto bg-gray-800 text-white p-4 text-center mt-auto">
            <div class="container mx-auto">
                <p>&copy; 2024 Open Case Filing System. All rights reserved.</p>
            </div>
        </footer>
    }
}