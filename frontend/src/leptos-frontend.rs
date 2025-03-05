// main.rs
use leptos::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;

// Domain Models
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct UserState {
    username: String,
    text_entry: String,
    category1: String,
    category2: String,
    category3: String,
    category4: String,
    is_recording: bool,
    last_saved: Option<String>,
    last_data: Option<String>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            username: String::new(),
            text_entry: String::new(),
            category1: String::new(),
            category2: String::new(),
            category3: String::new(),
            category4: String::new(),
            is_recording: false,
            last_saved: None,
            last_data: None,
        }
    }
}

// API Service
#[derive(Clone)]
struct ApiService {
    client: Client,
    base_url: String,
}

impl ApiService {
    fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:3000/api".to_string(),
        }
    }

    async fn save_state(&self, state: &UserState) -> Result<(), reqwest::Error> {
        self.client
            .post(&format!("{}/state", self.base_url))
            .json(state)
            .send()
            .await?;
        Ok(())
    }

    async fn load_state(&self, username: &str) -> Result<Option<UserState>, reqwest::Error> {
        let response = self.client
            .get(&format!("{}/state/{}", self.base_url, username))
            .send()
            .await?;
        
        if response.status().is_success() {
            let state = response.json::<UserState>().await?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }
}

// Components
#[component]
fn App() -> impl IntoView {
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

#[component]
fn LoginScreen(on_login: Callback<String>) -> impl IntoView {
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

#[component]
fn DataEntryScreen(
    state: RwSignal<UserState>,
    on_toggle_recording: Callback<bool>,
    on_update_field: Callback<(&'static str, String)>,
) -> impl IntoView {
    // Category options (you'll replace these with your actual categories)
    let category1_options = vec!["Option 1A", "Option 1B", "Option 1C"];
    let category2_options = vec!["Option 2A", "Option 2B", "Option 2C"];
    let category3_options = vec!["Option 3A", "Option 3B", "Option 3C"];
    let category4_options = vec!["Option 4A", "Option 4B", "Option 4C"];
    
    view! {
        <div class="data-entry-container">
            <h1>"Data Logger"</h1>
            <p class="welcome-message">"Welcome, " {move || state.get().username}</p>
            
            <div class="input-container">
                <div class="input-group">
                    <label for="text-entry">"Text Entry:"</label>
                    <textarea 
                        id="text-entry"
                        prop:value=move || state.get().text_entry
                        on:input=move |ev| on_update_field("text_entry", event_target_value(&ev))
                    ></textarea>
                </div>
                
                <div class="dropdown-container">
                    <DropdownSelect 
                        id="category1"
                        label="Category 1"
                        options=category1_options
                        value=move || state.get().category1
                        on_change=move |v| on_update_field("category1", v)
                    />
                    
                    <DropdownSelect 
                        id="category2"
                        label="Category 2"
                        options=category2_options
                        value=move || state.get().category2
                        on_change=move |v| on_update_field("category2", v)
                    />
                    
                    <DropdownSelect 
                        id="category3"
                        label="Category 3"
                        options=category3_options
                        value=move || state.get().category3
                        on_change=move |v| on_update_field("category3", v)
                    />
                    
                    <DropdownSelect 
                        id="category4"
                        label="Category 4"
                        options=category4_options
                        value=move || state.get().category4
                        on_change=move |v| on_update_field("category4", v)
                    />
                </div>
                
                <div class="button-container">
                    {move || {
                        let is_recording = state.get().is_recording;
                        view! {
                            <button 
                                class="start-button"
                                class:active=is_recording
                                on:click=move |_| on_toggle_recording(true)
                                disabled=is_recording
                            >
                                "Start Recording"
                            </button>
                            <button
                                class="stop-button"
                                class:active=!is_recording
                                on:click=move |_| on_toggle_recording(false)
                                disabled=!is_recording
                            >
                                "Stop Recording"
                            </button>
                        }
                    }}
                </div>
            </div>
            
            <div class="status-container">
                <h3>"Recording Status"</h3>
                {move || {
                    let current = state.get();
                    if let (Some(timestamp), Some(data)) = (current.last_saved.clone(), current.last_data.clone()) {
                        view! {
                            <div class="status-info">
                                <p class="timestamp">"Last saved: " {timestamp}</p>
                                <p class="data-summary">"Data: " {data}</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <p class="no-data">"No data saved yet"</p>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn DropdownSelect(
    id: &'static str,
    label: &'static str,
    options: Vec<&'static str>,
    value: Fn() -> String + 'static,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="dropdown-group">
            <label for={id}>{label}</label>
            <select
                id={id}
                prop:value=value
                on:change=move |ev| on_change(event_target_value(&ev))
            >
                <option value="">"-- Select a " {label} " --"</option>
                {options.iter().map(|option| {
                    view! {
                        <option value={*option}>{*option}</option>
                    }
                }).collect_view()}
            </select>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
