use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-blue-600 p-4 center">
            <div class="container mx-auto flex flex-wrap justify-between items-center">

                <div class="block lg:hidden">
                    <button class="text-white">
                        // Hamburger Icon for mobile
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                    </button>
                </div>
                <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                    <div class="lg:flex-grow">
                        <a href="/dashboard/overview" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400 mr-4">Dashboard</a>
                        <a href="/cases" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400 mr-4">Cases</a>
                        <a href="/participants" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400 mr-4">Participants</a>
                        <a href="/filings" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400 mr-4">Filings</a>
                        <a href="/events" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400 mr-4">Events</a>
                        <a href="/reports" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400 mr-4">Reports</a>
                        <a href="/settings" class="block mt-4 lg:inline-block lg:mt-0 text-white hover:text-gray-400">Settings</a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
