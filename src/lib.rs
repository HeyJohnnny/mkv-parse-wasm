use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use matroska::Matroska;
use std::io::Cursor;
use serde::Serialize;

#[derive(Serialize)]
struct MkvInfo {
    title: Option<String>,
    duration: Option<f64>,
    tracks: Vec<String>,
}

#[wasm_bindgen]
pub async fn analyze_mkv(blob: web_sys::Blob) -> Result<JsValue, JsValue> {
    let array_buffer = JsFuture::from(blob.array_buffer()).await?;
    let data = js_sys::Uint8Array::new(&array_buffer).to_vec();

    // 使用 Cursor 来模拟文件读取
    let cursor = Cursor::new(data);
    let mkv = Matroska::open(cursor).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // 创建简化的信息结构
    let info = MkvInfo {
        title: mkv.info.title,
        duration: mkv.info.duration.map(|d| d.as_secs_f64()),
        tracks: mkv.tracks.iter().map(|t| format!("Track {:?}", t.tracktype)).collect(),
    };
    
    // 使用 serde-wasm-bindgen 来序列化
    Ok(serde_wasm_bindgen::to_value(&info)?)
}