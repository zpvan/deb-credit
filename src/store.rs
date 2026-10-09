use crate::i18n::Lang;
use crate::models::*;

pub fn default_tasks(lang: Lang) -> &'static [(&'static str, i64, &'static str)] {
    match lang {
        Lang::En => &[
            ("Wake up on time", 2, "⏰"),
            ("Jump rope 5 min", 3, "🤸"),
            ("Sing an English song", 3, "🎵"),
            ("Go to bed on time", 2, "🌙"),
        ],
        Lang::Zh => &[
            ("按时起床", 2, "⏰"),
            ("跳绳 5 分钟", 3, "🤸"),
            ("学唱英文歌", 3, "🎵"),
            ("准时睡觉", 2, "🌙"),
        ],
    }
}

pub fn default_wishes(lang: Lang) -> &'static [(&'static str, i64, &'static str)] {
    match lang {
        Lang::En => &[
            ("Ice cream", 10, "🍦"),
            ("Snacks", 15, "🍿"),
            ("Watch a cartoon", 8, "📺"),
            ("Summer trip", 200, "✈️"),
        ],
        Lang::Zh => &[
            ("吃冰淇淋", 10, "🍦"),
            ("买零食", 15, "🍿"),
            ("看一集动画片", 8, "📺"),
            ("暑假去旅游", 200, "✈️"),
        ],
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Store {
    pub seq: i64,
    #[serde(default)]
    pub kids: Vec<Kid>,
    #[serde(default)]
    pub tasks: Vec<EarnTask>,
    #[serde(default)]
    pub wishes: Vec<WishItem>,
    #[serde(default)]
    pub txns: Vec<Txn>,
}

impl Store {
    pub fn empty() -> Self {
        Self::default()
    }

    fn next_id(&mut self) -> i64 {
        self.seq += 1;
        self.seq
    }

    /* ---------- kids ---------- */

    pub fn create_kid(&mut self, name: &str, avatar: &str, lang: Lang) -> i64 {
        let id = self.next_id();
        for (n, c, i) in default_tasks(lang).iter().copied() {
            let tid = self.next_id();
            self.tasks.push(EarnTask {
                id: tid,
                kid_id: id,
                name: n.into(),
                credit: c,
                icon: i.into(),
            });
        }
        for (n, c, i) in default_wishes(lang).iter().copied() {
            let wid = self.next_id();
            self.wishes.push(WishItem {
                id: wid,
                kid_id: id,
                name: n.into(),
                cost: c,
                icon: i.into(),
            });
        }
        self.kids.push(Kid {
            id,
            name: name.into(),
            avatar: avatar.into(),
        });
        id
    }

    pub fn update_kid(&mut self, id: i64, name: &str, avatar: &str) {
        if let Some(k) = self.kids.iter_mut().find(|k| k.id == id) {
            k.name = name.into();
            k.avatar = avatar.into();
        }
    }

    pub fn delete_kid(&mut self, id: i64) {
        self.kids.retain(|k| k.id != id);
        self.tasks.retain(|t| t.kid_id != id);
        self.wishes.retain(|w| w.kid_id != id);
        self.txns.retain(|t| t.kid_id != id);
    }

    /* ---------- check-in / redeem ---------- */

    pub fn toggle_task(&mut self, kid_id: i64, task_id: i64, today: &str, now: &str) {
        let check_key = format!("chk-{task_id}-{today}");
        let existing = self
            .txns
            .iter()
            .find(|t| t.kid_id == kid_id && t.check_key.as_deref() == Some(check_key.as_str()))
            .map(|t| t.id);
        if let Some(id) = existing {
            self.txns.retain(|t| t.id != id);
            return;
        }
        let Some(task) = self
            .tasks
            .iter()
            .find(|t| t.id == task_id && t.kid_id == kid_id)
            .cloned()
        else {
            return;
        };
        let id = self.next_id();
        self.txns.push(Txn {
            id,
            kid_id,
            kind: TxnKind::Earn,
            name: task.name,
            amount: task.credit,
            date: today.into(),
            check_key: Some(check_key),
            created_at: now.into(),
        });
    }

    pub fn redeem_wish(&mut self, kid_id: i64, wish_id: i64, today: &str, now: &str) -> bool {
        let Some(wish) = self
            .wishes
            .iter()
            .find(|w| w.id == wish_id && w.kid_id == kid_id)
            .cloned()
        else {
            return false;
        };
        if self.balance(kid_id) < wish.cost {
            return false;
        }
        let id = self.next_id();
        self.txns.push(Txn {
            id,
            kid_id,
            kind: TxnKind::Spend,
            name: wish.name,
            amount: wish.cost,
            date: today.into(),
            check_key: None,
            created_at: now.into(),
        });
        true
    }

    /* ---------- derived ---------- */

    pub fn balance(&self, kid_id: i64) -> i64 {
        self.txns
            .iter()
            .filter(|t| t.kid_id == kid_id)
            .map(|t| match t.kind {
                TxnKind::Earn => t.amount,
                TxnKind::Spend => -t.amount,
            })
            .sum()
    }

    pub fn total_earned(&self, kid_id: i64) -> i64 {
        self.txns
            .iter()
            .filter(|t| t.kid_id == kid_id && t.kind == TxnKind::Earn)
            .map(|t| t.amount)
            .sum()
    }

    pub fn total_spent(&self, kid_id: i64) -> i64 {
        self.txns
            .iter()
            .filter(|t| t.kid_id == kid_id && t.kind == TxnKind::Spend)
            .map(|t| t.amount)
            .sum()
    }

    pub fn is_task_done(&self, kid_id: i64, task_id: i64, today: &str) -> bool {
        let key = format!("chk-{task_id}-{today}");
        self.txns
            .iter()
            .any(|t| t.kid_id == kid_id && t.check_key.as_deref() == Some(key.as_str()))
    }

    /* ---------- tasks ---------- */

    pub fn add_task(&mut self, kid_id: i64, name: &str, credit: i64, icon: &str) -> i64 {
        let id = self.next_id();
        self.tasks.push(EarnTask {
            id,
            kid_id,
            name: name.into(),
            credit,
            icon: icon.into(),
        });
        id
    }

    pub fn update_task(&mut self, id: i64, name: &str, credit: i64, icon: &str) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.name = name.into();
            t.credit = credit;
            t.icon = icon.into();
        }
    }

    pub fn remove_task(&mut self, id: i64) {
        self.tasks.retain(|t| t.id != id);
    }

    /* ---------- wishes ---------- */

    pub fn add_wish(&mut self, kid_id: i64, name: &str, cost: i64, icon: &str) -> i64 {
        let id = self.next_id();
        self.wishes.push(WishItem {
            id,
            kid_id,
            name: name.into(),
            cost,
            icon: icon.into(),
        });
        id
    }

    pub fn update_wish(&mut self, id: i64, name: &str, cost: i64, icon: &str) {
        if let Some(w) = self.wishes.iter_mut().find(|w| w.id == id) {
            w.name = name.into();
            w.cost = cost;
            w.icon = icon.into();
        }
    }

    pub fn remove_wish(&mut self, id: i64) {
        self.wishes.retain(|w| w.id != id);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackupError {
    BadFormat,
    BadKids,
    BadRecords,
}

impl BackupError {
    pub fn message(&self) -> &'static str {
        match self {
            BackupError::BadFormat => "文件格式不正确",
            BackupError::BadKids => "宝贝数据不完整",
            BackupError::BadRecords => "记录数据不完整",
        }
    }
}

