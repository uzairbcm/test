use leptos::*;
use leptos::prelude::*;

#[component]
pub fn DropdownSelect(
    id: &'static str,
    label: &'static str,
    options: Vec<&'static str>,
    value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="dropdown-group">
            <label for={id}>{label}</label>
            <select
                id={id}
                prop:value={move || value.get()}
                on:change=move |ev| on_change.run(event_target_value(&ev))
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