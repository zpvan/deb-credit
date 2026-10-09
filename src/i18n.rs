use leptos::prelude::*;

/// 界面语言。默认英文；localStorage `lang` 持久化。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    En,
    Zh,
}

impl Lang {
    pub fn as_str(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Zh => "zh",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "zh" => Lang::Zh,
            _ => Lang::En,
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Lang::En => Lang::Zh,
            Lang::Zh => Lang::En,
        }
    }

    /// 语言切换按钮上显示的名字
    pub fn label(self) -> &'static str {
        match self {
            Lang::En => "EN",
            Lang::Zh => "中文",
        }
    }
}

/// 从 context 取当前语言信号（App 在 setup 中 provide）。
pub fn use_lang() -> RwSignal<Lang> {
    use_context::<RwSignal<Lang>>().expect("Lang context")
}

/// 从 localStorage 读初始语言，无记录默认英文。
pub fn initial_lang() -> Lang {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item("lang").ok().flatten())
        .map(|v| Lang::from_str(&v))
        .unwrap_or(Lang::En)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    AppTitle,
    ToggleDark,
    Export,
    ExportTitle,
    Import,
    ImportTitle,
    EditKid,
    DeleteKid,
    AddKid,
    HeroCreditsSuffix,
    TotalEarned,
    TotalSpent,
    HeroToday,
    EmptyWelcome,
    TabCheckin,
    TabWishes,
    TabCalendar,
    TabCharts,
    TabHistory,
    Cancel,
    ConfirmDelete,
    DeleteKidBody,
    TodayTitle,
    NewTask,
    CreditsUnit,
    Edit,
    Delete,
    EditTask,
    AddTask,
    TaskValueLabel,
    WishesTitle,
    WishesSubtitle,
    NewWish,
    Redeem,
    NotYet,
    ConfirmRedeem,
    EditWish,
    AddWish,
    WishValueLabel,
    CalendarTitle,
    PrevMonth,
    NextMonth,
    AllDone,
    LegendIntensity,
    EarnedWord,
    SpentWord,
    DetailsSuffix,
    NoRecords,
    HistoryTitle,
    HistoryEmpty,
    ChartsCompareTitle,
    ChartsCompareSubtitle,
    ChartsWeekTitle,
    ChartsWeekSubtitle,
    SvgCurrentCredits,
    SvgEarned,
    SvgSpent,
    Today,
    FieldName,
    KidNamePlaceholder,
    ItemNamePlaceholder,
    PickIcon,
    PickAvatar,
    Save,
    ErrBadFormat,
    ErrBadKids,
    ErrBadRecords,
}

