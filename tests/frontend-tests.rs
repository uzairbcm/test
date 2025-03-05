#[cfg(test)]
mod frontend_tests {
    use leptos::*;
    use wasm_bindgen_test::*;
    use leptos_frontend::components::{
        login_screen::LoginScreen,
        data_entry_screen::DataEntryScreen,
        dropdown_select::DropdownSelect,
    };
    use leptos_frontend::models::user_state::UserState;

    wasm_bindgen_test_configure!(run_in_browser);

    // Helper function to create test data
    fn create_test_user_state() -> UserState {
        UserState {
            username: "testuser".to_string(),
            text_entry: "test text".to_string(),
            category1: "Option 1A".to_string(),
            category2: "Option 2A".to_string(),
            category3: "Option 3A".to_string(),
            category4: "Option 4A".to_string(),
            is_recording: false,
            last_saved: None,
            last_data: None,
        }
    }

    #[wasm_bindgen_test]
    fn test_login_screen() {
        create_scope(create_runtime(), |cx| {
            // Track when the login callback is called
            let login_called = create_rw_signal(cx, false);
            let username_captured = create_rw_signal(cx, String::new());
            
            // Create on_login callback that updates our tracking signals
            let on_login = move |username: String| {
                login_called.set(true);
                username_captured.set(username);
            };
            
            // Mount the component
            let _ = mount_to_body(cx, || view! { cx, <LoginScreen on_login=on_login/> });
            
            // Simulate entering a username
            let input = document().query_selector("#username").unwrap().unwrap();
            let input_element = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            input_element.set_value("testuser");
            
            // Simulate form submission
            let form = document().query_selector("form").unwrap().unwrap();
            let event = web_sys::Event::new("submit").unwrap();
            event.prevent_default();
            form.dispatch_event(&event).unwrap();
            
            // Verify that the login callback was called with the correct username
            assert!(login_called.get());
            assert_eq!(username_captured.get(), "testuser");
        });
    }
    
    #[wasm_bindgen_test]
    fn test_data_entry_screen() {
        create_scope(create_runtime(), |cx| {
            // Create test state
            let state = create_rw_signal(cx, create_test_user_state());
            
            // Track field updates
            let field_updated = create_rw_signal(cx, false);
            let field_name = create_rw_signal(cx, String::new());
            let field_value = create_rw_signal(cx, String::new());
            
            // Create field update callback
            let on_update_field = move |field: &'static str, value: String| {
                field_updated.set(true);
                field_name.set(field.to_string());
                field_value.set(value.clone());
                
                // Update the state (mimicking parent component behavior)
                let mut current_state = state.get();
                match field {
                    "text_entry" => current_state.text_entry = value,
                    "category1" => current_state.category1 = value,
                    "category2" => current_state.category2 = value,
                    "category3" => current_state.category3 = value,
                    "category4" => current_state.category4 = value,
                    _ => {}
                }
                state.set(current_state);
            };
            
            // Track recording state changes
            let recording_changed = create_rw_signal(cx, false);
            let recording_value = create_rw_signal(cx, false);
            
            // Create recording toggle callback
            let on_toggle_recording = move |start: bool| {
                recording_changed.set(true);
                recording_value.set(start);
                
                // Update the state (mimicking parent component behavior)
                let mut current_state = state.get();
                current_state.is_recording = start;
                state.set(current_state);
            };
            
            // Mount the component
            let _ = mount_to_body(cx, || {
                view! { cx,
                    <DataEntryScreen
                        state=state
                        on_update_field=on_update_field
                        on_toggle_recording=on_toggle_recording
                    />
                }
            });
            
            // Verify initial rendering
            let welcome_message = document().query_selector(".welcome-message").unwrap().unwrap();
            assert!(welcome_message.text_content().unwrap().contains("testuser"));
            
            // Test start recording button
            let start_button = document()
                .query_selector(".start-button")
                .unwrap()
                .unwrap();
            
            let event = web_sys::Event::new("click").unwrap();
            start_button.dispatch_event(&event).unwrap();
            
            assert!(recording_changed.get());
            assert!(recording_value.get());
            assert!(state.get().is_recording);
        });
    }
    
    #[wasm_bindgen_test]
    fn test_dropdown_select() {
        create_scope(create_runtime(), |cx| {
            // Track option selection
            let option_selected = create_rw_signal(cx, false);
            let selected_value = create_rw_signal(cx, String::new());
            
            // Create on_change callback
            let on_change = move |value: String| {
                option_selected.set(true);
                selected_value.set(value);
            };
            
            // Setup test options
            let options = vec!["Option A", "Option B", "Option C"];
            let current_value = create_rw_signal(cx, "Option A".to_string());
            let value_fn = move || current_value.get();
            
            // Mount the component
            let _ = mount_to_body(cx, || {
                view! { cx,
                    <DropdownSelect
                        id="test-dropdown"
                        label="Test Dropdown"
                        options=options
                        value=value_fn
                        on_change=on_change
                    />
                }
            });
            
            // Verify the dropdown has all options
            let select = document().query_selector("#test-dropdown").unwrap().unwrap();
            let select_element = select.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            
            assert_eq!(select_element.options().length(), options.len() as u32 + 1); // +1 for the placeholder
        });
    }
}