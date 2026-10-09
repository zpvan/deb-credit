use crate::date::{date_label_md, format_date, last_7_days};
use crate::models::{Txn, TxnKind, WishItem};
use chrono::{Datelike, NaiveDate};

pub const COMPARE_COLORS: [&str; 6] =
    ["#984800", "#2f6b3c", "#0b57d0", "#a63c66", "#6d5a00", "#4c4aa8"];

const FONT: &str = "'Baloo 2','PingFang SC','Microsoft YaHei',sans-serif";

pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// 仅右端圆角的矩形 path（recharts radius=[0,14,14,0] 对应）
fn rounded_right_rect(x: f64, y: f64, w: f64, h: f64, r: f64) -> String {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    let wr = w - r;
    let h2 = h - 2.0 * r;
    format!(
        "M{x:.1} {y:.1} h{wr:.1} a{r:.1} {r:.1} 0 0 1 {r:.1} {r:.1} v{h2:.1} a{r:.1} {r:.1} 0 0 1 -{r:.1} {r:.1} h-{wr:.1} z"
    )
}

/// 仅顶部圆角的矩形 path（recharts radius=[8,8,0,0] 对应）
fn rounded_top_rect(x: f64, y: f64, w: f64, h: f64, r: f64) -> String {
    let r = r.min(w / 2.0).min(h).max(0.0);
    let hs = h - r;
    let w2 = w - 2.0 * r;
    format!(
        "M{x:.1} {yb:.1} v-{hs:.1} a{r:.1} {r:.1} 0 0 1 {r:.1} -{r:.1} h{w2:.1} a{r:.1} {r:.1} 0 0 1 {r:.1} {r:.1} v{hs:.1} z",
        yb = y + h
    )
}

/// 积分 vs 心愿 横向条形图
pub fn compare_chart_svg(balance: i64, wishes: &[WishItem]) -> String {
    struct Row {
        label: String,
        value: i64,
        color: &'static str,
    }
    let mut rows = vec![Row {
        label: "💰 当前积分".into(),
        value: balance,
        color: "var(--primary)",
    }];
    for (i, w) in wishes.iter().enumerate() {
        rows.push(Row {
            label: format!("{} {}", w.icon, w.name),
            value: w.cost,
            color: COMPARE_COLORS[(i + 1) % COMPARE_COLORS.len()],
        });
    }
    let n = rows.len();
    let height = (n as f64 * 52.0).max(220.0);
    let max_v = rows.iter().map(|r| r.value).max().unwrap_or(1).max(1) as f64;
    const LEFT: f64 = 140.0;
    let bar_max_w = 600.0 - LEFT - 48.0;

    let mut s = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 {height}" width="100%" font-family="{FONT}">"##
    );
    // 纵向虚线网格 + X 刻度（对应 CartesianGrid horizontal=false + XAxis）
    for t in 0..=4 {
        let x = LEFT + bar_max_w * t as f64 / 4.0;
        let v = (max_v * t as f64 / 4.0).round() as i64;
        s.push_str(&format!(
            r##"<line x1="{x:.1}" y1="4" x2="{x:.1}" y2="{y2:.1}" stroke="var(--outline-variant)" stroke-dasharray="4 4"/>"##,
            y2 = height - 20.0
        ));
        s.push_str(&format!(
            r##"<text x="{x:.1}" y="{y:.1}" text-anchor="middle" font-size="12" fill="var(--on-surface-variant)">{v}</text>"##,
            y = height - 4.0
        ));
    }
    for (i, row) in rows.iter().enumerate() {
        let y = 8.0 + i as f64 * 52.0;
        let w = row.value as f64 / max_v * bar_max_w;
        let label = escape_xml(&row.label);
        s.push_str(&format!(
            r##"<text x="132" y="{y:.1}" text-anchor="end" font-size="12" font-weight="700" fill="var(--on-surface-variant)">{label}</text>"##,
            y = y + 17.0
        ));
        if row.value > 0 {
            s.push_str(&format!(
                r##"<path d="{d}" fill="{color}"><title>{label}: {v} 积分</title></path>"##,
                d = rounded_right_rect(LEFT, y, w.max(2.0), 26.0, 13.0),
                color = row.color,
                v = row.value
            ));
        }
        s.push_str(&format!(
            r##"<text x="{x:.1}" y="{y:.1}" font-size="14" font-weight="800" fill="var(--on-surface)">{v}</text>"##,
            x = LEFT + w + 8.0,
            y = y + 18.0,
            v = row.value
        ));
    }
    s.push_str("</svg>");
    s
}

