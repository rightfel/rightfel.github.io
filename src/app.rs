use crate::components::shared::{
    footer::Footer, header::Header, main_frame::MainFrame, theme_button::ThemeButton,
};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        //web frame
        <div class="site-frame">
          // header main section
          <section class="header-main-section">
            <Header/>
            <MainFrame/>
          </section>
          //footer
          <Footer/>
        </div>

        <ThemeButton/>
    }
}
