use leptos::*;

use crate::components::utils::toggle_menu;

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
