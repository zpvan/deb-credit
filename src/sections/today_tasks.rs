use crate::components::item_dialog::{ItemDialog, TASK_ICONS};
use crate::credits::Credits;
use crate::icons::Icon;
use crate::models::EarnTask;
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn TodayTasks() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let dialog_open = RwSignal::new(false);
    let editing = RwSignal::new(None::<EarnTask>);
    let just_done = RwSignal::new(None::<i64>);

    let done_count = Memo::new(move |_| {
        let ts = credits.tasks.get();
        ts.iter().filter(|t| credits.is_task_done(t.id)).count()
    });
    let earned_today = Memo::new(move |_| {
        let ts = credits.tasks.get();
        ts.iter()
            .filter(|t| credits.is_task_done(t.id))
            .map(|t| t.credit)
            .sum::<i64>()
    });

    let initial = Signal::derive(move || {
        editing.get().map(|t| (t.name.clone(), t.credit, t.icon.clone()))
    });
    let dialog_title = Signal::derive(move || {
        if editing.get().is_some() { "编辑任务" } else { "添加赚积分任务" }
    });

    view! {
        <section>
            <div class="mb-4 flex items-end justify-between">
                <div>
                    <h2 class="font-display text-2xl font-bold">"今日打卡"</h2>
                    <p class="text-sm text-muted-foreground">
                        "已完成 " {move || done_count.get()} "/" {move || credits.tasks.get().len()}
                        " 项，今天赚到 "
                        <span class="font-display font-bold text-[#f8622f]">
                            "+" {move || earned_today.get()}
                        </span>
                        " 积分"
                    </p>
                </div>
                <button
                    class="inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all h-9 px-4 py-2 rounded-full bg-[#1d5d3f] hover:bg-[#164a32] text-white font-display"
                    on:click=move |_| {
                        editing.set(None);
                        dialog_open.set(true);
                    }
                >
                    <Icon name="plus" class="mr-1 h-4 w-4" /> " 新任务"
                </button>
            </div>

            <div class="grid gap-3 sm:grid-cols-2">
                <For
                    each=move || credits.tasks.get()
                    key=|t| t.id
                    children=move |t: EarnTask| {
                        let id = t.id;
                        let icon = t.icon.clone();
                        let name = t.name.clone();
                        let credit = t.credit;
                        let t_edit = t.clone();
                        view! {
                            <div class=move || {
                                let done = credits.is_task_done(id);
                                format!(
                                    "group relative flex items-center gap-3 rounded-3xl border-2 p-4 transition-all duration-300 {} {}",
                                    if done {
                                        "border-[#1d5d3f]/20 bg-[#1d5d3f] text-white shadow-md"
                                    } else {
                                        "border-border bg-card hover:border-[#ecc22e] hover:shadow-sm"
                                    },
                                    if just_done.get() == Some(id) { "animate-pop-in" } else { "" }
                                )
                            }>
                                <button
                                    class="flex flex-1 items-center gap-3 text-left"
                                    on:click=move |_| {
                                        let was_done = credits.is_task_done(id);
                                        credits.toggle_task(id);
                                        if !was_done {
                                            just_done.set(Some(id));
                                            set_timeout(
                                                move || just_done.set(None),
                                                Duration::from_millis(400),
                                            );
                                        }
                                    }
                                >
                                    <span class=move || format!(
                                        "flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl text-2xl {}",
                                        if credits.is_task_done(id) { "bg-white/15" } else { "bg-[#feffc9]" }
                                    )>
                                        {icon.clone()}
                                    </span>
                                    <span class="flex-1">
                                        <span class=move || format!(
                                            "block font-display text-lg font-bold leading-tight {}",
                                            if credits.is_task_done(id) { "line-through opacity-80" } else { "" }
                                        )>
                                            {name.clone()}
                                        </span>
                                        <span class=move || format!(
                                            "font-display text-sm font-semibold {}",
                                            if credits.is_task_done(id) { "text-[#ecc22e]" } else { "text-[#f8622f]" }
                                        )>
                                            "+" {credit} " 积分"
                                        </span>
                                    </span>
                                    <span class=move || format!(
                                        "flex h-8 w-8 shrink-0 items-center justify-center rounded-full border-2 transition-all {}",
                                        if credits.is_task_done(id) {
                                            "border-[#ecc22e] bg-[#ecc22e] text-[#1d5d3f]"
                                        } else {
                                            "border-muted-foreground/30 text-transparent"
                                        }
                                    )>
                                        <Icon name="check" class="h-5 w-5" stroke_width=3.5 />
                                    </span>
                                </button>
                                <div class="absolute -top-2 right-3 flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                                    <button
                                        class="flex h-7 w-7 items-center justify-center rounded-full bg-white text-[#4c87bf] shadow hover:scale-110"
                                        aria-label="编辑"
                                        on:click=move |_| {
                                            editing.set(Some(t_edit.clone()));
                                            dialog_open.set(true);
                                        }
                                    >
                                        <Icon name="pencil" class="h-3.5 w-3.5" />
                                    </button>
                                    <button
                                        class="flex h-7 w-7 items-center justify-center rounded-full bg-white text-[#872020] shadow hover:scale-110"
                                        aria-label="删除"
                                        on:click=move |_| credits.remove_task(id)
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
                value_label="完成可获得的积分"
                icons=TASK_ICONS
                initial=initial
                on_save=move |(name, value, icon): (String, i64, String)| {
                    match editing.get_untracked() {
                        Some(t) => credits.update_task(t.id, &name, value, &icon),
                        None => credits.add_task(&name, value, &icon),
                    }
                }
            />
        </section>
    }
}
