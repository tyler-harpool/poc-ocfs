use leptos::*;


use crate::components::card_list::CardList;
use crate::components::footer::Footer;
use crate::components::top_bar::TopBar;
use crate::components::navigation::Navigation;
use crate::components::right_bar::RightBar;


#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            <TopBar/>
            <div class="flex flex-1">
                <Navigation/>
                <div class="container mx-auto p-6 flex flex-col md:flex-row">
                    <div class="flex-1">
                        <h1 class="text-3xl font-semibold mb-4">Welcome to the <span class="highlight">Open Case Filing System</span></h1>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                            <CardList/>
                        </div>
                    </div>
                    <RightBar/>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
