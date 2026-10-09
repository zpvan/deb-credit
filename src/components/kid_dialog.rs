use crate::components::dialog::Dialog;
use leptos::prelude::*;

pub const KID_AVATARS: &[&str] = &[
    "🧒", "👧", "👦", "👶", "🧑", "👱‍♀️", "👱", "🐯", "🐰", "🐼", "🦊", "🐸",
];

pub(crate) const INPUT_CLASS: &str = "m3-input";
pub(crate) const LABEL_CLASS: &str = "font-display text-sm font-bold text-on-surface-variant";
pub(crate) const SAVE_BTN_CLASS: &str = "m3-btn-filled w-full";

/// 添加/编辑宝贝。initial 为 (name, avatar)；on_save 接收 (name, avatar)。
#[component]
pub fn KidDialog(
    open: RwSignal<bool>,
    #[prop(into)] initial: Signal<Option<(String, String)>>,
    #[prop(into)] on_save: Callback<(String, String)>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let avatar = RwSignal::new(KID_AVATARS[0].to_string());

    Effect::new(move |_| {
        if open.get() {
            match initial.get() {
                Some((n, a)) => {
                    name.set(n);
                    avatar.set(a);
                }
                None => {
                    name.set(String::new());
                    avatar.set(KID_AVATARS[0].to_string());
                }
            }
        }
    });

    let title = move || {
        if initial.get().is_some() { "编辑宝贝" } else { "添加宝贝" }
    };

    view! {
        <Dialog open=open content_class="sm:max-w-md">
            <h2 class="text-lg leading-none font-semibold font-display text-xl">{title}</h2>
            <div class="space-y-4 py-2">
                <div class="space-y-2">
                    <label class=LABEL_CLASS>"名字"</label>
                    <input
                        class=INPUT_CLASS
                        placeholder="例如：小宝"
                        maxlength="12"
                        bind:value=name
                    />
                </div>
                <div class="space-y-2">
                    <label class=LABEL_CLASS>"选一个头像"</label>
                    <div class="grid grid-cols-6 gap-1.5">
                        {KID_AVATARS
                            .iter()
                            .map(|a| {
                                let av = a.to_string();
                                let av2 = *a;
                                view! {
                                    <button
                                        type="button"
                                        class=move || format!(
                                            "flex h-10 items-center justify-center rounded-xl text-2xl transition-all {}",
                                            if avatar.get() == av { "bg-primary-container ring-2 ring-primary scale-110" } else { "bg-surface-container-low hover:bg-secondary-container" }
                                        )
                                        on:click=move |_| avatar.set(av2.to_string())
                                    >
                                        {*a}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>
            <div class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                <button
                    class=SAVE_BTN_CLASS
                    disabled=move || name.get().trim().is_empty()
                    on:click=move |_| {
                        on_save.run((name.get().trim().to_string(), avatar.get()));
                        open.set(false);
                    }
                >
                    保存
                </button>
            </div>
        </Dialog>
    }
}
