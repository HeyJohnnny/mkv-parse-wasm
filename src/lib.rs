use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use matroska::Matroska;
use matroska_demuxer::{MatroskaFile, TrackType, Frame};
use std::io::Cursor;
use serde::Serialize;

#[derive(Serialize)]
struct MkvInfo {
    title: Option<String>,
    duration: Option<f64>,
    tracks: Vec<String>,
    audio_tracks: Vec<AudioTrack>,
}

#[derive(Serialize)]
struct AudioTrack {
    track_id: u64,
    codec: String,
    data: Vec<u8>,
}

#[wasm_bindgen]
pub async fn analyze_mkv(blob: web_sys::Blob) -> Result<JsValue, JsValue> {
    let array_buffer = JsFuture::from(blob.array_buffer()).await?;
    let data = js_sys::Uint8Array::new(&array_buffer).to_vec();

    // 用 matroska 解析元信息
    let cursor = Cursor::new(data.clone());
    let mkv = Matroska::open(cursor).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 用 matroska-demuxer 提取音轨数据
    let cursor = Cursor::new(data);
    let mut mkvfile = MatroskaFile::open(cursor).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let mut audio_tracks = Vec::new();
    let tracks = mkvfile.tracks().to_vec();
    for track in &tracks {
        if track.track_type() == TrackType::Audio {
            let track_id = track.track_number().get();
            let codec = track.codec_id().to_string();
            let mut audio_data = Vec::new();
            let mut frame = Frame::default();
            while mkvfile.next_frame(&mut frame).unwrap_or(false) {
                if frame.track == track_id {
                    audio_data.extend_from_slice(&frame.data);
                }
            }
            audio_tracks.push(AudioTrack {
                track_id,
                codec,
                data: audio_data,
            });
            // 重新定位到文件开头以便下一个音轨提取
            mkvfile.seek(0).ok();
        }
    }

    let info = MkvInfo {
        title: mkv.info.title,
        duration: mkv.info.duration.map(|d| d.as_secs_f64()),
        tracks: mkv.tracks.iter().map(|t| format!("Track {:?}", t.tracktype)).collect(),
        audio_tracks,
    };
    Ok(serde_wasm_bindgen::to_value(&info)?)
}