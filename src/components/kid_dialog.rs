use crate::components::dialog::Dialog;
use leptos::prelude::*;

pub const KID_AVATARS: &[&str] = &[
    "🧒", "👧", "👦", "👶", "🧑", "👱‍♀️", "👱", "🐯", "🐰", "🐼", "🦊", "🐸",
];

pub(crate) const INPUT_CLASS: &str = "placeholder:text-muted-foreground border-input h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 md:text-sm rounded-xl";
pub(crate) const LABEL_CLASS: &str = "text-sm font-medium leading-none";
pub(crate) const SAVE_BTN_CLASS: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 h-9 px-4 py-2 w-full rounded-full bg-[#f8622f] text-white hover:bg-[#e04f20] font-display text-base";

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
        <Dialog open=open content_class="rounded-3xl sm:max-w-md">
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
                                            if avatar.get() == av { "bg-[#ecc22e] scale-110 shadow-sm" } else { "bg-muted hover:bg-[#feffc9]" }
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
