use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ui.css"/>
        <Title text="Welcome to Open Case Filing System"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Welcome to Open Case Filing System"/>
        <body class="bg-gray-100">
            <div class="flex">
                <aside class="w-full md:w-64 bg-gray-800 text-white h-screen md:fixed">
                    <MainContent/>
                    <nav>
                        <ul>
                            <li>
                                <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700">
                                    <i class="fas fa-tachometer-alt mr-2"></i> Dashboard
                                </button>
                                <ul id="dashboard-menu" class="hidden-submenu pl-8">
                                    <li class="sublink py-1"><a href="dashboard/overview.html"><i class="fas fa-chart-line mr-2"></i> Overview</a></li>
                                    <li class="sublink py-1"><a href="dashboard/statistics.html"><i class="fas fa-chart-bar mr-2"></i> Statistics</a></li>
                                </ul>
                            </li>
                            <li>
                                <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700">
                                    <i class="fas fa-folder-open mr-2"></i> Cases
                                </button>
                                <ul id="cases-menu" class="hidden-submenu pl-8">
                                    <li class="sublink py-1"><a href="cases/all.html"><i class="fas fa-folder mr-2"></i> All Cases</a></li>
                                    <li class="sublink py-1"><a href="cases/create.html"><i class="fas fa-plus mr-2"></i> Create New Case</a></li>
                                    <li class="sublink py-1"><a href="cases/types.html"><i class="fas fa-list-alt mr-2"></i> Case Types</a></li>
                                </ul>
                            </li>
                            <li>
                                <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700">
                                    <i class="fas fa-file-alt mr-2"></i> Documents
                                </button>
                                <ul id="documents-menu" class="hidden-submenu pl-8">
                                    <li class="sublink py-1"><a href="documents/all.html"><i class="fas fa-file mr-2"></i> All Documents</a></li>
                                    <li class="sublink py-1"><a href="documents/upload.html"><i class="fas fa-upload mr-2"></i> Upload Document</a></li>
                                    <li class="sublink py-1"><a href="documents/types.html"><i class="fas fa-file-alt mr-2"></i> Document Types</a></li>
                                </ul>
                            </li>
                            <li>
                                <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700">
                                    <i class="fas fa-calendar-alt mr-2"></i> Scheduling
                                </button>
                                <ul id="scheduling-menu" class="hidden-submenu pl-8">
                                    <li class="sublink py-1"><a href="scheduling/calendar.html"><i class="fas fa-calendar mr-2"></i> Calendar</a></li>
                                    <li class="sublink py-1"><a href="scheduling/deadlines.html"><i class="fas fa-hourglass-half mr-2"></i> Deadlines</a></li>
                                    <li class="sublink py-1"><a href="scheduling/reminders.html"><i class="fas fa-bell mr-2"></i> Reminders</a></li>
                                </ul>
                            </li>
                            <li>
                                <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700">
                                    <i class="fas fa-chart-pie mr-2"></i> Reports & Analytics
                                </button>
                                <ul id="reports-menu" class="hidden-submenu pl-8">
                                    <li class="sublink py-1"><a href="reports/generate.html"><i class="fas fa-file-alt mr-2"></i> Generate Report</a></li>
                                    <li class="sublink py-1"><a href="reports/analytics.html"><i class="fas fa-chart-line mr-2"></i> View Analytics</a></li>
                                </ul>
                            </li>
                            <li>
                                <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700">
                                    <i class="fas fa-users-cog mr-2"></i> User Management
                                </button>
                                <ul id="user-management-menu" class="hidden-submenu pl-8">
                                    <li class="sublink py-1"><a href="user-management/all.html"><i class="fas fa-users mr-2"></i> All Users</a></li>
                                    <li class="sublink py-1"><a href="user-management/add.html"><i class="fas fa-user-plus mr-2"></i> Add New User</a></li>
                                    <li class="sublink py-1"><a href="user-management/roles.html"><i class="fas fa-user-shield mr-2"></i> Roles & Permissions</a></li>
                                </ul>
                            </li>
                        </ul>
                    </nav>
                </aside>
                <div class="flex-1 p-6 ml-0 md:ml-64">
                    <Ocfs/>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                        <CardList/>
                    </div>
                    <aside class="space-y-6">
                        <RightBar/>
                    </aside>
                </div>
            </div>
        </body>
    }
}

#[component]
fn MainContent() -> impl IntoView {
    view! {
        <div class="p-4 text-center">
            <h2 class="text-2xl font-semibold">OCFS</h2>
        </div>
    }
}

#[component]
fn Ocfs() -> impl IntoView {
    view! {
        <h1 class="text-3xl font-semibold mb-4">Welcome to the <span class="highlight">Open Case Filing System</span></h1>
    }
}

