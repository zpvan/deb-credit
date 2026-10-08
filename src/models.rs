use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Kid {
    pub id: i64,
    pub name: String,
    pub avatar: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EarnTask {
    pub id: i64,
    #[serde(rename = "kidId")]
    pub kid_id: i64,
    pub name: String,
    pub credit: i64,
    pub icon: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WishItem {
    pub id: i64,
    #[serde(rename = "kidId")]
    pub kid_id: i64,
    pub name: String,
    pub cost: i64,
    pub icon: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TxnKind {
    Earn,
    Spend,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Txn {
    pub id: i64,
    #[serde(rename = "kidId")]
    pub kid_id: i64,
    #[serde(rename = "type")]
    pub kind: TxnKind,
    pub name: String,
    pub amount: i64,
    pub date: String, // YYYY-MM-DD
    #[serde(rename = "checkKey")]
    pub check_key: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kid_serde_roundtrip() {
        let k = Kid { id: 1, name: "小宝".into(), avatar: "🧒".into() };
        let json = serde_json::to_string(&k).unwrap();
        assert_eq!(json, r#"{"id":1,"name":"小宝","avatar":"🧒"}"#);
        let back: Kid = serde_json::from_str(&json).unwrap();
        assert_eq!(back, k);
    }

    #[test]
    fn txn_uses_camel_case_keys() {
        let t = Txn {
            id: 7, kid_id: 3, kind: TxnKind::Earn,
            name: "按时起床".into(), amount: 2,
            date: "2026-10-08".into(),
            check_key: Some("chk-1-2026-10-08".into()),
            created_at: "2026-10-08T01:23:45.000Z".into(),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains(r#""kidId":3"#));
        assert!(json.contains(r#""type":"earn""#));
        assert!(json.contains(r#""checkKey":"chk-1-2026-10-08""#));
        assert!(json.contains(r#""createdAt""#));
        let back: Txn = serde_json::from_str(&json).unwrap();
        assert_eq!(back, t);
    }

    #[test]
    fn txn_kind_spend_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&TxnKind::Spend).unwrap(), r#""spend""#);
    }
}
