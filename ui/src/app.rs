use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::HomePage;
use crate::pages::test::Test;
use crate::pages::participant::Participant;
use crate::pages::login::Login;
use crate::pages::judges::Judges;
use crate::pages::all_judges::AllJudges;
use crate::components::{Navbar, Sidebar};
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ui.css"/>
        <Title text="OCFS - Welcome to Open Case Filing System"/>
        

        <body class="bg-gray-900 bg-gradient-to-r from-cyan-800 to-blue-900">
        
    
        <div class="flex">
        <div class="flex flex-col min-h-screen text-white ">
            <div class="flex flex-1  divide-x divide-cyan-500 divide-dashed">
         
                // Main Navigation
               
        <Sidebar />
        <Navbar />
 
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
                        <Route path="/test" view=Test />
                        // <Route path="/cases" view=CaseList />
                        // <Route path="/cases/:id" view=CaseDetails />
                        <Route path="/participants" view=Participant />
                            // <Route path="/island" view=Island />
                        // <Route path="/filings" view=FilingsList />
                        // <Route path="/events" view=EventsCalendar />
                        // <Route path="/reports" view=ReportsList />
                        // <Route path="/settings" view=Settings />
                         <Route path="/judges/:id" view=Judges />
                         <Route path="/judges" view=AllJudges />
                    </Routes>
                </Router>
            
         
            
            </div>
            </div>
            </div>
            <footer class="bg-gray-800 text-white py-6 mt-auto w-full border-t-4 border-cyan-500">
            <div class="container mx-auto flex flex-col md:flex-row justify-between items-center">
                <div>
                    <h2 class="text-2xl font-semibold">Open Case Filing System</h2>
                    <p class="text-gray-400">Enhancing judicial efficiency through technology</p>
                </div>
                <div class="flex space-x-4">
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Facebook</a>
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Twitter</a>
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Linkedin</a>
                    <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Github</a>
                </div>
            </div>
            <div class="text-center text-gray-500 mt-4">
                &copy; 2024 Open Case Filing System. All rights reserved.
            </div>
        </footer>
        
        </body>      
    }
}
