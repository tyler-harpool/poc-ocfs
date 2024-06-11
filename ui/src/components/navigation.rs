use leptos::*;
use crate::components::utils::toggle_menu;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <aside class="w-full md:w-64 bg-gray-800 text-white h-screen md:fixed">
            <div class="p-4 text-center">
                <h2 class="text-2xl font-semibold">OCFS</h2>
            </div>
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
                    // Add more buttons and sublinks as needed
                </ul>
            </nav>
        </aside>
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
