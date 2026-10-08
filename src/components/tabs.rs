use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct TabsContext {
    pub value: RwSignal<String>,
}

fn use_tabs() -> TabsContext {
    use_context::<TabsContext>().expect("TabsTrigger/TabsContent 必须放在 <Tabs> 内")
}

#[component]
pub fn Tabs(
    #[prop(into)] default_value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let value = RwSignal::new(default_value);
    provide_context(TabsContext { value });
    view! { <div class="flex flex-col gap-2">{children()}</div> }
}

#[component]
pub fn TabsList(
    #[prop(into, optional)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div role="tablist" class=format!(
            "bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px] {class}"
        )>
            {children()}
        </div>
    }
}

const TRIGGER_BASE: &str = "inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap transition-[color,box-shadow] disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] active_class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_tabs();
    let v = value.clone();
    view! {
        <button
            role="tab"
            class=move || {
                if ctx.value.get() == v {
                    format!("{TRIGGER_BASE} {class} {active_class}")
                } else {
                    format!("{TRIGGER_BASE} {class}")
                }
            }
            on:click=move |_| ctx.value.set(value.clone())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_tabs();
    view! {
        <div role="tabpanel" class=format!("flex-1 outline-none {class}") hidden=move || ctx.value.get() != value>
            {children()}
        </div>
    }
}
