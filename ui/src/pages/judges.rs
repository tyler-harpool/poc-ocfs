use crate::api::judges::get::{self, Judges};
use leptos::*;
use leptos_router::*;
use chrono::Utc;
use chrono::DateTime;

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
                        <div class="min-h-screen flex flex-col bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500">
                            <div class="flex justify-between items-center mb-4">
                                <h3 class="text-lg font-semibold">"Judge Information"</h3>
                                <a href="/judges" class="text-cyan-500">"Go Back"</a>
                            </div>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 flex-grow">
                                <div class="bg-gray-900 p-4 rounded-lg shadow-md">
                                    <h4 class="text-lg font-semibold mb-2">"Personal Information"</h4>
                                    <p>
                                        <strong>"Name: "</strong>
                                        {format!("{} {} {}", judge.first_name.unwrap_or_default(), judge.middle_name.unwrap_or_default(), judge.last_name.unwrap_or_default())}
                                    </p>
                                    <p><strong>"Gender: "</strong>{judge.gender.unwrap_or_default()}</p>
                                    <p><strong>"Birth City: "</strong>{judge.birth_city.unwrap_or_default()}</p>
                                    <p><strong>"Birth State: "</strong>{judge.birth_state.unwrap_or_default()}</p>
                                    <p><strong>"Birth Year: "</strong>{judge.birth_year.unwrap_or_default()}</p>
                                    <p><strong>"Race/Ethnicity: "</strong>{judge.race_or_ethnicity.unwrap_or_default()}</p>
                                </div>
                                <div class="bg-gray-900 p-4 rounded-lg shadow-md">
                                    <h4 class="text-lg font-semibold mb-2">"Professional Information"</h4>
                                    <p><strong>"ABA Rating: "</strong>{judge.aba_rating_1.unwrap_or_default()}</p>
                                    <p><strong>"Appointing President: "</strong>{judge.appointing_president_1.unwrap_or_default()}</p>
                                    <p><strong>"Appointment Title: "</strong>{judge.appointment_title_1.unwrap_or_default()}</p>
                                    <p><strong>"Ayes/Nays: "</strong>{judge.ayes_nays_1.unwrap_or_default()}</p>
                                    <p><strong>"Commission Date: "</strong>{format_option_date(judge.commission_date_1)}</p>
                                    <p><strong>"Committee Action Date: "</strong>{format_option_date(judge.committee_action_date_1)}</p>
                                    <p><strong>"Committee Referral Date: "</strong>{format_option_date(judge.committee_referral_date_1)}</p>
                                    <p><strong>"Confirmation Date: "</strong>{format_option_date(judge.confirmation_date_1)}</p>
                                </div>
                                <div class="bg-gray-900 p-4 rounded-lg shadow-md">
                                    <h4 class="text-lg font-semibold mb-2">"Court Information"</h4>
                                    <p><strong>"Court Name: "</strong>{judge.court_name_1.unwrap_or_default()}</p>
                                    <p><strong>"Court Type: "</strong>{judge.court_type_1.unwrap_or_default()}</p>
                                </div>
                                <div class="bg-gray-900 p-4 rounded-lg shadow-md">
                                    <h4 class="text-lg font-semibold mb-2">"Education"</h4>
                                    <p>
                                        <strong>"Degrees: "</strong>
                                        {judge.degree_1.clone().unwrap_or_else(|| "N/A".to_string())} 
                                        {judge.degree_year_1.clone().unwrap_or_else(|| "N/A".to_string())}
                                         
                                        {judge.degree_2.clone().unwrap_or_else(|| "N/A".to_string())}
                                        {judge.degree_year_2.clone().unwrap_or_else(|| "N/A".to_string())}
                                    </p>
                                    <p><strong>"Schools: "</strong>{judge.school_1.unwrap_or_default()}, {judge.school_2.unwrap_or_default()}</p>
                                </div>
                                <div class="bg-gray-900 p-4 rounded-lg shadow-md col-span-1 md:col-span-2">
                                    <h4 class="text-lg font-semibold mb-2">"Professional Career"</h4>
                                    <p>{judge.professional_career.unwrap_or_default()}</p>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}

fn format_option_date(date: Option<DateTime<Utc>>) -> String {
    date.map_or("N/A".to_string(), |d| d.format("%Y-%m-%d").to_string())
}
