use crate::components::item_dialog::{ItemDialog, TASK_ICONS};
use crate::credits::Credits;
use crate::i18n::{t, today_progress_pre, today_progress_suffix, use_lang, K};
use crate::icons::Icon;
use crate::models::EarnTask;
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn TodayTasks() -> impl IntoView {
    let credits = use_context::<Credits>().expect("Credits context");
    let lang = use_lang();
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
        if editing.get().is_some() { t(lang.get(), K::EditTask) } else { t(lang.get(), K::AddTask) }
    });

    view! {
        <section>
            <div class="mb-4 flex items-end justify-between">
                <div>
                    <h2 class="font-display text-2xl font-bold">{move || t(lang.get(), K::TodayTitle)}</h2>
                    <p class="text-sm text-on-surface-variant">
                        {move || today_progress_pre(lang.get(), done_count.get(), credits.tasks.get().len())}
                        <span class="font-display font-bold text-primary">
                            "+" {move || earned_today.get()}
                        </span>
                        {move || today_progress_suffix(lang.get())}
                    </p>
                </div>
                <button
                    class="m3-btn-filled"
                    on:click=move |_| {
                        editing.set(None);
                        dialog_open.set(true);
                    }
                >
                    <Icon name="plus" class="mr-1 h-4 w-4" /> " " {move || t(lang.get(), K::NewTask)}
                </button>
            </div>

            <div class="grid gap-3 sm:grid-cols-2">
                <For
                    each=move || credits.tasks.get()
                    key=|t| t.id
                    children=move |task: EarnTask| {
                        let id = task.id;
                        let icon = task.icon.clone();
                        let name = task.name.clone();
                        let credit = task.credit;
                        let t_edit = task.clone();
                        view! {
                            <div class=move || {
                                let done = credits.is_task_done(id);
                                format!(
                                    "group relative flex items-center gap-3 p-4 transition-all duration-300 {} {}",
                                    if done {
                                        "rounded-2xl bg-earn text-on-earn shadow-elevation-1"
                                    } else {
                                        "m3-card hover:shadow-elevation-2"
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
                                        if credits.is_task_done(id) { "bg-on-earn/15" } else { "bg-secondary-container" }
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
                                            if credits.is_task_done(id) { "text-on-earn/80" } else { "text-primary" }
                                        )>
                                            "+" {credit} " " {move || t(lang.get(), K::CreditsUnit)}
                                        </span>
                                    </span>
                                    <span class=move || format!(
                                        "flex h-8 w-8 shrink-0 items-center justify-center rounded-full border-2 transition-all {}",
                                        if credits.is_task_done(id) {
                                            "border-transparent bg-on-earn text-earn"
                                        } else {
                                            "border-outline/50 text-transparent"
                                        }
                                    )>
                                        <Icon name="check" class="h-5 w-5" stroke_width=3.5 />
                                    </span>
                                </button>
                                <div class="absolute -top-2 right-3 flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                                    <button
                                        class="flex h-7 w-7 items-center justify-center rounded-full bg-surface-container-lowest text-primary shadow-elevation-1 hover:scale-110"
                                        aria-label=move || t(lang.get(), K::Edit)
                                        on:click=move |_| {
                                            editing.set(Some(t_edit.clone()));
                                            dialog_open.set(true);
                                        }
                                    >
                                        <Icon name="pencil" class="h-3.5 w-3.5" />
                                    </button>
                                    <button
                                        class="flex h-7 w-7 items-center justify-center rounded-full bg-surface-container-lowest text-error shadow-elevation-1 hover:scale-110"
                                        aria-label=move || t(lang.get(), K::Delete)
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
                value_label=t(lang.get_untracked(), K::TaskValueLabel)
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
