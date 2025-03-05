use leptos::*;
use crate::models::user_state::UserState;
use crate::services::api_service::ApiService;
use super::login_screen::LoginScreen;
use super::data_entry_screen::DataEntryScreen;

#[component]
pub fn App() -> impl IntoView {
    let is_logged_in = create_rw_signal(false);
    let current_state = create_rw_signal(UserState::default());
    let api_service = ApiService::new();
    
    // Login handler
    let handle_login = move |username: String| {
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
    };
    
    // Recording interval setup
    let interval_handle = create_rw_signal(None::<IntervalHandle>);
    
    // Toggle recording
    let toggle_recording = move |start: bool| {
        let mut state = current_state.get();
        state.is_recording = start;
        current_state.set(state);
        
        if start {
            let api = api_service.clone();
            let handle = set_interval_with_handle(
                move || {
                    let state = current_state.get();
                    let state_clone = state.clone();
                    
                    spawn_local(async move {
                        if let Ok(()) = api.save_state(&state_clone).await {
                            let timestamp = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
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
            ).unwrap();
            
            interval_handle.set(Some(handle));
        } else if let Some(handle) = interval_handle.get() {
            handle.clear();
            interval_handle.set(None);
        }
    };
    
    // Update state fields
    let update_field = move |field: &'static str, value: String| {
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
    };
    
    view! {
        {move || if !is_logged_in.get() {
            view! {
                <LoginScreen on_login=handle_login />
            }.into_view()
        } else {
            view! {
                <DataEntryScreen 
                    state=current_state
                    on_toggle_recording=toggle_recording
                    on_update_field=update_field
                />
            }.into_view()
        }}
    }
}