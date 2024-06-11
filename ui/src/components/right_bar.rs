use leptos::*;

#[component]
pub fn RightBar() -> impl IntoView {
    view! {
        <aside class="space-y-6">
            <section class="bg-gray-50 p-4 rounded-lg shadow fade-in" id="login-section">
                <h2 class="text-2xl font-semibold mb-4"><span class="highlight">Login</span></h2>
                <form id="login-form" onsubmit="handleLogin(event)">
                    <div class="mb-4">
                        <label for="username" class="block text-sm font-medium text-gray-700">Username</label>
                        <input type="text" id="username" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"/>
                    </div>
                    <div class="mb-4">
                        <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
                        <input type="password" id="password" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"/>
                    </div>
                    <button type="submit" class="w-full py-2 px-4 bg-blue-500 text-white font-semibold rounded-md shadow hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">Login</button>
                </form>
            </section>
        </aside>
    }
}
