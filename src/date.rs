use chrono::{Datelike, Duration, Local, NaiveDate, Utc};
use crate::i18n::Lang;

/// 对应原版 todayStr()：本地时区 YYYY-MM-DD
pub fn today_str() -> String {
    let d = Local::now().date_naive();
    format_date(d.year(), d.month(), d.day())
}

pub fn format_date(year: i32, month: u32, day: u32) -> String {
    format!("{year}-{month:02}-{day:02}")
}

/// 对应原版 new Date().toISOString()（UTC ISO，毫秒精度，带 Z）
pub fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

/// 返回 [today-6, ..., today]，升序
pub fn last_7_days(today: NaiveDate) -> Vec<NaiveDate> {
    (0..7).rev().map(|i| today - Duration::days(i)).collect()
}

/// 某月日历格子，周一开头；空格为 None。month 为 1-12。
pub fn month_cells(year: i32, month: u32) -> Vec<Option<u32>> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).expect("valid year/month");
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let days_in_month = NaiveDate::from_ymd_opt(ny, nm, 1)
        .and_then(|d| d.pred_opt())
        .expect("valid next month")
        .day();
    let offset = first.weekday().num_days_from_monday() as usize;
    let mut cells = vec![None; offset];
    cells.extend((1..=days_in_month).map(Some));
    while cells.len() % 7 != 0 {
        cells.push(None);
    }
    cells
}

pub const WEEKDAYS_CN: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];

/// "10/2"
pub fn date_label_md(d: NaiveDate) -> String {
    format!("{}/{}", d.month(), d.day())
}

pub const WEEKDAYS_ZH: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];
pub const WEEKDAYS_EN: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
pub const WEEKDAYS_EN_LONG: [&str; 7] =
    ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
pub const MONTHS_EN: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

/// 日历表头的星期标签（周一开头）
pub fn weekday_labels(lang: Lang) -> [&'static str; 7] {
    match lang {
        Lang::En => WEEKDAYS_EN,
        Lang::Zh => WEEKDAYS_ZH,
    }
}

/// 顶栏日期行：中「2026-10-09 · 10月9日 星期五」/ 英「Friday, October 9, 2026」
pub fn header_date_line(lang: Lang, today: NaiveDate) -> String {
    let wd = today.weekday().num_days_from_monday() as usize;
    match lang {
        Lang::En => format!(
            "{}, {} {}, {}",
            WEEKDAYS_EN_LONG[wd],
            MONTHS_EN[today.month() as usize - 1],
            today.day(),
            today.year()
        ),
        Lang::Zh => format!(
            "{} · {}月{}日 星期{}",
            format_date(today.year(), today.month(), today.day()),
            today.month(),
            today.day(),
            WEEKDAYS_ZH[wd]
        ),
    }
}

/// 日历月份标题：中「2026年10月」/ 英「October 2026」
pub fn month_title(lang: Lang, year: i32, month: u32) -> String {
    match lang {
        Lang::En => format!("{} {}", MONTHS_EN[month as usize - 1], year),
        Lang::Zh => format!("{year}年{month}月"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn format_date_pads() {
        assert_eq!(format_date(2026, 10, 8), "2026-10-08");
        assert_eq!(format_date(2026, 1, 2), "2026-01-02");
    }

    #[test]
    fn last_7_days_ends_at_today() {
        let days = last_7_days(d("2026-10-08"));
        assert_eq!(days.len(), 7);
        assert_eq!(days[0], d("2026-10-02"));
        assert_eq!(days[6], d("2026-10-08"));
    }

    #[test]
    fn month_cells_october_2026() {
        // 2026-10-01 是周四 → 周一起头偏移 3；10 月 31 天；格子总数补齐到 7 的倍数
        let cells = month_cells(2026, 10);
        assert_eq!(cells.len(), 35);
        assert_eq!(cells[0], None);
        assert_eq!(cells[2], None);
        assert_eq!(cells[3], Some(1));
        assert_eq!(cells[33], Some(31));
        assert_eq!(cells[34], None);
    }

    #[test]
    fn month_cells_december_rolls_year() {
        let cells = month_cells(2026, 12);
        assert_eq!(cells.iter().flatten().count(), 31);
    }

    #[test]
    fn date_label_md_formats() {
        assert_eq!(date_label_md(d("2026-10-02")), "10/2");
    }

    #[test]
    fn header_date_line_zh() {
        // 2026-10-08 是周四
        assert_eq!(header_date_line(Lang::Zh, d("2026-10-08")), "2026-10-08 · 10月8日 星期四");
    }

    #[test]
    fn header_date_line_en() {
        assert_eq!(header_date_line(Lang::En, d("2026-10-08")), "Thursday, October 8, 2026");
    }

    #[test]
    fn month_title_both_langs() {
        assert_eq!(month_title(Lang::Zh, 2026, 10), "2026年10月");
        assert_eq!(month_title(Lang::En, 2026, 10), "October 2026");
    }
}
