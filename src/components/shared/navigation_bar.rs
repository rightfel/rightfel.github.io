use leptos::*;
use leptos_router::Router;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view! {
      <Router>
            <nav class="header-nav" aria-label="site menu">
              <ol>
                <li>
                  <a class="_text" href="/">"Home"</a>
                </li>
                <li>
                  <a class="_text" href="resume">"Resume"</a>
                </li>
                <li>
                  <a class="_text" href="projects">"Projects"</a>
                </li>
              </ol>
            </nav>
      </Router>
    }
}
