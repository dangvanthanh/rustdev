use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
        >
            "Click me: "
            {count}
        </button>
        <p>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            {count.get()}
        </p>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount::mount_to_body(App)
}
