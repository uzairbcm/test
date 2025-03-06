use leptos::prelude::*;
use crate::models::user_state::UserState;
use super::dropdown_select::DropdownSelect;

#[component]
pub fn DataEntryScreen(
    state: RwSignal<UserState>,
    on_toggle_recording: Box<dyn Fn(bool)>,
    on_update_field: Box<dyn Fn(&'static str, String)>,
) -> impl IntoView {
    // Category options (replace these with your actual categories)
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
                        value=create_memo(move |_| state.get().category1.clone())
                        on_change=move |v| (on_update_field)("category1", v)
                    />
                    
                    <DropdownSelect 
                        id="category2"
                        label="Category 2"
                        options=category2_options
                        value=create_memo(move |_| state.get().category2.clone())
                        on_change=move |v| on_update_field("category2", v)
                    />
                    
                    <DropdownSelect 
                        id="category3"
                        label="Category 3"
                        options=category3_options
                        value=create_memo(move |_| state.get().category3.clone())
                        on_change=on_update_field("category3", v)
                    />
                    
                    <DropdownSelect 
                        id="category4"
                        label="Category 4"
                        options=category4_options
                        value=create_memo(move |_| state.get().category4.clone())
                        on_change=on_update_field("category4", v).set()
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