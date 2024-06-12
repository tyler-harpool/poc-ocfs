// use crate::api;
// use leptos::*;
//
// #[server(FetchStories, "/api")]
// pub async fn fetch_stories(
//     story_type: String,
//     page: usize,
// ) -> Result<Vec<api::Story>, ServerFnError> {
//     let path = format!("{}?page={}", category(&story_type), page);
//     Ok(api::fetch_api::<Vec<api::Story>>(&api::story(&path))
//         .await
//         .unwrap_or_default())
// }