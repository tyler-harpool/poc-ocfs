use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
    //     <header class="mt-auto p-4 text-white border-b-4 border-cyan-500 bg-gray-800">
    //     <div class="container mx-auto flex justify-between items-center ">
    //         <h1 class="text-2xl font-semibold text-cyan-500">"Open Case Filing System"</h1>
    //         <button class="text-white md:hidden" on:click=move |_| toggle_sidebar("sidebar")>
    //             <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    //                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
    //             </svg>
    //         </button>
    //     </div>
    // </header>

                    
   

               

                // Main Content
                <div class="w-full md:w-3/4 p-8">
                    <div class="flex justify-between items-center mb-8">
                        <h2 class="text-2xl font-semibold">Welcome to Open Case Filing System, Tyler</h2>
                        <div class="relative">
                            <select class="bg-gray-700 text-white p-2 rounded focus:outline-none">
                                <option>"Last week"</option>
                                <option>"Last month"</option>
                                <option>"Last year"</option>
                            </select>
                        </div>
                    </div>

                    // search
                    <div class="mb-8 w-full">
                    <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                    <nav class="flex flex-wrap justify-center space-x-4 text-white text-sm">
                        <input id="search" type="text" class="w-full bg-gray-800 text-white p-2 rounded focus:outline-none" placeholder="Search for cases..."/>
                    </nav>
                </div>
                    </div>


                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8 ">
                    <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                        <p class="text-gray-400 text-sm">"Total cases"</p>
                        <p class="text-2xl font-bold">"1,024"</p>
                        <p class="text-green-500 text-sm mt-1">"+3.2% from last week"</p>
                    </div>
                    <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                        <p class="text-gray-400 text-sm">"Active cases"</p>
                        <p class="text-2xl font-bold">"512"</p>
                        <p class="text-red-400 text-sm mt-1">"-1.8% from last week"</p>
                    </div>
                    <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                        <p class="text-gray-400 text-sm">"Closed cases"</p>
                        <p class="text-2xl font-bold">"256"</p>
                        <p class="text-green-500 text-sm mt-1">"+2.7% from last week"</p>
                    </div>
                    <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                        <p class="text-gray-400 text-sm">"Pending cases"</p>
                        <p class="text-2xl font-bold">"256"</p>
                        <p class="text-green-500 text-sm mt-1">"+5.1% from last week"</p>
                    </div>
                </div>
                    // Second Navigation for Activity Feeds
                    <div class="mb-8">
                    <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                    <nav class="flex flex-wrap justify-center space-x-4 text-white text-sm">
                        <a href="/participants" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Recent Case Activity"</a>
                        <span>"|"</span>
                        <a href="/dashboard/overview" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Court Orders"</a>
                        <span>"|"</span>
                        <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Filing Deadlines"</a>
                        <span>"|"</span>
                        <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Hearing Schedules"</a>
                        <span>"|"</span>
                        <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Case Notifications"</a>
                        <span>"|"</span>
                        <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Attorney Notes"</a>
                        <span>"|"</span>
                        <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Document Uploads"</a>
                        <span>"|"</span>
                        <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Motions and Filings"</a>
                    </nav>
                </div>
                    </div>

 

                    <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500">
                        <h3 class="text-lg font-semibold mb-4">"Recent Case Activity"</h3>
                        <div class="overflow-x-auto">
                            <table class="min-w-full bg-gray-800 hover:table-fixed">
                                <thead>
                                    <tr>
                                        <th class="px-4 py-2 text-left text-gray-400">"Case ID"</th>
                                        <th class="px-4 py-2 text-left text-gray-400">"Date"</th>
                                        <th class="px-4 py-2 text-left text-gray-400">"Activity"</th>
                                        <th class="px-4 py-2 text-left text-gray-400">"Involved Parties"</th>
                                        <th class="px-4 py-2 text-right text-gray-400">"Status"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-001"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"May 9, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Filing of Motion"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Leslie Alexander vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Pending"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-002"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"May 5, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Initial Hearing"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Michael Foster vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Scheduled"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-003"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Apr 28, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Evidence Submission"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Dries Vincent vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Under Review"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-004"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Apr 23, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Final Judgment"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Lindsay Walton vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Closed"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-005"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Apr 18, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Pre-Trial Conference"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Courtney Henry vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Scheduled"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-006"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Apr 14, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Evidence Submission"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Tom Cook vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Under Review"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-007"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Apr 10, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Initial Hearing"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Whitney Francis vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Scheduled"</td>
                                    </tr>
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">"2024-008"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Apr 6, 2024"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Filing of Complaint"</td>
                                        <td class="border-t border-gray-700 px-4 py-2">"Leonard Krasner vs. State"</td>
                                        <td class="border-t border-gray-700 px-4 py-2 text-right">"Pending"</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
  

    }
}

