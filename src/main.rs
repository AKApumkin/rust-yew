use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <h1>{"Hello World2!"}</h1>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}