
use crate::api::judges::models::Judges;
use leptos::*;
use reqwest::Error;
#[allow(unused_imports)]
use web_sys::window;




pub async fn fetch_all_judges() -> Result<Vec<Judges>, Error> {
    let response = reqwest::get("http://localhost:8000/api/judges").await?;
    let judges = response.json::<Vec<Judges>>().await?;
    Ok(judges)
}

#[cfg(not(feature = "ssr"))]
fn local_storage_get(key: &str) -> Option<String> {
    window()
        .and_then(|win| win.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(key).ok().flatten())
}

#[cfg(not(feature = "ssr"))]
fn local_storage_set(key: &str, value: &str) {
    if let Some(storage) = window().and_then(|win| win.local_storage().ok().flatten()) {
        let _ = storage.set_item(key, value);
    }
}

#[component]
pub fn AllJudges() -> impl IntoView {
    let (fetch_signal, _set_fetch_signal) = create_signal(());
    let judges = create_resource(move || fetch_signal.get(), |_| async {
        fetch_all_judges().await.ok()
    });

    #[cfg(not(feature = "ssr"))]
    let initial_filter_query = local_storage_get("filter_query").unwrap_or_default();

    #[cfg(feature = "ssr")]
    let initial_filter_query = String::new();

    let (filter_query, set_filter_query) = create_signal(initial_filter_query);

    // Save filter query to local storage whenever it changes
    #[cfg(not(feature = "ssr"))]
    {
        create_effect(move |_| {
            let query = filter_query.get();
            local_storage_set("filter_query", &query);
        });
    }

    view! {
        <div class="all-judges-view p-4">
            <input
                type="text"
                class="bg-gray-900 text-white p-2 mb-4 rounded w-full"
                placeholder="Search by name, court, or appointing president"
                value={move || filter_query.get()}
                on:input=move |ev| set_filter_query(event_target_value(&ev))
            />
            <Suspense fallback=|| view! { "Loading..." }>
                {move || judges.get().map(move |judges| match judges {
                    None => view! { <h1>"No judges found."</h1> }.into_any(),
                    Some(judges) => {
                        let query = filter_query.get().to_lowercase();
                        let keywords: Vec<&str> = query.split_whitespace().collect();

                        let filtered_judges = judges.iter().filter(|judge| {
                            keywords.iter().all(|&keyword| {
                                judge.first_name.clone().unwrap_or_default().to_lowercase().contains(keyword) ||
                                judge.middle_name.clone().unwrap_or_default().to_lowercase().contains(keyword) ||
                                judge.last_name.clone().unwrap_or_default().to_lowercase().contains(keyword) ||
                                judge.court_name_1.clone().unwrap_or_default().to_lowercase().contains(keyword) ||
                                judge.appointing_president_1.clone().unwrap_or_default().to_lowercase().contains(keyword)
                            })
                        }).collect::<Vec<_>>();

                        view! {
                            <ul class="space-y-6">
                                {filtered_judges.into_iter().map(|judge| view! {
                                    <li key={judge.id.unwrap_or_default()} class="bg-gray-900 p-6 rounded-lg shadow-md border-2 border-cyan-500 transition hover:border-pink-500">
                                    <a href={format!("/judges/{}", judge.id.unwrap_or_default())} class="block">
                                        <div class="flex items-center mb-4">
                                            <h2 class="text-xl font-bold text-white">
                                                {judge.first_name.clone().unwrap_or_default()}{" "}
                                                {judge.middle_name.clone().unwrap_or_default()}{" "}
                                                {judge.last_name.clone().unwrap_or_default()}
                                            </h2>
                                        </div>
                                        <p class="text-gray-300">
                                            <span class="font-semibold text-gray-400">"Court Name: "</span>
                                            <a href={format!("/judges/court/{}", judge.court_name_1.clone().unwrap_or_default())} class="text-cyan-500 hover:text-pink-500">
                                                {judge.court_name_1.clone().unwrap_or_default()}
                                            </a>
                                        </p>
                                        <p class="text-gray-300">
                                            <span class="font-semibold text-gray-400">"Appointing President: "</span>
                                            <a href={format!("/judges/president/{}", judge.appointing_president_1.clone().unwrap_or_default())} class="text-cyan-500 hover:text-pink-500">
                                                {judge.appointing_president_1.clone().unwrap_or_default()}
                                            </a>
                                        </p>
                                        <p class="text-gray-300">
                                            <span class="font-semibold text-gray-400">"Nomination Date: "</span>
                                            <a href={format!("http://localhost:8000/api/judges/nomination_date_range?start_date={}&end_date={}", judge.nomination_date_1.map_or("".to_string(), |d| d.format("%Y-%m-%d").to_string()), judge.nomination_date_1.map_or("".to_string(), |d| d.format("%Y-%m-%d").to_string()))} class="text-cyan-500 hover:text-pink-500">
                                                {judge.nomination_date_1.map_or("N/A".to_string(), |date| date.format("%Y-%m-%d").to_string())}
                                            </a>
                                        </p>
                                        <p class="text-gray-300">
                                            <span class="font-semibold text-gray-400">"Gender: "</span>
                                            <a href={format!("/judges/gender/{}", judge.gender.clone().unwrap_or_default())} class="text-cyan-500 hover:text-pink-500">
                                                {judge.gender.clone().unwrap_or_default()}
                                            </a>
                                        </p>
                                    </a>
                                    </li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}
