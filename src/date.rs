use chrono::{Datelike, Duration, Local, NaiveDate, Utc};

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

/// 对应原版 dateLabel："10月8日 星期四"
pub fn header_date_label(d: NaiveDate) -> String {
    let week = WEEKDAYS_CN[d.weekday().num_days_from_monday() as usize];
    format!("{}月{}日 星期{}", d.month(), d.day(), week)
}

/// "10/2"
pub fn date_label_md(d: NaiveDate) -> String {
    format!("{}/{}", d.month(), d.day())
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
    fn header_label_has_weekday() {
        // 2026-10-08 是周四
        assert_eq!(header_date_label(d("2026-10-08")), "10月8日 星期四");
    }

    #[test]
    fn date_label_md_formats() {
        assert_eq!(date_label_md(d("2026-10-02")), "10/2");
    }
}
