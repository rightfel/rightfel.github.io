use crate::components::module::{home::Home, projects::Projects, resume::Resume};
use leptos::*;
use leptos_router::{Route, Routes};

#[component]
pub fn MainFrame() -> impl IntoView {
    view! {
      <Routes>
      <Route
          path=""
          view=|| view! { <Home/> }
      />
      <Route
          path="resume"
          view=|| view! { <Resume/> }
      />
      <Route
          path="projects"
          view=|| view! { <Projects/> }
      />
      </Routes>
    }
}
