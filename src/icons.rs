use leptos::prelude::*;

fn paths(name: &str) -> &'static [&'static str] {
    match name {
        "check" => &["M20 6 9 17l-5-5"],
        "x" => &["M18 6 6 18", "m6 6 12 12"],
        "plus" => &["M5 12h14", "M12 5v14"],
        "pencil" => &[
            "M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z",
            "m15 5 4 4",
        ],
        "trash-2" => &[
            "M3 6h18",
            "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6",
            "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2",
            "M10 11v6",
            "M14 11v6",
        ],
        "gift" => &[
            "M4 8h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1z",
            "M12 8v13",
            "M19 12v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-7",
            "M7.5 8a2.5 2.5 0 0 1 0-5A4.8 4.8 0 0 1 12 8a4.8 4.8 0 0 1 4.5-5 2.5 2.5 0 0 1 0 5",
        ],
        "chevron-left" => &["m15 18-6-6 6-6"],
        "chevron-right" => &["m9 18 6-6-6-6"],
        "download" => &[
            "M12 15V3",
            "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4",
            "m7 10 5 5 5-5",
        ],
        "upload" => &[
            "M12 3v12",
            "m17 8-5-5-5 5",
            "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4",
        ],
        "bar-chart-3" => &[
            "M3 3v16a2 2 0 0 0 2 2h16",
            "M18 17V9",
            "M13 17V5",
            "M8 17v-3",
        ],
        "calendar-check" => &[
            "M5 4h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z",
            "M8 2v4",
            "M16 2v4",
            "M3 10h18",
            "m9 16 2 2 4-4",
        ],
        "calendar-days" => &[
            "M5 4h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z",
            "M8 2v4",
            "M16 2v4",
            "M3 10h18",
            "M8 14h.01", "M12 14h.01", "M16 14h.01",
            "M8 18h.01", "M12 18h.01", "M16 18h.01",
        ],
        "scroll-text" => &[
            "M15 12h-5",
            "M15 8h-5",
            "M19 17V5a2 2 0 0 0-2-2H4",
            "M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3",
        ],
        "sun" => &[
            "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
            "M12 2v2", "M12 20v2",
            "m4.93 4.93 1.41 1.41", "m17.66 17.66 1.41 1.41",
            "M2 12h2", "M20 12h2",
            "m6.34 17.66-1.41 1.41", "m19.07 4.93-1.41 1.41",
        ],
        "moon" => &["M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"],
        _ => &[],
    }
}

/// 内联 SVG 图标（lucide 风格：24x24，stroke=currentColor）
#[component]
pub fn Icon(
    #[prop(into)] name: &'static str,
    #[prop(into)] class: &'static str,
    #[prop(default = 2.0)] stroke_width: f64,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
        >
            {paths(name).iter().map(|d| view! { <path d=*d /> }).collect_view()}
        </svg>
    }
}
