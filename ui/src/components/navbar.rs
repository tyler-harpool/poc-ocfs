use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
         <div class="w-full md:w-1/4 bg-gray-800 p-4 hidden md:block " id="sidebar">
        <div class="mt-8 space-y-6 mr-2 ">
        <div>
            <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Upcoming Hearings"</h2>
            <ul class="mt-4 text-gray-400 ">
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-001: Hearing on June 15"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-002: Hearing on June 20"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-003: Hearing on June 25"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-004: Hearing on June 30"</li>
            </ul>
        </div>

        <div>
            <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Pending Motions"</h2>
            <ul class="mt-4 text-gray-400">
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-005: Motion to Dismiss"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-006: Motion for Summary Judgment"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-007: Motion to Compel"</li>
            </ul>
        </div>

        <div>
            <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Recent Filings"</h2>
            <ul class="mt-4 text-gray-400">
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-008: Plaintiff&apos;s Response"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-009: Defendant&apos;s Answer"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-010: Amended Complaint"</li>
            </ul>
        </div>

        <div>
            <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Courtroom Assignments"</h2>
            <ul class="mt-4 text-gray-400">
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-011: Courtroom 3B"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-012: Courtroom 2A"</li>
                <li class="mb-2 hover:text-white transition-colors duration-300">"Case 2024-013: Courtroom 1C"</li>
            </ul>
        </div>


        <div class="mt-8 flex items-center">
            <img src="https://via.placeholder.com/40" class="w-10 h-10 rounded-full mr-4" alt="User profile picture" />
            <div>
                <p class="text-sm font-semibold">"Tyler"</p>
                <p class="text-xs text-gray-400">"Tyler@example.com"</p>
            </div>
        </div>
        
    </div>
</div>

    }
}
