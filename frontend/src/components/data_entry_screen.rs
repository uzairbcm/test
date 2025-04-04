use leptos::prelude::*;
use crate::models::user_state::UserState;
use super::dropdown_select::DropdownSelect;

#[component]
pub fn DataEntryScreen(
    state: RwSignal<UserState>,
    on_toggle_recording: Callback<bool>,
    on_update_field: Callback<(&'static str, String)>,
) -> impl IntoView {
    // Category options (replace these with your actual categories)
    let category1_options = vec!["Option 1A", "Option 1B", "Option 1C"];
    let category2_options = vec!["Option 2A", "Option 2B", "Option 2C"];
    let category3_options = vec!["Option 3A", "Option 3B", "Option 3C"];
    let category4_options = vec!["Option 4A", "Option 4B", "Option 4C"];
    
    // Create individual callbacks for each dropdown
    let on_category1_change = Callback::new(move |v: String| {
        on_update_field.run(("category1", v));
    });
    
    let on_category2_change = Callback::new(move |v: String| {
        on_update_field.run(("category2", v));
    });
    
    let on_category3_change = Callback::new(move |v: String| {
        on_update_field.run(("category3", v));
    });
    
    let on_category4_change = Callback::new(move |v: String| {
        on_update_field.run(("category4", v));
    });
    
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
                        on:input=move |ev| on_update_field.run(("text_entry", event_target_value(&ev)))
                    ></textarea>
                </div>
                
                <div class="dropdown-container">
                    <DropdownSelect 
                        id="category1"
                        label="Category 1"
                        options=category1_options
                        value=Memo::new(move |_| state.get().category1.clone())
                        on_change=on_category1_change
                    />
                    
                    <DropdownSelect 
                        id="category2"
                        label="Category 2"
                        options=category2_options
                        value=Memo::new(move |_| state.get().category2.clone())
                        on_change=on_category2_change
                    />
                    
                    <DropdownSelect 
                        id="category3"
                        label="Category 3"
                        options=category3_options
                        value=Memo::new(move |_| state.get().category3.clone())
                        on_change=on_category3_change
                    />
                    
                    <DropdownSelect 
                        id="category4"
                        label="Category 4"
                        options=category4_options
                        value=Memo::new(move |_| state.get().category4.clone())
                        on_change=on_category4_change
                    />
                </div>
                
                <div class="button-container">
                    {move || {
                        let is_recording = state.get().is_recording;
                        view! {
                            <button 
                                class="start-button"
                                class:active=is_recording
                                on:click=move |_| on_toggle_recording.run(true)
                                disabled=is_recording
                            >
                                "Start Recording"
                            </button>
                            <button
                                class="stop-button"
                                class:active=!is_recording
                                on:click=move |_| on_toggle_recording.run(false)
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
                <Show
                    when=move || {
                        let current = state.get();
                        current.last_saved.is_some() && current.last_data.is_some()
                    }
                    fallback=move || {
                        view! {
                            <p class="no-data">"No data saved yet"</p>
                        }
                    }
                >
                    {move || {
                        let current = state.get();
                        // We can safely unwrap here because the Show component only renders this
                        // when both last_saved and last_data are Some
                        let timestamp = current.last_saved.clone().unwrap();
                        let data = current.last_data.clone().unwrap();
                        
                        view! {
                            <div class="status-info">
                                <p class="timestamp">"Last saved: " {timestamp}</p>
                                <p class="data-summary">"Data: " {data}</p>
                            </div>
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}