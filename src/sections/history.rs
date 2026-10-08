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
                    <section class="rounded-3xl border-2 border-dashed border-border bg-card/50 p-10 text-center text-muted-foreground">
                        "还没有记录，去完成今天的任务赚第一笔积分吧！"
                    </section>
                }.into_any()
            } else {
                view! {
                    <section>
                        <h2 class="mb-4 font-display text-2xl font-bold">"积分记录"</h2>
                        <div class="divide-y divide-border overflow-hidden rounded-3xl border-2 border-border bg-card">
                            {list.into_iter().map(|t| {
                                let is_earn = t.kind == TxnKind::Earn;
                                view! {
                                    <div class="flex items-center gap-3 px-5 py-3">
                                        <span class=format!(
                                            "flex h-9 w-9 shrink-0 items-center justify-center rounded-full font-display text-base font-bold {}",
                                            if is_earn { "bg-[#1d5d3f]/10 text-[#1d5d3f]" } else { "bg-[#f8622f]/10 text-[#f8622f]" }
                                        )>
                                            {if is_earn { "+" } else { "−" }}
                                        </span>
                                        <div class="flex-1">
                                            <div class="font-display font-bold">{t.name}</div>
                                            <div class="text-xs text-muted-foreground">{t.date}</div>
                                        </div>
                                        <span class=format!(
                                            "font-display text-lg font-extrabold {}",
                                            if is_earn { "text-[#1d5d3f]" } else { "text-[#f8622f]" }
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
