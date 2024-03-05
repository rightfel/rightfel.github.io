use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
          <main class="main home-frame">
            //content box
            <div class="content-frame">
              <p class="content">
                Hi, I am Hanifan Rizki Nurahman.
                <br />
                <br />
                Lead Data and AI services at IBM-JTI and a Statistics graduate from the
                Padjadjaran University. My main focus these days is explore the
                possibility of generative AI and Rust web development with Axum
                and Leptos.
                <br />
                <br />
                In my free time you will find me grinding my programming skills at
                Leetcode, or write some blogs article to add some internet
                footprint in this vast and crowded world.
                <br />
                <br />
                Check out my <a class="_link" href="./resume.html">Resume</a> for
                more details.
              </p>
            </div>
          </main>
    }
}
