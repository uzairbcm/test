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
    let api_service = ApiService::new();

    // Login logic - now uses the username signal directly
    let handle_login = Callback::new(move |username: String| {
        let username_clone = username.clone();
        let api = api_service.clone();

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

    // Start/stop recording function - maintains a reference to the state signal
    let start_stop_recording = Action::new(move |&start: &bool| {
        let api = api_service.clone();
        let state_signal = current_state;

        async move {
            let mut state = state_signal.get();
            state.is_recording = start;
            state_signal.set(state);

            if start {
                let handle = set_interval_with_handle(
                    move || {
                        let state = state_signal.get();
                        let state_clone = state.clone();

                        spawn_local(async move {
                            if let Ok(()) = api.save_state(&state_clone).await {
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

                                let mut updated_state = state_signal.get();
                                updated_state.last_saved = Some(timestamp);
                                updated_state.last_data = Some(data_summary);
                                state_signal.set(updated_state);
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

            // Return unit to satisfy the Action trait
            ()
        }
    });

    // Function to toggle recording
    let toggle_recording = Callback::new(move |start: bool| {
        start_stop_recording.dispatch(start);
    });
    
    // Function to update a field in the state
    let update_field = Callback::new(move |(field, value): (String, String)| {
        let mut state = current_state.get();
        match field.as_str() {
            "text_entry" => state.text_entry = value,
            "category1" => state.category1 = value,
            "category2" => state.category2 = value,
            "category3" => state.category3 = value,
            "category4" => state.category4 = value,
            _ => (),
        }
        current_state.set(state);
    });
    
    view! {
        <div>
            {move || match is_logged_in.get() {
                false => view! {
                    <LoginScreen on_login=handle_login />
                },
                true => view! {
                    <DataEntryScreen 
                        state=current_state
                        on_toggle_recording=toggle_recording
                        on_update_field=update_field
                    />
                }
            }}
        </div>
    }
}
