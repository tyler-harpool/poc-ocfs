use web_sys::window;

pub fn toggle_menu(menu_id: &str) {
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
