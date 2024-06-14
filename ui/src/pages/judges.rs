use crate::api::judges::get::{self, Judges};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Judges() -> impl IntoView {
    let params = use_params_map();
    let judge = create_resource(
        move || params.get().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                get::fetch_api::<Judges>(&get::judge(&id)).await
            }
        },
    );

    view! {
        <div class="judge-view">
            <Suspense fallback=|| view! {  "Loading..." }>
                {move || judge.get().map(|judge| match judge {
                    None => view! { <h1>"Judges not found."</h1> }.into_any(),
                    Some(judge) => view! {
                        <div>
                            <h1>"Judges: " {judge.id.map_or("N/A".to_string(), |id| id.to_string())}</h1>
                            <ul class="meta">
                                <li>
                                    <span class="label">"NID: "</span> {judge.nid.clone().unwrap_or_else(|| "N/A".to_string())}
                                </li>
                                <li>
                                    <span class="label">"JID: "</span> {judge.jid.clone().unwrap_or_else(|| "N/A".to_string())}
                                </li>
                                <li>
                                    <span class="label">"Last Name: "</span> {judge.last_name.clone().unwrap_or_else(|| "N/A".to_string())}
                                </li>
                                <li>
                                    <span class="label">"First Name: "</span> {judge.first_name.clone().unwrap_or_else(|| "N/A".to_string())}
                                </li>
                                <li>
                                    <span class="label">"Middle Name: "</span> {judge.middle_name.clone().unwrap_or_else(|| "N/A".to_string())}
                                </li>
                                <li>
                                    <span class="label">"Court Name: "</span> {judge.court_name_1.clone().unwrap_or_else(|| "N/A".to_string())}
                                </li>
                            </ul>
 
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}
