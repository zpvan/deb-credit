use crate::credits::Credits;
use crate::date::{format_date, month_cells, today_str, WEEKDAYS_CN};
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
            <div class="rounded-3xl border-2 border-border bg-card p-5">
                <div class="mb-4 flex items-center justify-between">
                    <h2 class="font-display text-2xl font-bold">"日历看板"</h2>
                    <div class="flex items-center gap-1">
                        <button
                            class="flex h-9 w-9 items-center justify-center rounded-full hover:bg-[#feffc9]"
                            aria-label="上个月"
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
                        <span class="w-28 text-center font-display text-lg font-extrabold">
                            {move || format!("{}年{}月", view_year.get(), view_month.get())}
                        </span>
                        <button
                            class="flex h-9 w-9 items-center justify-center rounded-full hover:bg-[#feffc9]"
                            aria-label="下个月"
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

                <div class="mb-1 grid grid-cols-7 text-center text-xs font-bold text-muted-foreground">
                    {WEEKDAYS_CN
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
                                    "relative flex min-h-[64px] flex-col items-center justify-start rounded-2xl border-2 px-1 py-1.5 transition-all sm:min-h-[76px] {}",
                                    if is_selected {
                                        "border-[#1d5d3f] bg-[#1d5d3f] text-white shadow"
                                    } else if has_earned {
                                        "border-transparent"
                                    } else {
                                        "border-transparent bg-muted/40 hover:bg-muted"
                                    }
                                );
                                let style = if !is_selected && has_earned {
                                    format!("background-color: rgba(29, 93, 63, {})", 0.08 + intensity * 0.25)
                                } else {
                                    String::new()
                                };
                                let num_cls = format!(
                                    "font-display text-sm font-bold leading-none {}",
                                    if is_selected {
                                        "text-white"
                                    } else if is_today {
                                        "text-[#f8622f]"
                                    } else if is_future {
                                        "text-muted-foreground/50"
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
                                            <span class="mt-0.5 h-1 w-1 rounded-full bg-[#f8622f]"></span>
                                        })}
                                        {s.map(|s| view! {
                                            <div class="mt-1 flex flex-col items-center gap-0.5">
                                                {(s.earned > 0).then(|| view! {
                                                    <span class=format!(
                                                        "font-display text-[11px] font-extrabold leading-none {}",
                                                        if is_selected { "text-[#ecc22e]" } else { "text-[#1d5d3f]" }
                                                    )>
                                                        "+" {s.earned}
                                                    </span>
                                                })}
                                                {(s.spent > 0).then(|| view! {
                                                    <span class=format!(
                                                        "font-display text-[11px] font-extrabold leading-none {}",
                                                        if is_selected { "text-[#f6bbfd]" } else { "text-[#f8622f]" }
                                                    )>
                                                        "−" {s.spent}
                                                    </span>
                                                })}
                                                {(task_count > 0 && s.checks > 0).then(|| view! {
                                                    <span class=format!(
                                                        "mt-0.5 rounded-full px-1.5 text-[10px] font-bold leading-4 {}",
                                                        if s.checks >= task_count {
                                                            if is_selected { "bg-[#ecc22e] text-[#1d5d3f]" } else { "bg-[#ecc22e]/70 text-[#1d5d3f]" }
                                                        } else if is_selected {
                                                            "bg-white/20 text-white/90"
                                                        } else {
                                                            "bg-muted-foreground/15 text-muted-foreground"
                                                        }
                                                    )>
                                                        {if s.checks >= task_count { "全勤".to_string() } else { format!("{}项", s.checks) }}
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

                <div class="mt-4 flex flex-wrap items-center gap-4 text-xs text-muted-foreground">
                    <span class="flex items-center gap-1.5">
                        <span class="h-3 w-3 rounded bg-[#1d5d3f]/25"></span>"绿色深浅 = 赚得多少"
                    </span>
                    <span class="font-display font-extrabold text-[#1d5d3f]">"+赚得"</span>
                    <span class="font-display font-extrabold text-[#f8622f]">"−花掉"</span>
                    <span class="rounded-full bg-[#ecc22e]/70 px-1.5 font-bold text-[#1d5d3f]">"全勤"</span>
                </div>
            </div>

            <div class="rounded-3xl border-2 border-border bg-card p-5">
                <h3 class="font-display text-xl font-bold">
                    {move || {
                        let sel = selected.get();
                        if sel == today_str() { "今天".to_string() } else { sel }
                    }}
                    " 明细"
                    {move || {
                        let sel = selected.get();
                        stats.get().get(&sel).copied().map(|s| view! {
                            <span class="ml-3 text-sm font-bold text-muted-foreground">
                                "赚 " <span class="text-[#1d5d3f]">"+" {s.earned}</span>
                                " · 花 " <span class="text-[#f8622f]">"−" {s.spent}</span>
                            </span>
                        })
                    }}
                </h3>
                {move || {
                    let list = selected_txns.get();
                    if list.is_empty() {
                        view! {
                            <p class="mt-3 text-sm text-muted-foreground">"这一天没有记录"</p>
                        }.into_any()
                    } else {
                        view! {
                            <div class="mt-3 divide-y divide-border">
                                {list.into_iter().map(|t| {
                                    let is_earn = t.kind == TxnKind::Earn;
                                    view! {
                                        <div class="flex items-center gap-3 py-2.5">
                                            <span class=format!(
                                                "flex h-8 w-8 shrink-0 items-center justify-center rounded-full font-display text-sm font-bold {}",
                                                if is_earn { "bg-[#1d5d3f]/10 text-[#1d5d3f]" } else { "bg-[#f8622f]/10 text-[#f8622f]" }
                                            )>
                                                {if is_earn { "+" } else { "−" }}
                                            </span>
                                            <span class="flex-1 font-display font-bold">{t.name}</span>
                                            <span class=format!(
                                                "font-display font-extrabold {}",
                                                if is_earn { "text-[#1d5d3f]" } else { "text-[#f8622f]" }
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
