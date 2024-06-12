use leptos::*;

#[component]
pub fn Modal(title: &str, content: View, on_close: impl Fn() -> () + 'static) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-gray-800 bg-opacity-75 flex items-center justify-center z-50">
            <div class="bg-white p-6 rounded-lg shadow-lg w-full max-w-md mx-auto">
                <div class="flex justify-between items-center mb-4">
                    <h2 class="text-xl font-bold">{title}</h2>
                    <button on:click=move |_| on_close() class="text-red-600">X</button>
                </div>
                <div>
                    {content}
                </div>
            </div>
        </div>
    }
}