pub fn t(lang: Lang, key: K) -> &'static str {
    match key {
        K::AppTitle => match lang { Lang::En => "Kids Credit Station", Lang::Zh => "宝贝积分站" },
        K::ToggleDark => match lang { Lang::En => "Toggle dark mode", Lang::Zh => "切换深色模式" },
        K::Export => match lang { Lang::En => "Export", Lang::Zh => "导出" },
        K::ExportTitle => match lang { Lang::En => "Export all data as a JSON file", Lang::Zh => "导出全部数据为 JSON 文件" },
        K::Import => match lang { Lang::En => "Import", Lang::Zh => "导入" },
        K::ImportTitle => match lang { Lang::En => "Restore data from a JSON file", Lang::Zh => "从 JSON 文件恢复数据" },
        K::EditKid => match lang { Lang::En => "Edit kid", Lang::Zh => "编辑宝贝" },
        K::DeleteKid => match lang { Lang::En => "Delete kid", Lang::Zh => "删除宝贝" },
        K::AddKid => match lang { Lang::En => "Add Kid", Lang::Zh => "添加宝贝" },
        K::HeroCreditsSuffix => match lang { Lang::En => "'s Credits", Lang::Zh => " 的当前积分" },
        K::TotalEarned => match lang { Lang::En => "Total Earned", Lang::Zh => "累计赚得" },
        K::TotalSpent => match lang { Lang::En => "Total Spent", Lang::Zh => "累计花掉" },
        K::HeroToday => match lang { Lang::En => "Today", Lang::Zh => "今日打卡" },
        K::EmptyWelcome => match lang { Lang::En => "Add a kid to start earning credits!", Lang::Zh => "先添加一个宝贝，开始攒积分吧！" },
        K::TabCheckin => match lang { Lang::En => "Check-in", Lang::Zh => "打卡" },
        K::TabWishes => match lang { Lang::En => "Wishes", Lang::Zh => "心愿" },
        K::TabCalendar => match lang { Lang::En => "Calendar", Lang::Zh => "日历" },
        K::TabCharts => match lang { Lang::En => "Charts", Lang::Zh => "图表" },
        K::TabHistory => match lang { Lang::En => "History", Lang::Zh => "记录" },
        K::Cancel => match lang { Lang::En => "Cancel", Lang::Zh => "取消" },
        K::ConfirmDelete => match lang { Lang::En => "Delete", Lang::Zh => "确定删除" },
        K::DeleteKidBody => match lang { Lang::En => "This will also delete all tasks, wishes, and credit records of this kid. This cannot be undone.", Lang::Zh => "将同时删除该宝贝的所有任务、心愿和积分记录，此操作不可恢复。" },
        K::TodayTitle => match lang { Lang::En => "Today's Check-ins", Lang::Zh => "今日打卡" },
        K::NewTask => match lang { Lang::En => "New Task", Lang::Zh => "新任务" },
        K::CreditsUnit => match lang { Lang::En => "credits", Lang::Zh => "积分" },
        K::Edit => match lang { Lang::En => "Edit", Lang::Zh => "编辑" },
        K::Delete => match lang { Lang::En => "Delete", Lang::Zh => "删除" },
        K::EditTask => match lang { Lang::En => "Edit Task", Lang::Zh => "编辑任务" },
        K::AddTask => match lang { Lang::En => "Add Earn Task", Lang::Zh => "添加赚积分任务" },
        K::TaskValueLabel => match lang { Lang::En => "Credits earned on completion", Lang::Zh => "完成可获得的积分" },
        K::WishesTitle => match lang { Lang::En => "Wish Shop", Lang::Zh => "心愿兑换" },
        K::WishesSubtitle => match lang { Lang::En => "Save up credits to redeem wishes", Lang::Zh => "攒够积分，就可以兑换心愿啦" },
        K::NewWish => match lang { Lang::En => "New Wish", Lang::Zh => "新心愿" },
        K::Redeem => match lang { Lang::En => "Redeem", Lang::Zh => "兑换" },
        K::NotYet => match lang { Lang::En => "Not yet", Lang::Zh => "再想想" },
        K::ConfirmRedeem => match lang { Lang::En => "Redeem", Lang::Zh => "确定兑换" },
        K::EditWish => match lang { Lang::En => "Edit Wish", Lang::Zh => "编辑心愿" },
        K::AddWish => match lang { Lang::En => "Add Wish", Lang::Zh => "添加兑换心愿" },
        K::WishValueLabel => match lang { Lang::En => "Credits required", Lang::Zh => "兑换所需积分" },
        K::CalendarTitle => match lang { Lang::En => "Calendar", Lang::Zh => "日历看板" },
        K::PrevMonth => match lang { Lang::En => "Previous month", Lang::Zh => "上个月" },
        K::NextMonth => match lang { Lang::En => "Next month", Lang::Zh => "下个月" },
        K::AllDone => match lang { Lang::En => "All done", Lang::Zh => "全勤" },
        K::LegendIntensity => match lang { Lang::En => "Green depth = credits earned", Lang::Zh => "绿色深浅 = 赚得多少" },
        K::EarnedWord => match lang { Lang::En => "Earned", Lang::Zh => "赚" },
        K::SpentWord => match lang { Lang::En => "Spent", Lang::Zh => "花" },
        K::DetailsSuffix => match lang { Lang::En => " Details", Lang::Zh => " 明细" },
        K::NoRecords => match lang { Lang::En => "No records on this day", Lang::Zh => "这一天没有记录" },
        K::HistoryTitle => match lang { Lang::En => "Credit History", Lang::Zh => "积分记录" },
        K::HistoryEmpty => match lang { Lang::En => "No records yet. Finish today's tasks to earn your first credits!", Lang::Zh => "还没有记录，去完成今天的任务赚第一笔积分吧！" },
        K::ChartsCompareTitle => match lang { Lang::En => "Credits vs Wishes", Lang::Zh => "积分 vs 心愿" },
        K::ChartsCompareSubtitle => match lang { Lang::En => "The orange bar is your current credits; colored bars are what each wish costs.", Lang::Zh => "橙色是当前攒下的积分，彩色是每个心愿需要的积分，一眼看出还差多少" },
        K::ChartsWeekTitle => match lang { Lang::En => "Last 7 Days", Lang::Zh => "最近 7 天收支" },
        K::ChartsWeekSubtitle => match lang { Lang::En => "Green is credits earned each day; orange is credits spent.", Lang::Zh => "绿色是每天赚到的积分，橙色是花掉的积分" },
        K::SvgCurrentCredits => match lang { Lang::En => "💰 Current credits", Lang::Zh => "💰 当前积分" },
        K::SvgEarned => match lang { Lang::En => "earned", Lang::Zh => "赚得" },
        K::SvgSpent => match lang { Lang::En => "spent", Lang::Zh => "花掉" },
        K::Today => match lang { Lang::En => "Today", Lang::Zh => "今天" },
        K::FieldName => match lang { Lang::En => "Name", Lang::Zh => "名称" },
        K::KidNamePlaceholder => match lang { Lang::En => "e.g. Alex", Lang::Zh => "例如：小宝" },
        K::ItemNamePlaceholder => match lang { Lang::En => "e.g. Put away toys", Lang::Zh => "例如：自己收拾玩具" },
        K::PickIcon => match lang { Lang::En => "Pick an icon", Lang::Zh => "选一个图标" },
        K::PickAvatar => match lang { Lang::En => "Pick an avatar", Lang::Zh => "选一个头像" },
        K::Save => match lang { Lang::En => "Save", Lang::Zh => "保存" },
        K::ErrBadFormat => match lang { Lang::En => "invalid file format", Lang::Zh => "文件格式不正确" },
        K::ErrBadKids => match lang { Lang::En => "kid data is incomplete", Lang::Zh => "宝贝数据不完整" },
        K::ErrBadRecords => match lang { Lang::En => "record data is incomplete", Lang::Zh => "记录数据不完整" },
    }
}

