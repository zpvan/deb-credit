use crate::components::dialog::Dialog;
use crate::components::item_dialog::{ItemDialog, WISH_ICONS};
use crate::credits::Credits;
use crate::icons::Icon;
use crate::models::WishItem;
use leptos::prelude::*;
use std::time::Duration;

const ALERT_CANCEL_CLASS: &str = "m3-btn-outlined";
const ALERT_ACTION_CLASS: &str = "m3-btn-filled";

#[component]
pub fn Wishes() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let dialog_open = RwSignal::new(false);
    let editing = RwSignal::new(None::<WishItem>);
    let confirming = RwSignal::new(None::<WishItem>);
    let confirm_open = RwSignal::new(false);
    let celebrate = RwSignal::new(None::<i64>);

    let initial = Signal::derive(move || {
        editing.get().map(|w| (w.name.clone(), w.cost, w.icon.clone()))
    });
    let dialog_title = Signal::derive(move || {
        if editing.get().is_some() { "编辑心愿" } else { "添加兑换心愿" }
    });

    view! {
        <section>
            <div class="mb-4 flex items-end justify-between">
                <div>
                    <h2 class="font-display text-2xl font-bold">"心愿兑换"</h2>
                    <p class="text-sm text-muted-foreground">"攒够积分，就可以兑换心愿啦"</p>
                </div>
                <button
                    class="m3-btn-filled"
                    on:click=move |_| {
                        editing.set(None);
                        dialog_open.set(true);
                    }
                >
                    <Icon name="plus" class="mr-1 h-4 w-4" /> " 新心愿"
                </button>
            </div>

            <div class="grid gap-3 sm:grid-cols-2">
                <For
                    each=move || credits.wishes.get()
                    key=|w| w.id
                    children=move |w: WishItem| {
                        let id = w.id;
                        let cost = w.cost;
                        let icon = w.icon.clone();
                        let name = w.name.clone();
                        let w_edit = w.clone();
                        let w_confirm = w.clone();
                        view! {
                            <div class=move || {
                                let affordable = credits.balance.get() >= cost;
                                format!(
                                    "group relative p-4 transition-all {} {}",
                                    if affordable { "m3-card ring-2 ring-primary" } else { "m3-card" },
                                    if celebrate.get() == Some(id) { "animate-wiggle" } else { "" }
                                )
                            }>
                                <div class="flex items-center gap-3">
                                    <span class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-secondary-container text-2xl">
                                        {icon.clone()}
                                    </span>
                                    <div class="flex-1">
                                        <div class="font-display text-lg font-bold leading-tight">{name.clone()}</div>
                                        <div class="font-display text-sm font-semibold text-on-surface-variant">
                                            "需要 " {cost} " 积分"
                                        </div>
                                    </div>
                                    <button
                                        class=move || {
                                            let affordable = credits.balance.get() >= cost;
                                            format!(
                                                "inline-flex h-9 items-center justify-center gap-1.5 whitespace-nowrap rounded-full px-4 font-display text-sm font-bold transition-all {}",
                                                if affordable {
                                                    "bg-primary text-on-primary shadow-elevation-1 hover:shadow-elevation-2"
                                                } else {
                                                    "bg-surface-container-highest text-on-surface-variant"
                                                }
                                            )
                                        }
                                        disabled=move || credits.balance.get() < cost
                                        on:click=move |_| {
                                            confirming.set(Some(w_confirm.clone()));
                                            confirm_open.set(true);
                                        }
                                    >
                                        <Icon name="gift" class="mr-1 h-4 w-4" />
                                        {move || {
                                            let bal = credits.balance.get();
                                            if bal >= cost {
                                                "兑换".to_string()
                                            } else {
                                                format!("还差 {}", cost - bal)
                                            }
                                        }}
                                    </button>
                                </div>
                                <div class="mt-3 h-2.5 overflow-hidden rounded-full bg-surface-container-highest">
                                    <div
                                        class=move || format!(
                                            "h-full rounded-full transition-all duration-500 {}",
                                            if credits.balance.get() >= cost { "bg-earn" } else { "bg-primary/50" }
                                        )
                                        style=move || {
                                            let bal = credits.balance.get();
                                            let p = if cost > 0 {
                                                (bal as f64 / cost as f64 * 100.0).min(100.0)
                                            } else {
                                                100.0
                                            };
                                            format!("width: {p}%")
                                        }
                                    ></div>
                                </div>
                                <div class="absolute -top-2 right-3 flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                                    <button
                                        class="flex h-7 w-7 items-center justify-center rounded-full bg-surface-container-lowest text-primary shadow-elevation-1 hover:scale-110"
                                        aria-label="编辑"
                                        on:click=move |_| {
                                            editing.set(Some(w_edit.clone()));
                                            dialog_open.set(true);
                                        }
                                    >
                                        <Icon name="pencil" class="h-3.5 w-3.5" />
                                    </button>
                                    <button
                                        class="flex h-7 w-7 items-center justify-center rounded-full bg-surface-container-lowest text-error shadow-elevation-1 hover:scale-110"
                                        aria-label="删除"
                                        on:click=move |_| credits.remove_wish(id)
                                    >
                                        <Icon name="trash-2" class="h-3.5 w-3.5" />
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            <ItemDialog
                open=dialog_open
                title=dialog_title
                value_label="兑换所需积分"
                icons=WISH_ICONS
                initial=initial
                on_save=move |(name, value, icon): (String, i64, String)| {
                    match editing.get_untracked() {
                        Some(w) => credits.update_wish(w.id, &name, value, &icon),
                        None => credits.add_wish(&name, value, &icon),
                    }
                }
            />

            <Dialog open=confirm_open show_close=false>
                <div class="flex flex-col gap-2 text-center sm:text-left">
                    <h2 class="text-lg font-semibold font-display text-xl">
                        "兑换「" {move || confirming.get().map(|w| w.name).unwrap_or_default()} "」？"
                    </h2>
                    <p class="text-muted-foreground text-sm">
                        "将消耗 "
                        <span class="font-display font-bold text-primary">
                            {move || confirming.get().map(|w| w.cost).unwrap_or(0)}
                        </span>
                        " 积分，兑换后剩余 "
                        {move || {
                            let cost = confirming.get().map(|w| w.cost).unwrap_or(0);
                            credits.balance.get() - cost
                        }}
                        " 积分。"
                    </p>
                </div>
                <div class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                    <button class=ALERT_CANCEL_CLASS on:click=move |_| confirm_open.set(false)>
                        再想想
                    </button>
                    <button
                        class=ALERT_ACTION_CLASS
                        on:click=move |_| {
                            if let Some(w) = confirming.get() {
                                credits.redeem_wish(w.id);
                                celebrate.set(Some(w.id));
                                set_timeout(move || celebrate.set(None), Duration::from_millis(500));
                            }
                            confirm_open.set(false);
                        }
                    >
                        确定兑换
                    </button>
                </div>
            </Dialog>
        </section>
    }
}
