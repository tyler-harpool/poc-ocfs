use leptos::*;

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class="bg-gray-800 text-white p-4 flex justify-between items-center">
            <h1 class="text-xl font-bold">Open Case Filing System</h1>
            <input type="text" placeholder="Search..." class="p-2 rounded"/>
            <div class="space-x-4">
                <a href="#" class="hover:underline">Login</a>
                <a href="#" class="hover:underline">Register</a>
            </div>
        </div>
    }
}
