use crate::api::judges::get::Judges;
use leptos::*;


use reqwest::Error;



pub async fn fetch_all_judges() -> Result<Vec<Judges>, Error> {
    let response = reqwest::get("http://localhost:8000/api/judges").await?;
    let judges = response.json::<Vec<Judges>>().await?;
    Ok(judges)
}



#[component]
pub fn AllJudges() -> impl IntoView {
    let (fetch_signal, _set_fetch_signal) = create_signal(());
    let judges = create_resource(move || fetch_signal.get(), |_| async {
        fetch_all_judges().await.ok()
    });

    view! {
        <div class="all-judges-view">
            <Suspense fallback=|| view! {  "Loading..." }>
                {move || judges.get().map(|judges| match judges {
                    None => view! { <h1>"No judges found."</h1> }.into_any(),
                    Some(judges) => view! {
                        <div>
                            <h1>"All Judges"</h1>
                            <ul>
                                {judges.iter().map(|judge| view! {
                                    <li key={judge.id.unwrap_or_default()}>
                                        <h2>{judge.first_name.clone().unwrap_or_default()} {judge.last_name.clone().unwrap_or_default()}</h2>
                                        <p>
                                            <span class="label">"Court Name: "</span> {judge.court_name_1.clone().unwrap_or_default()}
                                        </p>
                                        <p>
                                            <span class="label">"Appointing President: "</span> {judge.appointing_president_1.clone().unwrap_or_default()}
                                        </p>
                                        <p>
                                            <span class="label">"Race or Ethnicity: "</span> {judge.race_or_ethnicity.clone().unwrap_or_default()}
                                        </p>
                                        <p>
                                            <span class="label">"Gender: "</span> {judge.gender.clone().unwrap_or_default()}
                                        </p>
                                        <p>
                                            <span class="label">"Professional Career: "</span> {judge.professional_career.clone().unwrap_or_default()}
                                        </p>
                                    </li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}