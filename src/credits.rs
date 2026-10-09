use crate::models::*;
use crate::store::Store;
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct Credits {
    pub store: RwSignal<Store>,
    pub selected_kid_id: RwSignal<Option<i64>>,
    pub kids: Memo<Vec<Kid>>,
    pub kid: Memo<Option<Kid>>,
    pub tasks: Memo<Vec<EarnTask>>,
    pub wishes: Memo<Vec<WishItem>>,
    pub txns: Memo<Vec<Txn>>,
    pub balance: Memo<i64>,
    pub total_earned: Memo<i64>,
    pub total_spent: Memo<i64>,
}

impl Credits {
    /// 在 App 组件 setup 中调用一次并 provide_context。
    pub fn new() -> Self {
        let store = RwSignal::new(crate::persist::load_store().unwrap_or_default());
        let selected_kid_id = RwSignal::new(None::<i64>);

        let kids = Memo::new(move |_| store.with(|s| s.kids.clone()));
        let kid = Memo::new(move |_| {
            let sel = selected_kid_id.get();
            store.with(|s| sel.and_then(|id| s.kids.iter().find(|k| k.id == id).cloned()))
        });
        let tasks = Memo::new(move |_| {
            let sel = selected_kid_id.get();
            store.with(|s| {
                s.tasks
                    .iter()
                    .filter(|t| Some(t.kid_id) == sel)
                    .cloned()
                    .collect()
            })
        });
        let wishes = Memo::new(move |_| {
            let sel = selected_kid_id.get();
            store.with(|s| {
                s.wishes
                    .iter()
                    .filter(|w| Some(w.kid_id) == sel)
                    .cloned()
                    .collect()
            })
        });
        let txns = Memo::new(move |_| {
            let sel = selected_kid_id.get();
            store.with(|s| {
                s.txns
                    .iter()
                    .filter(|t| Some(t.kid_id) == sel)
                    .cloned()
                    .collect()
            })
        });
        let balance = Memo::new(move |_| {
            match selected_kid_id.get() {
                Some(id) => store.with(|s| s.balance(id)),
                None => 0,
            }
        });
        let total_earned = Memo::new(move |_| {
            match selected_kid_id.get() {
                Some(id) => store.with(|s| s.total_earned(id)),
                None => 0,
            }
        });
        let total_spent = Memo::new(move |_| {
            match selected_kid_id.get() {
                Some(id) => store.with(|s| s.total_spent(id)),
                None => 0,
            }
        });

        // auto-select 第一个宝贝（仅在值真正变化时 set，避免 Effect 自我触发死循环）
        Effect::new(move |_| {
            let kids = store.with(|s| s.kids.clone());
            let sel = selected_kid_id.get();
            let new = if kids.is_empty() {
                None
            } else if sel.map_or(true, |id| !kids.iter().any(|k| k.id == id)) {
                Some(kids[0].id)
            } else {
                sel
            };
            if new != sel {
                selected_kid_id.set(new);
            }
        });

        // 变更后自动写入 localStorage
        Effect::new(move |_| {
            let s = store.get();
            crate::persist::save_store(&s);
        });

        Self {
            store,
            selected_kid_id,
            kids,
            kid,
            tasks,
            wishes,
            txns,
            balance,
            total_earned,
            total_spent,
        }
    }

    fn with_kid(&self, f: impl FnOnce(&mut Store, i64)) {
        if let Some(kid_id) = self.selected_kid_id.get_untracked() {
            self.store.update(|s| f(s, kid_id));
        }
    }

    /* ---------- check-in / redeem ---------- */

    pub fn toggle_task(&self, task_id: i64) {
        let today = crate::date::today_str();
        let now = crate::date::now_iso();
        self.with_kid(|s, kid| s.toggle_task(kid, task_id, &today, &now));
    }

    pub fn redeem_wish(&self, wish_id: i64) -> bool {
        let today = crate::date::today_str();
        let now = crate::date::now_iso();
        let mut ok = false;
        self.with_kid(|s, kid| ok = s.redeem_wish(kid, wish_id, &today, &now));
        ok
    }

    pub fn is_task_done(&self, task_id: i64) -> bool {
        let today = crate::date::today_str();
        match self.selected_kid_id.get() {
            Some(kid) => self.store.with(|s| s.is_task_done(kid, task_id, &today)),
            None => false,
        }
    }

    /* ---------- kids ---------- */

    pub fn create_kid(&self, name: &str, avatar: &str) {
        let mut id = 0;
        self.store.update(|s| id = s.create_kid(name, avatar));
        self.selected_kid_id.set(Some(id));
    }

    pub fn update_kid(&self, id: i64, name: &str, avatar: &str) {
        self.store.update(|s| s.update_kid(id, name, avatar));
    }

    pub fn delete_kid(&self, id: i64) {
        self.store.update(|s| s.delete_kid(id));
        self.selected_kid_id.set(None);
    }

    /* ---------- tasks / wishes ---------- */

    pub fn add_task(&self, name: &str, credit: i64, icon: &str) {
        self.with_kid(|s, kid| {
            s.add_task(kid, name, credit, icon);
        });
    }

    pub fn update_task(&self, id: i64, name: &str, credit: i64, icon: &str) {
        self.store.update(|s| s.update_task(id, name, credit, icon));
    }

    pub fn remove_task(&self, id: i64) {
        self.store.update(|s| s.remove_task(id));
    }

    pub fn add_wish(&self, name: &str, cost: i64, icon: &str) {
        self.with_kid(|s, kid| {
            s.add_wish(kid, name, cost, icon);
        });
    }

    pub fn update_wish(&self, id: i64, name: &str, cost: i64, icon: &str) {
        self.store.update(|s| s.update_wish(id, name, cost, icon));
    }

    pub fn remove_wish(&self, id: i64) {
        self.store.update(|s| s.remove_wish(id));
    }

    /* ---------- backup ---------- */

    pub fn import_store(&self, new_store: Store) {
        self.store.set(new_store);
        self.selected_kid_id.set(None);
    }
}
