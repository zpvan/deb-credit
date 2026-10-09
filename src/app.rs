use crate::components::dialog::Dialog;
use crate::components::kid_dialog::KidDialog;
use crate::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use crate::credits::Credits;
use crate::i18n::{delete_kid_title, import_error_msg, import_success_msg, initial_lang, t, K};
use crate::icons::Icon;
use crate::models::Kid;
use crate::sections::calendar_board::CalendarBoard;
use crate::sections::charts::Charts;
use crate::sections::history::History;
use crate::sections::today_tasks::TodayTasks;
use crate::sections::wishes::Wishes;
use crate::store::BackupError;
use chrono::Local;
use leptos::prelude::*;

const HEADER_BTN: &str = "m3-btn-outlined";
const ALERT_CANCEL_CLASS: &str = "m3-btn-outlined";

#[component]
pub fn App() -> impl IntoView {
    let credits = Credits::new();
    provide_context(credits);

    let lang = RwSignal::new(initial_lang());
    provide_context(lang);
    Effect::new(move |_| {
        let l = lang.get();
        if let Some(el) = document().document_element() {
            let _ = el.set_attribute("lang", l.as_str());
        }
        document().set_title(t(l, K::AppTitle));
        if let Some(storage) = window().local_storage().ok().flatten() {
            let _ = storage.set_item("lang", l.as_str());
        }
    });

    let kid_dialog_open = RwSignal::new(false);
    let editing_kid = RwSignal::new(None::<Kid>);
    let deleting_kid = RwSignal::new(None::<Kid>);
    let delete_open = RwSignal::new(false);
    let import_error = RwSignal::new(None::<String>);
    let import_success = RwSignal::new(None::<i64>);
    let file_ref = NodeRef::<leptos::html::Input>::new();

    let dark_mode = RwSignal::new(
        document()
            .document_element()
            .map(|el| el.class_list().contains("dark"))
            .unwrap_or(false),
    );
    Effect::new(move |_| {
        let d = dark_mode.get();
        if let Some(el) = document().document_element() {
            let cl = el.class_list();
            if d {
                let _ = cl.add_1("dark");
            } else {
                let _ = cl.remove_1("dark");
            }
        }
        if let Some(storage) = window().local_storage().ok().flatten() {
            let _ = storage.set_item("theme", if d { "dark" } else { "light" });
        }
    });

    let kid_initial = Signal::derive(move || {
        editing_kid.get().map(|k| (k.name.clone(), k.avatar.clone()))
    });

    let on_file = move |ev: leptos::ev::Event| {
        let input: web_sys::HtmlInputElement = event_target(&ev);
        let Some(files) = input.files() else { return };
        let Some(file) = files.get(0) else { return };
        leptos::task::spawn_local(async move {
            let result = match crate::persist::read_file_text(&file).await {
                Ok(text) => crate::store::parse_backup(&text).map_err(|e| match e {
                    BackupError::BadFormat => t(lang.get_untracked(), K::ErrBadFormat).to_string(),
                    BackupError::BadKids => t(lang.get_untracked(), K::ErrBadKids).to_string(),
                    BackupError::BadRecords => t(lang.get_untracked(), K::ErrBadRecords).to_string(),
                }),
                Err(msg) => Err(msg),
            };
            match result {
                Ok(new_store) => {
                    let n = new_store.kids.len() as i64;
                    credits.import_store(new_store);
                    import_error.set(None);
                    import_success.set(Some(n));
                }
                Err(msg) => {
                    import_success.set(None);
                    import_error.set(Some(msg));
                }
            }
            if let Some(el) = file_ref.get() {
                el.set_value("");
            }
        });
    };

    view! {
        <div class="min-h-screen pb-16">
            // decorative blobs
            <div class="pointer-events-none fixed -left-24 -top-24 h-72 w-72 rounded-full bg-earn-container/60 blur-3xl"></div>
            <div class="pointer-events-none fixed -right-24 top-40 h-80 w-80 rounded-full bg-primary-container/60 blur-3xl"></div>

            <header class="relative mx-auto max-w-3xl px-4 pt-10">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="font-display text-sm font-bold tracking-widest text-on-surface-variant">{move || crate::date::header_date_line(lang.get(), Local::now().date_naive())}</div>
                        <h1 class="font-display text-4xl font-extrabold tracking-tight">{move || t(lang.get(), K::AppTitle)}</h1>
                    </div>
                    <div class="flex items-center gap-2">
                        <button
                            class="m3-icon-btn w-auto px-2.5 font-display text-sm font-bold"
                            title="Switch language / 切换语言"
                            on:click=move |_| lang.update(|l| *l = l.toggle())
                        >
                            {move || lang.get().label()}
                        </button>
                        <button
                            class="m3-icon-btn"
                            title=move || t(lang.get(), K::ToggleDark)
                            on:click=move |_| dark_mode.update(|v| *v = !*v)
                        >
                            {move || if dark_mode.get() {
                                view! { <Icon name="sun" class="h-5 w-5" /> }.into_any()
                            } else {
                                view! { <Icon name="moon" class="h-5 w-5" /> }.into_any()
                            }}
                        </button>
                        <button
                            class=HEADER_BTN
                            title=move || t(lang.get(), K::ExportTitle)
                            on:click=move |_| credits.store.with(|s| crate::persist::export_store(s))
                        >
                            <Icon name="download" class="h-4 w-4" /> " " {move || t(lang.get(), K::Export)}
                        </button>
                        <button
                            class=HEADER_BTN
                            title=move || t(lang.get(), K::ImportTitle)
                            on:click=move |_| {
                                if let Some(el) = file_ref.get() {
                                    el.click();
                                }
                            }
                        >
                            <Icon name="upload" class="h-4 w-4" /> " " {move || t(lang.get(), K::Import)}
                        </button>
                        <input
                            node_ref=file_ref
                            type="file"
                            accept="application/json,.json"
                            class="hidden"
                            on:change=on_file
                        />
                        <span class="animate-float-coin text-5xl">"🪙"</span>
                    </div>
                </div>

                {move || import_error.get().map(|msg| view! {
                    <div class="mt-3 rounded-2xl bg-error-container px-4 py-2.5 font-display text-sm font-bold text-on-error-container">
                        {import_error_msg(lang.get(), &msg)}
                    </div>
                })}
                {move || import_success.get().map(|n| view! {
                    <div class="mt-3 rounded-2xl bg-earn-container px-4 py-2.5 font-display text-sm font-bold text-on-earn-container">
                        {import_success_msg(lang.get(), n)}
                    </div>
                })}

                // kid switcher
                <div class="mt-5 flex flex-wrap items-center gap-2">
                    <For
                        each=move || credits.kids.get()
                        key=|k| k.id
                        children=move |k: Kid| {
                            let id = k.id;
                            let name = k.name.clone();
                            let avatar = k.avatar.clone();
                            let k_edit = k.clone();
                            let k_del = k.clone();
                            view! {
                                <div class="group relative">
                                    <button
                                        class=move || format!(
                                            "flex items-center gap-2 rounded-full border py-1.5 pl-2 pr-4 font-display font-bold transition-all {}",
                                            if credits.selected_kid_id.get() == Some(id) {
                                                "border-transparent bg-primary text-on-primary shadow-elevation-1"
                                            } else {
                                                "border-outline-variant bg-surface-container-lowest hover:bg-secondary-container"
                                            }
                                        )
                                        on:click=move |_| credits.selected_kid_id.set(Some(id))
                                    >
                                        <span class=move || format!(
                                            "flex h-8 w-8 items-center justify-center rounded-full text-lg {}",
                                            if credits.selected_kid_id.get() == Some(id) { "bg-on-primary/15" } else { "bg-secondary-container" }
                                        )>
                                            {avatar.clone()}
                                        </span>
                                        {name.clone()}
                                    </button>
                                    {move || (credits.selected_kid_id.get() == Some(id)).then(|| {
                                        let ke = k_edit.clone();
                                        let kd = k_del.clone();
                                        view! {
                                            <div class="absolute -top-3 right-0 flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                                                <button
                                                    class="flex h-6 w-6 items-center justify-center rounded-full bg-surface-container-lowest text-primary shadow-elevation-1 hover:scale-110"
                                                    aria-label=move || t(lang.get(), K::EditKid)
                                                    on:click=move |_| {
                                                        editing_kid.set(Some(ke.clone()));
                                                        kid_dialog_open.set(true);
                                                    }
                                                >
                                                    <Icon name="pencil" class="h-3 w-3" />
                                                </button>
                                                <button
                                                    class="flex h-6 w-6 items-center justify-center rounded-full bg-surface-container-lowest text-error shadow-elevation-1 hover:scale-110"
                                                    aria-label=move || t(lang.get(), K::DeleteKid)
                                                    on:click=move |_| {
                                                        deleting_kid.set(Some(kd.clone()));
                                                        delete_open.set(true);
                                                    }
                                                >
                                                    <Icon name="trash-2" class="h-3 w-3" />
                                                </button>
                                            </div>
                                        }
                                    })}
                                </div>
                            }
                        }
                    />
                    <button
                        class="flex items-center gap-1 rounded-full border border-dashed border-outline px-4 py-2 font-display font-bold text-primary transition-all hover:bg-secondary-container"
                        on:click=move |_| {
                            editing_kid.set(None);
                            kid_dialog_open.set(true);
                        }
                    >
                        <Icon name="plus" class="h-4 w-4" /> " " {move || t(lang.get(), K::AddKid)}
                    </button>
                </div>

                // balance hero
                {move || credits.kid.get().map(|k| {
                    let done_count = {
                        let ts = credits.tasks.get();
                        ts.iter().filter(|t| credits.is_task_done(t.id)).count()
                    };
                    let task_count = credits.tasks.get().len();
                    view! {
                        <div class="mt-6 overflow-hidden rounded-[2rem] bg-primary p-6 text-on-primary shadow-elevation-2">
                            <div class="flex flex-wrap items-end justify-between gap-4">
                                <div>
                                    <div class="font-display text-sm font-bold tracking-widest text-on-primary/80">
                                        {k.avatar} " " {k.name} {move || t(lang.get(), K::HeroCreditsSuffix)}
                                    </div>
                                    <div class="font-display text-6xl font-extrabold leading-none text-on-primary">
                                        {credits.balance.get()}
                                    </div>
                                </div>
                                <div class="flex gap-6">
                                    <div>
                                        <div class="text-xs text-on-primary/70">{move || t(lang.get(), K::TotalEarned)}</div>
                                        <div class="font-display text-2xl font-bold text-on-primary">
                                            "+" {credits.total_earned.get()}
                                        </div>
                                    </div>
                                    <div>
                                        <div class="text-xs text-on-primary/70">{move || t(lang.get(), K::TotalSpent)}</div>
                                        <div class="font-display text-2xl font-bold text-on-primary">
                                            "−" {credits.total_spent.get()}
                                        </div>
                                    </div>
                                    <div>
                                        <div class="text-xs text-on-primary/70">{move || t(lang.get(), K::HeroToday)}</div>
                                        <div class="font-display text-2xl font-bold text-on-primary">
                                            {done_count} "/" {task_count}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </header>

            <main class="relative mx-auto mt-8 max-w-3xl px-4">
                {move || {
                    if credits.kid.get().is_none() {
                        view! {
                            <div class="rounded-3xl border-2 border-dashed border-outline-variant bg-surface-container-low/50 p-14 text-center text-on-surface-variant">
                                <div class="mb-3 text-5xl">"👋"</div>
                                <p class="font-display text-lg font-bold">{move || t(lang.get(), K::EmptyWelcome)}</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <Tabs default_value="today">
                                <TabsList class="grid h-auto w-full grid-cols-5 rounded-full bg-surface-container-low p-1.5">
                                    <TabsTrigger value="today" class="rounded-full py-2 font-display font-bold" active_class="bg-primary text-on-primary shadow-elevation-1">
                                        <Icon name="calendar-check" class="mr-1.5 h-4 w-4" />{move || t(lang.get(), K::TabCheckin)}
                                    </TabsTrigger>
                                    <TabsTrigger value="wishes" class="rounded-full py-2 font-display font-bold" active_class="bg-primary text-on-primary shadow-elevation-1">
                                        <Icon name="gift" class="mr-1.5 h-4 w-4" />{move || t(lang.get(), K::TabWishes)}
                                    </TabsTrigger>
                                    <TabsTrigger value="calendar" class="rounded-full py-2 font-display font-bold" active_class="bg-primary text-on-primary shadow-elevation-1">
                                        <Icon name="calendar-days" class="mr-1.5 h-4 w-4" />{move || t(lang.get(), K::TabCalendar)}
                                    </TabsTrigger>
                                    <TabsTrigger value="charts" class="rounded-full py-2 font-display font-bold" active_class="bg-primary text-on-primary shadow-elevation-1">
                                        <Icon name="bar-chart-3" class="mr-1.5 h-4 w-4" />{move || t(lang.get(), K::TabCharts)}
                                    </TabsTrigger>
                                    <TabsTrigger value="history" class="rounded-full py-2 font-display font-bold" active_class="bg-primary text-on-primary shadow-elevation-1">
                                        <Icon name="scroll-text" class="mr-1.5 h-4 w-4" />{move || t(lang.get(), K::TabHistory)}
                                    </TabsTrigger>
                                </TabsList>
                                <TabsContent value="today" class="mt-6"><TodayTasks /></TabsContent>
                                <TabsContent value="wishes" class="mt-6"><Wishes /></TabsContent>
                                <TabsContent value="calendar" class="mt-6"><CalendarBoard /></TabsContent>
                                <TabsContent value="charts" class="mt-6"><Charts /></TabsContent>
                                <TabsContent value="history" class="mt-6"><History /></TabsContent>
                            </Tabs>
                        }.into_any()
                    }
                }}
            </main>

            <KidDialog
                open=kid_dialog_open
                initial=kid_initial
                on_save=move |(name, avatar): (String, String)| {
                    match editing_kid.get_untracked() {
                        Some(k) => credits.update_kid(k.id, &name, &avatar),
                        None => credits.create_kid(&name, &avatar),
                    }
                }
            />

            <Dialog open=delete_open show_close=false>
                <div class="flex flex-col gap-2 text-center sm:text-left">
                    <h2 class="text-lg font-semibold font-display text-xl">
                        {move || delete_kid_title(lang.get(), &deleting_kid.get().map(|k| k.name).unwrap_or_default())}
                    </h2>
                    <p class="text-on-surface-variant text-sm">
                        {move || t(lang.get(), K::DeleteKidBody)}
                    </p>
                </div>
                <div class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                    <button class=ALERT_CANCEL_CLASS on:click=move |_| delete_open.set(false)>
                        {move || t(lang.get(), K::Cancel)}
                    </button>
                    <button
                        class="m3-btn-danger"
                        on:click=move |_| {
                            if let Some(k) = deleting_kid.get() {
                                credits.delete_kid(k.id);
                            }
                            delete_open.set(false);
                        }
                    >
                        {move || t(lang.get(), K::ConfirmDelete)}
                    </button>
                </div>
            </Dialog>
        </div>
    }
}