/* ---------- 带插值的文案 ---------- */

/// 今日打卡进度行（前缀，+earned 由调用方单独渲染成彩色 span）
pub fn today_progress_pre(lang: Lang, done: usize, total: usize) -> String {
    match lang {
        Lang::En => format!("{done}/{total} done · "),
        Lang::Zh => format!("已完成 {done}/{total} 项，今天赚到 "),
    }
}

/// 今日打卡进度行（后缀，跟在 +earned 后面）
pub fn today_progress_suffix(lang: Lang) -> &'static str {
    match lang {
        Lang::En => " credits earned today",
        Lang::Zh => " 积分",
    }
}

pub fn import_success_msg(lang: Lang, n: i64) -> String {
    match lang {
        Lang::En => format!("Import successful! Restored all settings and records for {n} kid(s) (existing data was replaced)."),
        Lang::Zh => format!("导入成功！已恢复 {n} 个宝贝的全部配置和记录（原有数据已被替换）"),
    }
}

pub fn import_error_msg(lang: Lang, detail: &str) -> String {
    match lang {
        Lang::En => format!("Import failed: {detail}. Please make sure this is a JSON backup exported by this app."),
        Lang::Zh => format!("导入失败：{detail}，请确认是本应用导出的 JSON 备份文件"),
    }
}

pub fn delete_kid_title(lang: Lang, name: &str) -> String {
    match lang {
        Lang::En => format!("Delete \"{name}\"?"),
        Lang::Zh => format!("删除「{name}」？"),
    }
}

pub fn redeem_title(lang: Lang, name: &str) -> String {
    match lang {
        Lang::En => format!("Redeem \"{name}\"?"),
        Lang::Zh => format!("兑换「{name}」？"),
    }
}

/// 兑换确认正文（前缀，cost 由调用方单独渲染成彩色 span）
pub fn redeem_body_pre(lang: Lang) -> &'static str {
    match lang {
        Lang::En => "This costs ",
        Lang::Zh => "将消耗 ",
    }
}

/// 兑换确认正文（剩余部分，跟在 cost 后面）
pub fn redeem_body_rest(lang: Lang, remaining: i64) -> String {
    match lang {
        Lang::En => format!(" credits. You'll have {remaining} credits left."),
        Lang::Zh => format!(" 积分，兑换后剩余 {remaining} 积分。"),
    }
}