/// 最近 7 天收支 分组柱状图
pub fn week_chart_svg(txns: &[Txn], today: NaiveDate) -> String {
    let days = last_7_days(today);
    let mut data: Vec<(String, String, i64, i64)> = Vec::new(); // (date, label, earn, spend)
    for (i, d) in days.iter().enumerate() {
        let date = format_date(d.year(), d.month(), d.day());
        let label = if i == 6 { "今天".into() } else { date_label_md(*d) };
        let (mut earn, mut spend) = (0i64, 0i64);
        for t in txns.iter().filter(|t| t.date == date) {
            match t.kind {
                TxnKind::Earn => earn += t.amount,
                TxnKind::Spend => spend += t.amount,
            }
        }
        data.push((date, label, earn, spend));
    }
    let max_v = data
        .iter()
        .flat_map(|d| [d.2, d.3])
        .max()
        .unwrap_or(0)
        .max(1);
    // Y 轴整数刻度：<=6 时步长 1，否则 4 等分向上取整
    let (tick_max, tick_step) = if max_v <= 6 {
        (max_v, 1)
    } else {
        let step = (max_v + 3) / 4;
        (step * 4, step)
    };
    const W: f64 = 600.0;
    const H: f64 = 256.0;
    const LEFT: f64 = 40.0;
    const TOP: f64 = 8.0;
    const BOTTOM: f64 = 24.0;
    let plot_h = H - TOP - BOTTOM;
    let group_w = (W - LEFT - 8.0) / 7.0;

    let mut s = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 256" width="100%" font-family="{FONT}">"##
    );
    // 横向虚线网格 + Y 刻度
    let mut t = 0;
    while t <= tick_max {
        let y = TOP + plot_h - (t as f64 / tick_max as f64) * plot_h;
        s.push_str(&format!(
            r##"<line x1="{LEFT}" y1="{y:.1}" x2="{x2:.1}" y2="{y:.1}" stroke="var(--outline-variant)" stroke-dasharray="4 4"/>"##,
            x2 = W - 8.0
        ));
        s.push_str(&format!(
            r##"<text x="{x:.1}" y="{yt:.1}" text-anchor="end" font-size="12" fill="var(--on-surface-variant)">{t}</text>"##,
            x = LEFT - 8.0,
            yt = y + 4.0
        ));
        t += tick_step;
    }
    for (i, (date, label, earn, spend)) in data.iter().enumerate() {
        let gx = LEFT + group_w * i as f64 + (group_w - 40.0) / 2.0;
        for (j, v, color, kind_label) in [
            (0.0, *earn, "var(--earn)", "赚得"),
            (1.0, *spend, "var(--primary)", "花掉"),
        ] {
            if v > 0 {
                let h = v as f64 / tick_max as f64 * plot_h;
                s.push_str(&format!(
                    r##"<path d="{d}" fill="{color}"><title>{date} {kind_label} {v} 积分</title></path>"##,
                    d = rounded_top_rect(gx + j * 22.0, TOP + plot_h - h, 18.0, h, 8.0)
                ));
            }
        }
        s.push_str(&format!(
            r##"<text x="{x:.1}" y="{y:.1}" text-anchor="middle" font-size="12" font-weight="700" fill="var(--on-surface-variant)">{label}</text>"##,
            x = gx + 20.0,
            y = H - 6.0
        ));
    }
    s.push_str("</svg>");
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Txn, TxnKind, WishItem};

    #[test]
    fn escape_xml_escapes_specials() {
        assert_eq!(escape_xml("<b>&\"</b>"), "&lt;b&gt;&amp;&quot;&lt;/b&gt;");
    }

    #[test]
    fn compare_chart_has_one_row_per_wish_plus_balance() {
        let wishes = vec![
            WishItem { id: 1, kid_id: 1, name: "冰淇淋".into(), cost: 10, icon: "🍦".into() },
            WishItem { id: 2, kid_id: 1, name: "动画<片>".into(), cost: 8, icon: "📺".into() },
        ];
        let svg = compare_chart_svg(5, &wishes);
        // 3 行：余额 + 2 心愿 → 3 个名称 label
        assert_eq!(svg.matches("text-anchor=\"end\"").count(), 3);
        assert!(svg.contains("💰 当前积分"));
        // 心愿名中的 < > 必须被转义
        assert!(svg.contains("动画&lt;片&gt;"));
        assert!(!svg.contains("动画<片>"));
        // 高度 = max(220, 3*52)
        assert!(svg.contains("viewBox=\"0 0 600 220\""));
        // 余额条用主题色
        assert!(svg.contains("fill=\"var(--primary)\""));
    }

    #[test]
    fn week_chart_renders_7_days_and_bars() {
        let today = chrono::NaiveDate::from_ymd_opt(2026, 10, 8).unwrap();
        let txns = vec![
            Txn { id: 1, kid_id: 1, kind: TxnKind::Earn, name: "a".into(), amount: 3,
                  date: "2026-10-08".into(), check_key: None, created_at: "x".into() },
            Txn { id: 2, kid_id: 1, kind: TxnKind::Spend, name: "b".into(), amount: 2,
                  date: "2026-10-07".into(), check_key: None, created_at: "x".into() },
        ];
        let svg = week_chart_svg(&txns, today);
        assert!(svg.contains(">今天</text>"));
        assert!(svg.contains(">10/2</text>"));
        // 仅 2 天有交易 → 恰好 2 根柱子
        assert_eq!(svg.matches("<path").count(), 2);
        assert!(svg.contains("fill=\"var(--earn)\""));
        assert!(svg.contains("fill=\"var(--primary)\""));
    }

    #[test]
    fn week_chart_no_txns_still_renders_grid() {
        let today = chrono::NaiveDate::from_ymd_opt(2026, 10, 8).unwrap();
        let svg = week_chart_svg(&[], today);
        assert!(svg.contains("stroke-dasharray=\"4 4\""));
        assert_eq!(svg.matches("<path").count(), 0);
    }
}
