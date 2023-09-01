
mod components;
mod pages;
mod router;
mod app;
mod types;


use app::App;
fn main() {
    yew::start_app::<App>();
}