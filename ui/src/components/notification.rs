use leptos::*;

#[component]
pub fn Notification(message: String, kind: String, on_close: impl Fn() -> () + 'static) -> impl IntoView {
    let (bg_color, text_color) = match kind.as_str() {
        "success" => ("bg-green-500".to_string(), "text-white".to_string()),
        "error" => ("bg-red-500".to_string(), "text-white".to_string()),
        "info" => ("bg-blue-500".to_string(), "text-white".to_string()),
        _ => ("bg-gray-500".to_string(), "text-white".to_string()),
    };

    view! {
        <div class=format!("fixed top-4 right-4 p-4 rounded shadow-lg {} {}", bg_color, text_color)>
            <div class="flex justify-between items-center">
                <span>{message}</span>
                <button on:click=move |_| on_close() class="ml-4">X</button>
            </div>
        </div>
    }
}
