use leptos::*;
use components::app::App;

mod components;
mod models;
mod services;

fn main() {
    mount_to_body(|| view! { <App /> })
}