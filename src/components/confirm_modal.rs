use leptos::*;

#[component]
pub fn ConfirmModal(
    show: RwSignal<bool>,
    title: String,
    message: String,
    confirm_label: String,
    on_confirm: impl Fn() + 'static,
) -> impl IntoView {
    let confirmed = create_rw_signal(false);
    let trigger_confirm = create_rw_signal(false);
    let trigger_cancel = create_rw_signal(false);

    create_effect(move |_| {
        if show.get() {
            confirmed.set(false);
        }
    });

    create_effect(move |_| {
        if trigger_confirm.get() {
            trigger_confirm.set(false);
            if !confirmed.get() {
                confirmed.set(true);
                on_confirm();
                show.set(false);
            }
        }
    });

    create_effect(move |_| {
        if trigger_cancel.get() {
            trigger_cancel.set(false);
            show.set(false);
        }
    });

    view! {
        {move || {
            if !show.get() {
                return view! { <div></div> }.into_view();
            }
            view! {
                <div
                    style="
                        position: fixed; top: 0; left: 0; width: 100%; height: 100%;
                        background: var(--bg-overlay); display: flex; align-items: center;
                        justify-content: center; z-index: 1000;
                    "
                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                        if ev.key() == "Escape" { trigger_cancel.set(true); }
                    }
                    on:click=move |ev: leptos::ev::MouseEvent| {
                        let target = event_target::<web_sys::HtmlElement>(&ev);
                        if target.tag_name() == "DIV" && target.style().get_property_value("position").ok().as_deref() == Some("fixed") {
                            trigger_cancel.set(true);
                        }
                    }
                >
                    <div
                        style="
                            background: var(--bg-main); border-radius: 12px; padding: 28px;
                            width: 400px; max-width: 90%; border: 1px solid var(--border-color);
                            box-shadow: 0 8px 32px rgba(0,0,0,0.4);
                        "
                        tabindex="0"
                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                            if ev.key() == "Escape" { trigger_cancel.set(true); }
                            else if ev.key() == "Enter" { trigger_confirm.set(true); }
                        }
                    >
                        <h2 style="margin: 0 0 12px 0; font-size: 20px; color: var(--text-main);">
                            {title.clone()}
                        </h2>
                        <p style="color: var(--text-muted); font-size: 14px; line-height: 1.5; margin: 0 0 24px 0;">
                            {message.clone()}
                        </p>
                        <div style="display: flex; gap: 10px; justify-content: flex-end;">
                            <button
                                aria-label="Cancel"
                                on:click=move |_| trigger_cancel.set(true)
                                style="
                                    padding: 10px 20px; border: 1px solid var(--button-ghost-border); border-radius: 8px;
                                    background: transparent; color: var(--button-ghost-text); cursor: pointer;
                                    font-size: 14px;
                                "
                            >
                                Cancel
                            </button>
                            <button
                                aria-label="Confirm"
                                on:click=move |_| trigger_confirm.set(true)
                                style="
                                    padding: 10px 20px; border: none; border-radius: 8px;
                                    background: var(--button-primary-bg); color: var(--button-primary-text); cursor: pointer;
                                    font-size: 14px;
                                "
                            >
                                {confirm_label.clone()}
                            </button>
                        </div>
                    </div>
                </div>
            }.into_view()
        }}
    }
}
