use leptos::*;





#[component]
pub fn Sidebar() -> impl IntoView {

    view! {
        <aside id="sidebar" class="w-full lg:w-64 bg-gray-800 h-full text-white">
        <div class="p-4 text-center">
            <h2 class="text-2xl font-semibold ">Home</h2>
        </div>
        <div class="p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto">
        <nav class="flex flex-col space-y-4 text-white text-sm">
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h18a2 2 0 012 2v16a2 2 0 01-2 2H3a2 2 0 01-2-2V5a2 2 0 012-2z"></path>
                </svg>
                "Dashboard"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h10M7 16h10M5 6v14m14-14v14M3 4h18"></path>
                </svg>
                "Cases"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4h16v16H4z"></path>
                </svg>
                "Documents"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v1M8 4v1M16 4v1M12 15v1M8 15v1M16 15v1M4 8h16M4 11h16M4 14h16"></path>
                </svg>
                "Scheduling"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4v16h18V4H3zm9 14H8v-4h4v4zm0-6H8V8h4v4zm6 6h-4v-4h4v4zm0-6h-4V8h4v4z"></path>
                </svg>
                "Reports & Analytics"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7h18M5 10v10h14V10M9 15h6"></path>
                </svg>
                "User Management"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 5.636a9 9 0 11-12.728 0A9 9 0 0118.364 5.636z"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2v4M12 18v4M2 12h4m12 0h4M16.95 7.05l2.828 2.828M7.05 16.95l-2.828-2.828"></path>
                </svg>
                "Support"
            </a>
            <a href="#" class="flex items-center text-gray-400 hover:text-white">
                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7v4M5 7v4M7 7h4M13 7h4M9 21V8M15 21V8M3 3h18a2 2 0 012 2v16a2 2 0 01-2 2H3a2 2 0 01-2-2V5a2 2 0 012-2z"></path>
                </svg>
                "Changelog"
            </a>
        </nav>
    </div>
        
    </aside>
    }
}

