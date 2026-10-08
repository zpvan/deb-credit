use crate::charts_svg::{compare_chart_svg, week_chart_svg};
use crate::credits::Credits;
use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn Charts() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let compare = Memo::new(move |_| compare_chart_svg(credits.balance.get(), &credits.wishes.get()));
    let week = Memo::new(move |_| week_chart_svg(&credits.txns.get(), Local::now().date_naive()));

    view! {
        <section class="space-y-6">
            <div class="rounded-3xl border-2 border-border bg-card p-5">
                <h2 class="font-display text-2xl font-bold">"积分 vs 心愿"</h2>
                <p class="mb-4 text-sm text-muted-foreground">
                    "金色是当前攒下的积分，彩色是每个心愿需要的积分，一眼看出还差多少"
                </p>
                <div inner_html=move || compare.get()></div>
            </div>
            <div class="rounded-3xl border-2 border-border bg-card p-5">
                <h2 class="font-display text-2xl font-bold">"最近 7 天收支"</h2>
                <p class="mb-4 text-sm text-muted-foreground">"绿色是每天赚到的积分，橙色是花掉的积分"</p>
                <div inner_html=move || week.get()></div>
            </div>
        </section>
    }
}