/// 校验并解析备份 JSON；与原版 parseBackup 行为一致，包括 seq 重建。
pub fn parse_backup(raw: &str) -> Result<Store, BackupError> {
    let v: serde_json::Value = serde_json::from_str(raw).map_err(|_| BackupError::BadFormat)?;
    let get_arr = |key: &str| v.get(key).filter(|x| x.is_array()).cloned();
    let (Some(kids_v), Some(tasks_v), Some(wishes_v), Some(txns_v)) = (
        get_arr("kids"),
        get_arr("tasks"),
        get_arr("wishes"),
        get_arr("txns"),
    ) else {
        return Err(BackupError::BadFormat);
    };
    let kids: Vec<Kid> = serde_json::from_value(kids_v).map_err(|_| BackupError::BadKids)?;
    let tasks: Vec<EarnTask> =
        serde_json::from_value(tasks_v).map_err(|_| BackupError::BadRecords)?;
    let wishes: Vec<WishItem> =
        serde_json::from_value(wishes_v).map_err(|_| BackupError::BadRecords)?;
    let txns: Vec<Txn> = serde_json::from_value(txns_v).map_err(|_| BackupError::BadRecords)?;
    let max_id = kids
        .iter()
        .map(|k| k.id)
        .chain(tasks.iter().map(|t| t.id))
        .chain(wishes.iter().map(|w| w.id))
        .chain(txns.iter().map(|t| t.id))
        .max()
        .unwrap_or(0);
    let seq = match v.get("seq").and_then(|s| s.as_i64()) {
        Some(s) if s >= max_id => s,
        _ => max_id,
    };
    Ok(Store {
        seq,
        kids,
        tasks,
        wishes,
        txns,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /* ---------- Task 3: kids CRUD ---------- */

    #[test]
    fn create_kid_assigns_id_and_defaults() {
        let mut s = Store::empty();
        let id = s.create_kid("小宝", "🧒", Lang::Zh);
        assert_eq!(s.kids.len(), 1);
        assert_eq!(s.kids[0].name, "小宝");
        assert_eq!(s.tasks.len(), 4);
        assert!(s.tasks.iter().all(|t| t.kid_id == id));
        assert_eq!(s.wishes.len(), 4);
        assert!(s.wishes.iter().all(|w| w.kid_id == id));
        assert_eq!(s.tasks[0].name, "按时起床");
        assert_eq!(s.tasks[0].credit, 2);
        assert_eq!(s.tasks[0].icon, "⏰");
        assert_eq!(s.wishes[3].name, "暑假去旅游");
        assert_eq!(s.wishes[3].cost, 200);
    }

    #[test]
    fn create_kid_en_seeds_english_defaults() {
        let mut s = Store::empty();
        s.create_kid("Alex", "🧒", Lang::En);
        assert_eq!(s.tasks[0].name, "Wake up on time");
        assert_eq!(s.wishes[3].name, "Summer trip");
    }

    #[test]
    fn ids_are_unique_across_entities() {
        let mut s = Store::empty();
        s.create_kid("甲", "🧒", Lang::Zh);
        s.create_kid("乙", "👧", Lang::Zh);
        let mut ids: Vec<i64> = s
            .kids
            .iter()
            .map(|k| k.id)
            .chain(s.tasks.iter().map(|t| t.id))
            .chain(s.wishes.iter().map(|w| w.id))
            .collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 2 + 8 + 8);
    }

    #[test]
    fn update_kid_changes_name_and_avatar() {
        let mut s = Store::empty();
        let id = s.create_kid("小宝", "🧒", Lang::Zh);
        s.update_kid(id, "大宝", "🐯");
        assert_eq!(s.kids[0].name, "大宝");
        assert_eq!(s.kids[0].avatar, "🐯");
    }

    #[test]
    fn delete_kid_cascades() {
        let mut s = Store::empty();
        let a = s.create_kid("甲", "🧒", Lang::Zh);
        let b = s.create_kid("乙", "👧", Lang::Zh);
        s.txns.push(Txn {
            id: 900,
            kid_id: a,
            kind: TxnKind::Earn,
            name: "x".into(),
            amount: 1,
            date: "2026-10-08".into(),
            check_key: None,
            created_at: "2026-10-08T00:00:00.000Z".into(),
        });
        s.delete_kid(a);
        assert!(s.kids.iter().all(|k| k.id != a));
        assert!(s.tasks.iter().all(|t| t.kid_id == b));
        assert!(s.wishes.iter().all(|w| w.kid_id == b));
        assert!(s.txns.iter().all(|t| t.kid_id == b));
    }

    /* ---------- Task 4: check-in / redeem ---------- */

    fn store_with_kid() -> (Store, i64) {
        let mut s = Store::empty();
        let id = s.create_kid("小宝", "🧒", Lang::Zh);
        (s, id)
    }

    #[test]
    fn toggle_task_earns_and_untoggles() {
        let (mut s, kid) = store_with_kid();
        let task_id = s.tasks[0].id;
        let credit = s.tasks[0].credit;
        s.toggle_task(kid, task_id, "2026-10-08", "2026-10-08T00:00:00.000Z");
        assert_eq!(s.balance(kid), credit);
        assert!(s.is_task_done(kid, task_id, "2026-10-08"));
        assert_eq!(s.txns.len(), 1);
        assert_eq!(s.txns[0].kind, TxnKind::Earn);
        assert_eq!(
            s.txns[0].check_key.as_deref(),
            Some(format!("chk-{task_id}-2026-10-08").as_str())
        );
        s.toggle_task(kid, task_id, "2026-10-08", "2026-10-08T00:00:01.000Z");
        assert_eq!(s.balance(kid), 0);
        assert!(s.txns.is_empty());
    }

    #[test]
    fn toggle_task_is_per_day() {
        let (mut s, kid) = store_with_kid();
        let task_id = s.tasks[0].id;
        s.toggle_task(kid, task_id, "2026-10-07", "2026-10-07T00:00:00.000Z");
        s.toggle_task(kid, task_id, "2026-10-08", "2026-10-08T00:00:00.000Z");
        assert!(s.is_task_done(kid, task_id, "2026-10-07"));
        assert!(s.is_task_done(kid, task_id, "2026-10-08"));
        assert_eq!(s.txns.len(), 2);
    }

    #[test]
    fn toggle_task_ignores_wrong_kid() {
        let (mut s, kid) = store_with_kid();
        let task_id = s.tasks[0].id;
        s.toggle_task(kid + 999, task_id, "2026-10-08", "2026-10-08T00:00:00.000Z");
        assert!(s.txns.is_empty());
    }

    #[test]
    fn redeem_wish_checks_balance() {
        let (mut s, kid) = store_with_kid();
        let wish_id = s.wishes[0].id; // 吃冰淇淋 10 分
        assert!(!s.redeem_wish(kid, wish_id, "2026-10-08", "2026-10-08T00:00:00.000Z"));
        assert!(s.txns.is_empty());
        let task_id = s.tasks[0].id;
        for d in 1..=5 {
            let day = format!("2026-10-0{d}");
            s.toggle_task(kid, task_id, &day, "2026-10-08T00:00:00.000Z");
        }
        assert_eq!(s.balance(kid), 10);
        assert!(s.redeem_wish(kid, wish_id, "2026-10-08", "2026-10-08T00:00:00.000Z"));
        assert_eq!(s.balance(kid), 0);
        assert_eq!(s.txns.last().unwrap().kind, TxnKind::Spend);
        assert_eq!(s.txns.last().unwrap().name, "吃冰淇淋");
        assert!(s.txns.last().unwrap().check_key.is_none());
    }

    #[test]
    fn totals_split_earn_and_spend() {
        let (mut s, kid) = store_with_kid();
        let task_id = s.tasks[0].id;
        s.toggle_task(kid, task_id, "2026-10-08", "2026-10-08T00:00:00.000Z");
        assert_eq!(s.total_earned(kid), 2);
        assert_eq!(s.total_spent(kid), 0);
    }

    /* ---------- Task 5: parse_backup ---------- */

    const VALID_BACKUP: &str = r#"{
        "app": "kid-credit-tracker", "version": 2, "exportedAt": "2026-10-08T00:00:00.000Z",
        "seq": 5,
        "kids": [{"id": 1, "name": "小宝", "avatar": "🧒", "createdAt": "ignored-extra-field"}],
        "tasks": [{"id": 2, "kidId": 1, "name": "按时起床", "credit": 2, "icon": "⏰"}],
        "wishes": [{"id": 3, "kidId": 1, "name": "吃冰淇淋", "cost": 10, "icon": "🍦"}],
        "txns": [{"id": 4, "kidId": 1, "type": "earn", "name": "按时起床", "amount": 2,
                  "date": "2026-10-08", "checkKey": "chk-2-2026-10-08",
                  "createdAt": "2026-10-08T00:00:00.000Z"}]
    }"#;

    #[test]
    fn parse_valid_backup() {
        let s = parse_backup(VALID_BACKUP).unwrap();
        assert_eq!(s.kids.len(), 1);
        assert_eq!(s.txns[0].kind, TxnKind::Earn);
        assert_eq!(s.seq, 5);
    }

    #[test]
    fn parse_backup_rebuilds_seq_from_max_id() {
        let raw = VALID_BACKUP.replace(r#""seq": 5"#, r#""seq": 1"#);
        let s = parse_backup(&raw).unwrap();
        assert_eq!(s.seq, 4);
    }

    #[test]
    fn parse_backup_rejects_non_json() {
        assert_eq!(parse_backup("not json").unwrap_err(), BackupError::BadFormat);
    }

    #[test]
    fn parse_backup_rejects_missing_arrays() {
        let e = parse_backup(r#"{"kids": [], "tasks": []}"#).unwrap_err();
        assert_eq!(e, BackupError::BadFormat);
        assert_eq!(e.message(), "文件格式不正确");
    }

    #[test]
    fn parse_backup_rejects_bad_kid() {
        let raw = VALID_BACKUP.replace(r#""name": "小宝""#, r#""name": 123"#);
        let e = parse_backup(&raw).unwrap_err();
        assert_eq!(e, BackupError::BadKids);
        assert_eq!(e.message(), "宝贝数据不完整");
    }

    #[test]
    fn parse_backup_rejects_bad_txn() {
        let raw = VALID_BACKUP.replace(r#""amount": 2"#, r#""amount": "two""#);
        let e = parse_backup(&raw).unwrap_err();
        assert_eq!(e, BackupError::BadRecords);
        assert_eq!(e.message(), "记录数据不完整");
    }
}
