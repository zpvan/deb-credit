use crate::components::dialog::Dialog;
use crate::components::kid_dialog::{INPUT_CLASS, LABEL_CLASS, SAVE_BTN_CLASS};
use leptos::prelude::*;

pub const TASK_ICONS: &[&str] = &[
    "⏰", "🤸", "🎵", "🌙", "📚", "🦷", "🧹", "🏃", "🎨", "✍️", "🧺", "🥦", "🛁", "🎹", "🧩", "🚲",
];
pub const WISH_ICONS: &[&str] = &[
    "🍦", "🍿", "✈️", "📺", "🎁", "🧸", "🎮", "🍕", "🎢", "⚽", "🚗", "🏖️", "🎬", "🍰", "🪁", "🛝",
];

/// 任务/心愿共用的编辑弹窗。initial 为 (name, value, icon)；on_save 接收 (name, value, icon)。
/// value 解析对应原版 Math.max(1, Math.round(Number(value) || 1))：
/// 空串/非法输入 → 1；小数四舍五入；最小为 1。
#[component]
pub fn ItemDialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<&'static str>,
    #[prop(into)] value_label: &'static str,
    icons: &'static [&'static str],
    #[prop(into)] initial: Signal<Option<(String, i64, String)>>,
    #[prop(into)] on_save: Callback<(String, i64, String)>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let value = RwSignal::new("1".to_string());
    let icon = RwSignal::new(icons[0].to_string());

    Effect::new(move |_| {
        if open.get() {
            match initial.get() {
                Some((n, v, i)) => {
                    name.set(n);
                    value.set(v.to_string());
                    icon.set(i);
                }
                None => {
                    name.set(String::new());
                    value.set("1".to_string());
                    icon.set(icons[0].to_string());
                }
            }
        }
    });

    view! {
        <Dialog open=open content_class="rounded-3xl sm:max-w-md">
            <h2 class="text-lg leading-none font-semibold font-display text-xl">{move || title.get()}</h2>
            <div class="space-y-4 py-2">
                <div class="space-y-2">
                    <label class=LABEL_CLASS>"名称"</label>
                    <input
                        class=INPUT_CLASS
                        placeholder="例如：自己收拾玩具"
                        maxlength="20"
                        bind:value=name
                    />
                </div>
                <div class="space-y-2">
                    <label class=LABEL_CLASS>{value_label}</label>
                    <input class=INPUT_CLASS type="number" min="1" bind:value=value />
                </div>
                <div class="space-y-2">
                    <label class=LABEL_CLASS>"选一个图标"</label>
                    <div class="grid grid-cols-8 gap-1.5">
                        {icons
                            .iter()
                            .map(|ic| {
                                let cur = ic.to_string();
                                let ic2 = *ic;
                                view! {
                                    <button
                                        type="button"
                                        class=move || format!(
                                            "flex h-9 items-center justify-center rounded-xl text-xl transition-all {}",
                                            if icon.get() == cur { "bg-[#ecc22e] scale-110 shadow-sm" } else { "bg-muted hover:bg-[#feffc9]" }
                                        )
                                        on:click=move |_| icon.set(ic2.to_string())
                                    >
                                        {*ic}
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
                        let v = value.get().trim().parse::<f64>().unwrap_or(1.0);
                        let v = v.round().max(1.0) as i64;
                        let n = name.get().trim().to_string();
                        if n.is_empty() {
                            return;
                        }
                        on_save.run((n, v, icon.get()));
                        open.set(false);
                    }
                >
                    保存
                </button>
            </div>
        </Dialog>
    }
}
