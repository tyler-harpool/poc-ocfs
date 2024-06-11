use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::{window, Document, Element};

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
                <Sidebar/>
                <div class="flex-1 p-6 ml-0 md:ml-64">
                    <Ocfs/>
                    <CardList/>
                    <RightBar/>
                    <Footer/>
                </div>
            </div>
        </body>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside class="w-full md:w-64 bg-gray-800 text-white h-screen md:fixed">
            <MainContent/>
            <nav>
                <ul>
                    {sidebar_button("Dashboard", "dashboard-menu", "fas fa-tachometer-alt", vec![
                        sidebar_sublink("dashboard/overview.html", "fas fa-chart-line", "Overview"),
                        sidebar_sublink("dashboard/statistics.html", "fas fa-chart-bar", "Statistics"),
                    ])}
                    {sidebar_button("Cases", "cases-menu", "fas fa-folder-open", vec![
                        sidebar_sublink("cases/all.html", "fas fa-folder", "All Cases"),
                        sidebar_sublink("cases/create.html", "fas fa-plus", "Create New Case"),
                        sidebar_sublink("cases/types.html", "fas fa-list-alt", "Case Types"),
                    ])}
                    {sidebar_button("Documents", "documents-menu", "fas fa-file-alt", vec![
                        sidebar_sublink("documents/all.html", "fas fa-file", "All Documents"),
                        sidebar_sublink("documents/upload.html", "fas fa-upload", "Upload Document"),
                        sidebar_sublink("documents/types.html", "fas fa-file-alt", "Document Types"),
                    ])}
                    {sidebar_button("Scheduling", "scheduling-menu", "fas fa-calendar-alt", vec![
                        sidebar_sublink("scheduling/calendar.html", "fas fa-calendar", "Calendar"),
                        sidebar_sublink("scheduling/deadlines.html", "fas fa-hourglass-half", "Deadlines"),
                        sidebar_sublink("scheduling/reminders.html", "fas fa-bell", "Reminders"),
                    ])}
                    {sidebar_button("Reports & Analytics", "reports-menu", "fas fa-chart-pie", vec![
                        sidebar_sublink("reports/generate.html", "fas fa-file-alt", "Generate Report"),
                        sidebar_sublink("reports/analytics.html", "fas fa-chart-line", "View Analytics"),
                    ])}
                    {sidebar_button("User Management", "user-management-menu", "fas fa-users-cog", vec![
                        sidebar_sublink("user-management/all.html", "fas fa-users", "All Users"),
                        sidebar_sublink("user-management/add.html", "fas fa-user-plus", "Add New User"),
                        sidebar_sublink("user-management/roles.html", "fas fa-user-shield", "Roles & Permissions"),
                    ])}
                </ul>
            </nav>
        </aside>
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
    let cards = vec![
        card_item("dashboard/overview.html", "fas fa-tachometer-alt text-2xl text-blue-500", "Dashboard Overview", "Get a quick overview of your case activities, upcoming deadlines, and recent updates."),
        card_item("cases/all.html", "fas fa-briefcase text-2xl text-green-500", "Case Management", "Efficiently manage cases, track progress, and ensure timely updates."),
        card_item("documents/all.html", "fas fa-file-alt text-2xl text-yellow-500", "Document Management", "Organize and manage all your legal documents securely in one place."),
        card_item("scheduling/calendar.html", "fas fa-calendar-alt text-2xl text-red-500", "Scheduling", "Keep track of court schedules, hearings, and deadlines with real-time updates and notifications."),
        card_item("reports/analytics.html", "fas fa-chart-line text-2xl text-purple-500", "Reports & Analytics", "Generate comprehensive reports and gain actionable insights with advanced analytics tools."),
        card_item("user-management/all.html", "fas fa-users text-2xl text-teal-500", "User Management", "Manage user roles and permissions, and ensure secure access to sensitive information."),
        card_item("collaboration/tools.html", "fas fa-comments text-2xl text-pink-500", "Collaboration Tools", "Enhance collaboration among legal teams with integrated communication and file-sharing tools."),
        card_item("ai/insights.html", "fas fa-brain text-2xl text-indigo-500", "AI-Powered Insights", "Leverage AI to uncover patterns, predict outcomes, and support decision-making processes."),
        card_item("storage/cloud.html", "fas fa-cloud text-2xl text-blue-500", "Secure Cloud Storage", "Ensure the security and accessibility of your data with our robust cloud storage solutions."),
    ];

    view! {
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {cards.into_iter().map(|card| card).collect_view()}
        </div>
    }
}

fn card_item(href: &str, icon_class: &str, title: &str, description: &str) -> impl IntoView {
    let href = href.to_string();
    let icon_class = icon_class.to_string();
    let title = title.to_string();
    let description = description.to_string();
    view! {
        <div class="p-4 bg-gray-50 rounded-lg shadow card">
            <a href={href}>
                <div class="flex items-center mb-2">
                    <i class={icon_class}></i>
                    <h3 class="text-lg font-medium">{title}</h3>
                </div>
                <p>{description}</p>
            </a>
        </div>
    }
}

#[component]
fn RightBar() -> impl IntoView {
    view! {
        <div class="space-y-6">
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
        </div>
        
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white py-6 mt-8">
            <div class="container mx-auto flex flex-col md:flex-row justify-between items-center">
                <div>
                    <h2 class="text-2xl font-semibold">Open Case Filing System</h2>
                    <p class="text-gray-400">Enhancing judicial efficiency through technology</p>
                </div>
                <div class="flex space-x-4">
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300"><i class="fab fa-facebook-f"></i></a>
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300"><i class="fab fa-twitter"></i></a>
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300"><i class="fab fa-linkedin-in"></i></a>
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300"><i class="fab fa-github"></i></a>
                </div>
            </div>
            <div class="text-center text-gray-500 mt-4">
                &copy; 2024 Open Case Filing System. All rights reserved.
            </div>
        </footer>
    }
}

fn sidebar_button(title: &str, menu_id: &str, icon_class: &str, sublinks: Vec<impl IntoView>) -> impl IntoView {
    let title = title.to_string();
    let menu_id = menu_id.to_string();
    let icon_class = icon_class.to_string();
    let menu_id_clone = menu_id.clone(); // Clone here to avoid move issue
    view! {
        <li>
            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" on:click=move |_| toggle_menu(&menu_id_clone)>
                <i class={icon_class.clone()}></i> {title.clone()}
            </button>
            <ul id={menu_id} class="hidden-submenu pl-8">
                {sublinks.into_iter().map(|sublink| sublink).collect_view()}
            </ul>
        </li>
    }
}

fn sidebar_sublink(href: &str, icon_class: &str, title: &str) -> impl IntoView {
    let href = href.to_string();
    let icon_class = icon_class.to_string();
    let title = title.to_string();
    view! {
        <li class="sublink py-1">
            <a href={href}>
                <i class={icon_class}></i> {title}
            </a>
        </li>
    }
}

fn toggle_menu(menu_id: &str) {
    let document = window().unwrap().document().unwrap();
    if let Some(menu) = document.get_element_by_id(menu_id) {
        if menu.class_list().contains("hidden-submenu") {
            menu.class_list().remove_1("hidden-submenu").unwrap();
            menu.class_list().add_1("show-submenu").unwrap();
        } else {
            menu.class_list().remove_1("show-submenu").unwrap();
            menu.class_list().add_1("hidden-submenu").unwrap();
        }
    }
}
