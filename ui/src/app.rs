use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::HomePage;
use crate::pages::test::Test;
use crate::pages::participant::Participant;
use crate::pages::login::Login;
// use crate::components::{Navbar, Sidebar, Footer, TopBar};
use crate::components::utils::toggle_menu;
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ui.css"/>
        <Title text="OCFS - Welcome to Open Case Filing System"/>
        

        <body class="bg-gray-900 bg-gradient-to-r from-cyan-800 to-blue-900">
        
    
        // <TopBar />
        // <Sidebar />
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate outside_errors/>
                    }
                    .into_view()
                }>
                    <Routes>
                        <Route path="/" view=Login />
                        <Route path="/dashboard/overview" view=HomePage />
                        // <Route path="/cases" view=CaseList />
                        // <Route path="/cases/:id" view=CaseDetails />
                        <Route path="/participants" view=Participant />
                        // <Route path="/filings" view=FilingsList />
                        // <Route path="/events" view=EventsCalendar />
                        // <Route path="/reports" view=ReportsList />
                        // <Route path="/settings" view=Settings />
                    </Routes>
                </Router>
            
         

        
        </body>      
    }
}
