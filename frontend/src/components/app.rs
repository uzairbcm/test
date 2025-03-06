use std::sync::Arc;

use super::data_entry_screen::DataEntryScreen;
use super::login_screen::LoginScreen;
use crate::models::user_state::UserState;
use crate::services::api_service::ApiService;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    // Main state signals
    let is_logged_in = RwSignal::new(false);
    let current_state = RwSignal::new(UserState::default());
    let api_service: Arc<ApiService> = Arc::new(ApiService::new());

    // Login logic
    let api_service_login = Arc::clone(&api_service);
    let handle_login = Callback::new(move |username: String| {
        let username_clone = username.clone();
        let api = Arc::clone(&api_service_login);  // Clone the one owned by this closure
        spawn_local(async move {
            let mut state = UserState::default();
            state.username = username_clone.clone();

            // Try to load existing state
            if let Ok(Some(loaded_state)) = api.load_state(&username_clone).await {
                current_state.set(loaded_state);
            } else {
                current_state.set(state);
            }

            is_logged_in.set(true);
        });
    });

    // Recording interval setup
    let interval_handle = RwSignal::new(None::<IntervalHandle>);

    // Start/stop recording function
    let api_service_recording = Arc::clone(&api_service); 
    let toggle_recording = Callback::new(move |start: bool| {
        let mut state = current_state.get();
        state.is_recording = start;
        current_state.set(state);

        if start {
            let api = Arc::clone(&api_service_recording); // Clone the one owned by this closure
            let handle = set_interval_with_handle(
                move || {
                    let state = current_state.get();
                    let state_clone = state.clone();
                    let api_clone = Arc::clone(&api);
                    spawn_local(async move {
                        if let Ok(()) = api_clone.save_state(&state_clone).await {
                            let timestamp =
                                js_sys::Date::new_0().to_iso_string().as_string().unwrap();
                            let data_summary = format!(
                                "Text: {}, Categories: {}, {}, {}, {}",
                                state_clone.text_entry,
                                state_clone.category1,
                                state_clone.category2,
                                state_clone.category3,
                                state_clone.category4
                            );

                            let mut updated_state = current_state.get();
                            updated_state.last_saved = Some(timestamp);
                            updated_state.last_data = Some(data_summary);
                            current_state.set(updated_state);
                        }
                    });
                },
                std::time::Duration::from_secs(5),
            )
            .unwrap();

            interval_handle.set(Some(handle));
        } else if let Some(handle) = interval_handle.get() {
            handle.clear();
            interval_handle.set(None);
        }
    });

    // Function to update a field in the state
    let update_field = Callback::new(move |(field, value): (&'static str, String)| {
        let mut state = current_state.get();
        match field {
            "text_entry" => state.text_entry = value,
            "category1" => state.category1 = value,
            "category2" => state.category2 = value,
            "category3" => state.category3 = value,
            "category4" => state.category4 = value,
            _ => {}
        }
        current_state.set(state);
    });

    view! {
        <div>
            <Show
                when=move || is_logged_in.get()
                fallback=move || view! {
                    <LoginScreen on_login=handle_login />
                }
            >
                <DataEntryScreen
                    state=current_state
                    on_toggle_recording=toggle_recording
                    on_update_field=update_field
                />
            </Show>
        </div>
    }
}
