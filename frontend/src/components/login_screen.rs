use leptos::*;

#[component]
pub fn LoginScreen(on_login: Callback<String>) -> impl IntoView {
    let username = create_rw_signal(String::new());
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if !username.get().is_empty() {
            on_login(username.get());
        }
    };
    
    view! {
        <div class="login-container">
            <h1>"Welcome to Data Logger"</h1>
            <p>"Please enter your username to continue"</p>
            
            <form on:submit=handle_submit>
                <div class="input-group">
                    <label for="username">"Username:"</label>
                    <input 
                        id="username"
                        type="text"
                        prop:value=move || username.get()
                        on:input=move |ev| username.set(event_target_value(&ev))
                        required
                    />
                </div>
                <button type="submit">"Login"</button>
            </form>
        </div>
    }
}