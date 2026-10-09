use crate::credits::Credits;
use crate::models::TxnKind;
use leptos::prelude::*;

#[component]
pub fn History() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let sorted = Memo::new(move |_| {
        let mut v = credits.txns.get();
        v.sort_by(|a, b| b.id.cmp(&a.id));
        v.truncate(50);
        v
    });

    view! {
        {move || {
            let list = sorted.get();
            if list.is_empty() {
                view! {
                    <section class="rounded-3xl border-2 border-dashed border-outline-variant bg-surface-container-low/50 p-10 text-center text-on-surface-variant">
                        "还没有记录，去完成今天的任务赚第一笔积分吧！"
                    </section>
                }.into_any()
            } else {
                view! {
                    <section>
                        <h2 class="mb-4 font-display text-2xl font-bold">"积分记录"</h2>
                        <div class="m3-card divide-y divide-outline-variant overflow-hidden">
                            {list.into_iter().map(|t| {
                                let is_earn = t.kind == TxnKind::Earn;
                                view! {
                                    <div class="flex items-center gap-3 px-5 py-3">
                                        <span class=format!(
                                            "flex h-9 w-9 shrink-0 items-center justify-center rounded-full font-display text-base font-bold {}",
                                            if is_earn { "bg-earn-container text-on-earn-container" } else { "bg-primary-container text-on-primary-container" }
                                        )>
                                            {if is_earn { "+" } else { "−" }}
                                        </span>
                                        <div class="flex-1">
                                            <div class="font-display font-bold">{t.name}</div>
                                            <div class="text-xs text-on-surface-variant">{t.date}</div>
                                        </div>
                                        <span class=format!(
                                            "font-display text-lg font-extrabold {}",
                                            if is_earn { "text-earn" } else { "text-primary" }
                                        )>
                                            {if is_earn { "+" } else { "−" }} {t.amount}
                                        </span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </section>
                }.into_any()
            }
        }}
    }
}