#[component]
fn CardList() -> impl IntoView {
    view! {
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="dashboard/overview.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-tachometer-alt text-2xl text-blue-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Dashboard Overview</h3>
                </div>
                <p>Get a quick overview of your case activities, upcoming deadlines, and recent updates.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="cases/all.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-briefcase text-2xl text-green-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Case Management</h3>
                </div>
                <p>Efficiently manage cases, track progress, and ensure timely updates.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="documents/all.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-file-alt text-2xl text-yellow-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Document Management</h3>
                </div>
                <p>Organize and manage all your legal documents securely in one place.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="scheduling/calendar.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-calendar-alt text-2xl text-red-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Scheduling</h3>
                </div>
                <p>Keep track of court schedules, hearings, and deadlines with real-time updates and notifications.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="reports/analytics.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-chart-line text-2xl text-purple-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Reports & Analytics</h3>
                </div>
                <p>Generate comprehensive reports and gain actionable insights with advanced analytics tools.</p>
                <canvas id="analyticsChart"></canvas>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="user-management/all.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-users text-2xl text-teal-500 mr-2"></i>
                    <h3 class="text-lg font-medium">User Management</h3>
                </div>
                <p>Manage user roles and permissions, and ensure secure access to sensitive information.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="collaboration/tools.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-comments text-2xl text-pink-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Collaboration Tools</h3>
                </div>
                <p>Enhance collaboration among legal teams with integrated communication and file-sharing tools.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="ai/insights.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-brain text-2xl text-indigo-500 mr-2"></i>
                    <h3 class="text-lg font-medium">AI-Powered Insights</h3>
                </div>
                <p>Leverage AI to uncover patterns, predict outcomes, and support decision-making processes.</p>
            </a>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href="storage/cloud.html">
                <div class="flex items-center mb-2">
                    <i class="fas fa-cloud text-2xl text-blue-500 mr-2"></i>
                    <h3 class="text-lg font-medium">Secure Cloud Storage</h3>
                </div>
                <p>Ensure the security and accessibility of your data with our robust cloud storage solutions.</p>
            </a>
        </div>
    }
}

#[component]
fn RightBar() -> impl IntoView {
    view! {
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

        <section class="bg-gray-50 p-4 rounded-lg shadow fade-in hidden" id="user-section">
            <h2 class="text-2xl font-semibold mb-4"><span class="highlight">Hello,</span> <span id="user-name" class="important"></span></h2>
        </section>

        <section class="bg-gray-50 p-4 rounded-lg shadow fade-in hidden" id="notifications-section">
            <h2 class="text-2xl font-semibold mb-4"><span class="highlight">Recent Notifications</span></h2>
            <ul class="divide-y divide-gray-200">
                <li class="py-2 bg-white shadow-sm rounded-md p-4 mb-2">New case <strong class="important">Akira Harrison v. The State of Texas</strong> added.</li>
                <li class="py-2 bg-white shadow-sm rounded-md p-4 mb-2">Document <strong class="important">Motion to Dismiss</strong> filed in case <strong class="important">Elsie Torres Franco v. The State of Texas</strong>.</li>
                <li class="py-2 bg-white shadow-sm rounded-md p-4 mb-2">User <strong class="important">Jane Doe</strong> granted access to <strong>Case Management</strong>.</li>
            </ul>
        </section>

        <section class="bg-gray-50 p-4 rounded-lg shadow fade-in hidden" id="stats-section">
            <h2 class="text-2xl font-semibold mb-4"><span class="highlight">Quick Stats</span></h2>
            <div class="grid grid-cols-1 gap-4">
                <div class="p-4 bg-white rounded-lg shadow">
                    <div class="flex items-center mb-2">
                        <i class="fas fa-briefcase text-2xl text-blue-500 mr-2"></i>
                        <h3 class="text-lg font-medium">Cases Opened</h3>
                    </div>
                    <div class="relative pt-1">
                        <div class="flex mb-2 items-center justify-between">
                            <div>
                                <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-blue-600 bg-blue-200">
                                    75%
                                </span>
                            </div>
                            <div class="text-right">
                                <span class="text-xs font-semibold inline-block text-blue-600">
                                    75%
                                </span>
                            </div>
                        </div>
                        <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-blue-200">
                            <div style="width:75%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500"></div>
                        </div>
                    </div>
                </div>
                <div class="p-4 bg-white rounded-lg shadow">
                    <div class="flex items-center mb-2">
                        <i class="fas fa-check text-2xl text-green-500 mr-2"></i>
                        <h3 class="text-lg font-medium">Cases Closed</h3>
                    </div>
                    <div class="relative pt-1">
                        <div class="flex mb-2 items-center justify-between">
                            <div>
                                <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-green-600 bg-green-200">
                                    60%
                                </span>
                            </div>
                            <div class="text-right">
                                <span class="text-xs font-semibold inline-block text-green-600">
                                    60%
                                </span>
                            </div>
                        </div>
                        <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-green-200">
                            <div style="width:60%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-green-500"></div>
                        </div>
                    </div>
                </div>
                <div class="p-4 bg-white rounded-lg shadow">
                    <div class="flex items-center mb-2">
                        <i class="fas fa-file-alt text-2xl text-yellow-500 mr-2"></i>
                        <h3 class="text-lg font-medium">Documents Processed</h3>
                    </div>
                    <div class="relative pt-1">
                        <div class="flex mb-2 items-center justify-between">
                            <div>
                                <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-yellow-600 bg-yellow-200">
                                    85%
                                </span>
                            </div>
                            <div class="text-right">
                                <span class="text-xs font-semibold inline-block text-yellow-600">
                                    85%
                                </span>
                            </div>
                        </div>
                        <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-yellow-200">
                            <div style="width:85%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-yellow-500"></div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
