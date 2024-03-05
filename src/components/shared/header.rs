use crate::components::shared::navigation_bar::NavigationBar;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
          //header
          <header class="header">
            <h1 class="header-title">
              <span class="c1">Hanifan | </span> <span class="c2">Home</span>
            </h1>
            <p class="header-label">Software Engineer</p>
            <NavigationBar/>
          </header>
    }
}
