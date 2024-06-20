use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Navbar, Sidebar, Dashboard, Settings, Footer};
use crate::pages::judges::id::Id as JudgeId;
use crate::pages::judges::list::List as JudgeList;
use crate::pages::home::HomePage;
use crate::pages::login::Login;
use crate::pages::participant::Participant;
use crate::pages::test::Test;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ui.css"/>
        <Title text="OCFS - Welcome to Open Case Filing System"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>

        <div class="bg-gray-900 bg-gradient-to-r from-cyan-800 to-blue-900">


        <div class="flex">
        <div class="flex flex-col min-h-screen text-white ">
            <div class="flex flex-1  divide-x divide-cyan-500 divide-dashed">

                // Main Navigation

        <Sidebar />
        <Navbar />


                    <Routes>
                        <Route path="/" view=Login />
                        <Route path="/dashboard/overview" view=HomePage />
                        <Route path="/test" view=Test />
                        // <Route path="/cases" view=CaseList />
                        // <Route path="/cases/:id" view=CaseDetails />
                        <Route path="/participants" view=Participant />
                        <Route path="/events" view=Dashboard />
                        <Route path="/reports" view=Dashboard />
                        <Route path="/settings" view=Settings />
                         <Route path="/judges/:id" view=JudgeId />
                         <Route path="/judges" view=JudgeList />
                    </Routes>
            



            </div>
            </div>
            </div>
        <Footer />

        </div>
        </Router>
    }
}
