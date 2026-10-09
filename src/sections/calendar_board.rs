use crate::credits::Credits;
use crate::date::{format_date, month_cells, today_str};
use crate::i18n::{checks_label, t, use_lang, K};
use crate::icons::Icon;
use crate::models::TxnKind;
use chrono::{Datelike, Local};
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Default, PartialEq)]
struct DayStat {
    earned: i64,
    spent: i64,
    checks: i64,
}

#[component]
pub fn CalendarBoard() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let lang = use_lang();
    let now = Local::now().date_naive();
    let view_year = RwSignal::new(now.year());
    let view_month = RwSignal::new(now.month());
    let selected = RwSignal::new(today_str());

    let stats = Memo::new(move |_| {
        let mut map = HashMap::<String, DayStat>::new();
        for t in credits.txns.get().iter() {
            let s = map.entry(t.date.clone()).or_default();
            match t.kind {
                TxnKind::Earn => {
                    s.earned += t.amount;
                    if t.check_key.is_some() {
                        s.checks += 1;
                    }
                }
                TxnKind::Spend => s.spent += t.amount,
            }
        }
        map
    });

    let cells = Memo::new(move |_| month_cells(view_year.get(), view_month.get()));

    let selected_txns = Memo::new(move |_| {
        let sel = selected.get();
        let mut v: Vec<_> = credits
            .txns
            .get()
            .into_iter()
            .filter(|t| t.date == sel)
            .collect();
        v.sort_by(|a, b| b.id.cmp(&a.id));
        v
    });

    view! {
        <section class="space-y-6">
            <div class="m3-card p-5">
                <div class="mb-4 flex items-center justify-between">
                    <h2 class="font-display text-2xl font-bold">{move || t(lang.get(), K::CalendarTitle)}</h2>
                    <div class="flex items-center gap-1">
                        <button
                            class="m3-icon-btn h-9 w-9"
                            aria-label=move || t(lang.get(), K::PrevMonth)
                            on:click=move |_| {
                                if view_month.get() == 1 {
                                    view_year.update(|y| *y -= 1);
                                    view_month.set(12);
                                } else {
                                    view_month.update(|m| *m -= 1);
                                }
                            }
                        >
                            <Icon name="chevron-left" class="h-5 w-5" />
                        </button>
                        <span class="min-w-28 text-center font-display text-lg font-extrabold whitespace-nowrap">
                            {move || crate::date::month_title(lang.get(), view_year.get(), view_month.get())}
                        </span>
                        <button
                            class="m3-icon-btn h-9 w-9"
                            aria-label=move || t(lang.get(), K::NextMonth)
                            on:click=move |_| {
                                if view_month.get() == 12 {
                                    view_year.update(|y| *y += 1);
                                    view_month.set(1);
                                } else {
                                    view_month.update(|m| *m += 1);
                                }
                            }
                        >
                            <Icon name="chevron-right" class="h-5 w-5" />
                        </button>
                    </div>
                </div>

                <div class="mb-1 grid grid-cols-7 text-center text-xs font-bold text-on-surface-variant">
                    {move || crate::date::weekday_labels(lang.get())
                        .iter()
                        .map(|w| view! { <div class="py-1">{*w}</div> })
                        .collect_view()}
                </div>

                <div class="grid grid-cols-7 gap-1">
                    {move || {
                        let today = today_str();
                        let max_earn = stats
                            .get()
                            .values()
                            .map(|s| s.earned)
                            .max()
                            .unwrap_or(0)
                            .max(1);
                        cells
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, day)| {
                                let Some(day) = day else {
                                    return view! { <div></div> }.into_any();
                                };
                                let ds = format_date(view_year.get(), view_month.get(), day);
                                let ds_click = ds.clone();
                                let s = stats.get().get(&ds).copied();
                                let is_today = ds == today;
                                let is_selected = ds == selected.get();
                                let is_future = ds > today;
                                let intensity = s.map(|s| (s.earned as f64 / max_earn as f64).min(1.0)).unwrap_or(0.0);
                                let has_earned = s.map(|s| s.earned > 0).unwrap_or(false);
                                let cls = format!(
                                    "relative flex min-h-[64px] flex-col items-center justify-start rounded-2xl px-1 py-1.5 transition-all sm:min-h-[76px] {}",
                                    if is_selected {
                                        "bg-primary text-on-primary shadow-elevation-1"
                                    } else if has_earned {
                                        ""
                                    } else {
                                        "bg-surface-container-low hover:bg-surface-container-high"
                                    }
                                );
                                let style = if !is_selected && has_earned {
                                    format!("background-color: rgb(var(--earn) / {:.2})", 0.10 + intensity * 0.25)
                                } else {
                                    String::new()
                                };
                                let num_cls = format!(
                                    "font-display text-sm font-bold leading-none {}",
                                    if is_selected {
                                        "text-on-primary"
                                    } else if is_today {
                                        "text-primary"
                                    } else if is_future {
                                        "text-on-surface-variant/50"
                                    } else {
                                        ""
                                    }
                                );
                                let task_count = credits.tasks.get().len() as i64;
                                let _ = i;
                                view! {
                                    <button class=cls style=style on:click=move |_| selected.set(ds_click.clone())>
                                        <span class=num_cls>{day}</span>
                                        {(is_today && !is_selected).then(|| view! {
                                            <span class="mt-0.5 h-1 w-1 rounded-full bg-primary"></span>
                                        })}
                                        {s.map(|s| view! {
                                            <div class="mt-1 flex flex-col items-center gap-0.5">
                                                {(s.earned > 0).then(|| view! {
                                                    <span class=format!(
                                                        "font-display text-[11px] font-extrabold leading-none {}",
                                                        if is_selected { "text-on-primary" } else { "text-earn" }
                                                    )>
                                                        "+" {s.earned}
                                                    </span>
                                                })}
                                                {(s.spent > 0).then(|| view! {
                                                    <span class=format!(
                                                        "font-display text-[11px] font-extrabold leading-none {}",
                                                        if is_selected { "text-on-primary/80" } else { "text-primary" }
                                                    )>
                                                        "−" {s.spent}
                                                    </span>
                                                })}
                                                {(task_count > 0 && s.checks > 0).then(|| view! {
                                                    <span class=format!(
                                                        "mt-0.5 rounded-full px-1.5 text-[10px] font-bold leading-4 {}",
                                                        if s.checks >= task_count {
                                                            if is_selected { "bg-on-primary text-primary" } else { "bg-earn text-on-earn" }
                                                        } else if is_selected {
                                                            "bg-on-primary/20 text-on-primary"
                                                        } else {
                                                            "bg-surface-container-highest text-on-surface-variant"
                                                        }
                                                    )>
                                                        {if s.checks >= task_count { t(lang.get(), K::AllDone).to_string() } else { checks_label(lang.get(), s.checks) }}
                                                    </span>
                                                })}
                                            </div>
                                        })}
                                    </button>
                                }
                                .into_any()
                            })
                            .collect_view()
                    }}
                </div>

                <div class="mt-4 flex flex-wrap items-center gap-4 text-xs text-on-surface-variant">
                    <span class="flex items-center gap-1.5">
                        <span class="h-3 w-3 rounded bg-earn/25"></span>{move || t(lang.get(), K::LegendIntensity)}
                    </span>
                    <span class="font-display font-extrabold text-earn">"+" {move || t(lang.get(), K::EarnedWord)}</span>
                    <span class="font-display font-extrabold text-primary">"−" {move || t(lang.get(), K::SpentWord)}</span>
                    <span class="rounded-full bg-earn px-1.5 font-bold text-on-earn">{move || t(lang.get(), K::AllDone)}</span>
                </div>
            </div>

            <div class="m3-card p-5">
                <h3 class="font-display text-xl font-bold">
                    {move || {
                        let sel = selected.get();
                        if sel == today_str() { t(lang.get(), K::Today).to_string() } else { sel }
                    }}
                    {move || t(lang.get(), K::DetailsSuffix)}
                    {move || {
                        let sel = selected.get();
                        stats.get().get(&sel).copied().map(|s| view! {
                            <span class="ml-3 text-sm font-bold text-on-surface-variant">
                                {move || t(lang.get(), K::EarnedWord)} " " <span class="text-earn">"+" {s.earned}</span>
                                " · " {move || t(lang.get(), K::SpentWord)} " " <span class="text-primary">"−" {s.spent}</span>
                            </span>
                        })
                    }}
                </h3>
                {move || {
                    let list = selected_txns.get();
                    if list.is_empty() {
                        view! {
                            <p class="mt-3 text-sm text-on-surface-variant">{move || t(lang.get(), K::NoRecords)}</p>
                        }.into_any()
                    } else {
                        view! {
                            <div class="mt-3 divide-y divide-outline-variant">
                                {list.into_iter().map(|t| {
                                    let is_earn = t.kind == TxnKind::Earn;
                                    view! {
                                        <div class="flex items-center gap-3 py-2.5">
                                            <span class=format!(
                                                "flex h-8 w-8 shrink-0 items-center justify-center rounded-full font-display text-sm font-bold {}",
                                                if is_earn { "bg-earn-container text-on-earn-container" } else { "bg-primary-container text-on-primary-container" }
                                            )>
                                                {if is_earn { "+" } else { "−" }}
                                            </span>
                                            <span class="flex-1 font-display font-bold">{t.name}</span>
                                            <span class=format!(
                                                "font-display font-extrabold {}",
                                                if is_earn { "text-earn" } else { "text-primary" }
                                            )>
                                                {if is_earn { "+" } else { "−" }} {t.amount}
                                            </span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </section>
    }
}
