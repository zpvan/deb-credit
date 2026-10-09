use crate::charts_svg::{compare_chart_svg, week_chart_svg};
use crate::credits::Credits;
use crate::i18n::{t, use_lang, K};
use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn Charts() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let lang = use_lang();
    let compare = Memo::new(move |_| compare_chart_svg(credits.balance.get(), &credits.wishes.get(), lang.get()));
    let week = Memo::new(move |_| week_chart_svg(&credits.txns.get(), Local::now().date_naive(), lang.get()));

    view! {
        <section class="space-y-6">
            <div class="m3-card p-5">
                <h2 class="font-display text-2xl font-bold">{move || t(lang.get(), K::ChartsCompareTitle)}</h2>
                <p class="mb-4 text-sm text-on-surface-variant">
                    {move || t(lang.get(), K::ChartsCompareSubtitle)}
                </p>
                <div inner_html=move || compare.get()></div>
            </div>
            <div class="m3-card p-5">
                <h2 class="font-display text-2xl font-bold">{move || t(lang.get(), K::ChartsWeekTitle)}</h2>
                <p class="mb-4 text-sm text-on-surface-variant">{move || t(lang.get(), K::ChartsWeekSubtitle)}</p>
                <div inner_html=move || week.get()></div>
            </div>
        </section>
    }
}
