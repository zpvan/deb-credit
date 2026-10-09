use crate::icons::Icon;
use leptos::prelude::*;

const CONTENT_BASE: &str = "bg-surface-container-high text-on-surface fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-[28px] p-6 shadow-elevation-3 duration-200 outline-none sm:max-w-lg";

/// 受控对话框：open 为 true 时渲染遮罩 + 居中面板。
/// 点击遮罩、按 Esc、点右上角 X（show_close=true 时）均关闭。
#[component]
pub fn Dialog(
    open: RwSignal<bool>,
    #[prop(into, optional)] content_class: String,
    #[prop(default = true)] show_close: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let panel_ref = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        if open.get() {
            if let Some(el) = panel_ref.get() {
                let _ = el.focus();
            }
        }
    });
    let cls = format!("{CONTENT_BASE} {content_class}");
    view! {
        {move || {
            if open.get() {
                Some(view! {
                    <div class="fixed inset-0 z-50 bg-black/40" on:click=move |_| open.set(false)>
                        <div
                            node_ref=panel_ref
                            tabindex="-1"
                            role="dialog"
                            aria-modal="true"
                            class=cls.clone()
                            on:click=move |ev: leptos::ev::MouseEvent| ev.stop_propagation()
                            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                if ev.key() == "Escape" {
                                    open.set(false);
                                }
                            }
                        >
                            {children()}
                            {show_close.then(|| view! {
                                <button
                                    class="m3-icon-btn absolute top-3 right-3 h-8 w-8"
                                    on:click=move |_| open.set(false)
                                    aria-label="Close"
                                >
                                    <Icon name="x" class="h-4 w-4" />
                                </button>
                            })}
                        </div>
                    </div>
                })
            } else {
                None
            }
        }}
    }
}