pub fn wish_cost(lang: Lang, cost: i64) -> String {
    match lang {
        Lang::En => format!("Costs {cost} credits"),
        Lang::Zh => format!("需要 {cost} 积分"),
    }
}

pub fn wish_gap(lang: Lang, gap: i64) -> String {
    match lang {
        Lang::En => format!("{gap} to go"),
        Lang::Zh => format!("还差 {gap}"),
    }
}

/// 日历格子里的打卡项数（"3项" / "3"）
pub fn checks_label(lang: Lang, n: i64) -> String {
    match lang {
        Lang::En => format!("{n}"),
        Lang::Zh => format!("{n}项"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 所有 K 变体（新增 key 时必须同步加入此表）
    const ALL_KEYS: &[K] = &[
        K::AppTitle, K::ToggleDark, K::Export, K::ExportTitle, K::Import, K::ImportTitle,
        K::EditKid, K::DeleteKid, K::AddKid, K::HeroCreditsSuffix, K::TotalEarned,
        K::TotalSpent, K::HeroToday, K::EmptyWelcome, K::TabCheckin, K::TabWishes,
        K::TabCalendar, K::TabCharts, K::TabHistory, K::Cancel, K::ConfirmDelete,
        K::DeleteKidBody, K::TodayTitle, K::NewTask, K::CreditsUnit, K::Edit, K::Delete,
        K::EditTask, K::AddTask, K::TaskValueLabel, K::WishesTitle, K::WishesSubtitle,
        K::NewWish, K::Redeem, K::NotYet, K::ConfirmRedeem, K::EditWish, K::AddWish,
        K::WishValueLabel, K::CalendarTitle, K::PrevMonth, K::NextMonth, K::AllDone,
        K::LegendIntensity, K::EarnedWord, K::SpentWord, K::DetailsSuffix, K::NoRecords,
        K::HistoryTitle, K::HistoryEmpty, K::ChartsCompareTitle, K::ChartsCompareSubtitle,
        K::ChartsWeekTitle, K::ChartsWeekSubtitle, K::SvgCurrentCredits, K::SvgEarned,
        K::SvgSpent, K::Today, K::FieldName, K::KidNamePlaceholder, K::ItemNamePlaceholder,
        K::PickIcon, K::PickAvatar, K::Save, K::ErrBadFormat, K::ErrBadKids, K::ErrBadRecords,
    ];

    #[test]
    fn every_key_has_two_nonempty_translations() {
        for k in ALL_KEYS {
            let en = t(Lang::En, *k);
            let zh = t(Lang::Zh, *k);
            assert!(!en.is_empty(), "{k:?} en empty");
            assert!(!zh.is_empty(), "{k:?} zh empty");
            assert_ne!(en, zh, "{k:?} en == zh");
        }
    }

    #[test]
    fn lang_roundtrip_and_toggle() {
        assert_eq!(Lang::from_str("zh"), Lang::Zh);
        assert_eq!(Lang::from_str("en"), Lang::En);
        assert_eq!(Lang::from_str("anything"), Lang::En);
        assert_eq!(Lang::En.toggle(), Lang::Zh);
        assert_eq!(Lang::Zh.toggle(), Lang::En);
    }

    #[test]
    fn interpolation_fns_format() {
        assert_eq!(today_progress_pre(Lang::En, 1, 6), "1/6 done · ");
        assert_eq!(today_progress_pre(Lang::Zh, 1, 6), "已完成 1/6 项，今天赚到 ");
        assert!(import_success_msg(Lang::En, 2).contains("2 kid(s)"));
        assert!(import_error_msg(Lang::Zh, "x").starts_with("导入失败：x"));
        assert_eq!(delete_kid_title(Lang::En, "Alex"), "Delete \"Alex\"?");
        assert_eq!(redeem_title(Lang::Zh, "冰淇淋"), "兑换「冰淇淋」？");
        assert_eq!(wish_cost(Lang::En, 10), "Costs 10 credits");
        assert_eq!(wish_gap(Lang::Zh, 8), "还差 8");
        assert_eq!(checks_label(Lang::En, 3), "3");
        assert_eq!(checks_label(Lang::Zh, 3), "3项");
        assert!(redeem_body_rest(Lang::En, 90).contains("90 credits left"));
    }
}
