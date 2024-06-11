use leptos::*;

#[component]
pub fn CardList() -> impl IntoView {
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
