use crate::store::Store;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub const STORAGE_KEY: &str = "kid-credit-tracker-v2";

/// localStorage 读取；解析失败或不存在返回 None（对应原版 try/catch fallthrough）
pub fn load_store() -> Option<Store> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let raw = storage.get_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

/// 全量写入 localStorage；失败静默忽略（同原版）
pub fn save_store(store: &Store) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(json) = serde_json::to_string(store) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }
}

#[derive(serde::Serialize)]
struct BackupFile<'a> {
    app: &'a str,
    version: u32,
    #[serde(rename = "exportedAt")]
    exported_at: String,
    #[serde(flatten)]
    store: &'a Store,
}

/// 导出为 JSON 文件并触发浏览器下载
pub fn export_store(store: &Store) {
    let payload = BackupFile {
        app: "kid-credit-tracker",
        version: 2,
        exported_at: crate::date::now_iso(),
        store,
    };
    let Ok(json) = serde_json::to_string_pretty(&payload) else {
        return;
    };
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(&json));
    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type("application/json");
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &opts) else {
        return;
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else {
        return;
    };
    let Ok(el) = document.create_element("a") else { return };
    let a: web_sys::HtmlAnchorElement = el.unchecked_into();
    a.set_href(&url);
    a.set_download(&format!(
        "宝贝积分站-备份-{}.json",
        crate::date::today_str()
    ));
    a.click();
    let _ = web_sys::Url::revoke_object_url(&url);
}

/// 读取 <input type=file> 选中的文件内容
pub async fn read_file_text(file: &web_sys::File) -> Result<String, String> {
    let text = JsFuture::from(file.text())
        .await
        .map_err(|_| "文件无法解析".to_string())?;
    text.as_string().ok_or_else(|| "文件无法解析".to_string())
}
