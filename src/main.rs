use hanifanrn_web::app::App;
use leptos::*;

pub fn main() {
    mount_to_body(|| {
        view! {
          <App/>
        }
    })
}
