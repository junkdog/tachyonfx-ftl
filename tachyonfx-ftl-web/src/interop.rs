use crate::examples::get_examples;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use flate2::{write::DeflateEncoder, Compression};
use std::io::Write;
use std::sync::mpsc::Sender;
use tfxed_core::{AppEvent, AppEvent::CompileDsl, AppEvent::UpdateCanvas, Dispatcher};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_dsl(s: &str) {
    sender().dispatch(CompileDsl(s.into()));
}

#[wasm_bindgen]
pub fn update_canvas(s: &str) {
    sender().dispatch(UpdateCanvas(s.into()));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn dsl_error_callback(error_message: &str);
}

#[wasm_bindgen]
pub fn notify_error(error_message: &str) {
    dsl_error_callback(error_message);
}

#[wasm_bindgen]
pub fn get_example_list() -> String {
    let examples = get_examples();

    // Return minimal data for UI (just id, title, category, description, tags)
    let ui_data: Vec<_> = examples
        .iter()
        .map(|ex| {
            serde_json::json!({
                "id": ex.id,
                "title": ex.title,
                "description": ex.description,
                "category": ex.category,
                "tags": ex.tags
            })
        })
        .collect();

    serde_json::to_string(&ui_data).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn load_example(example_id: &str) -> String {
    let examples = get_examples();

    if let Some(example) = examples.iter().find(|ex| ex.id == example_id) {
        let compressed_code = compress_and_encode(&example.code);
        let compressed_canvas = compress_and_encode(&example.canvas);

        serde_json::json!({
            "code": compressed_code,
            "canvas": compressed_canvas,
            "title": example.title,
            "description": example.description,
            "id": example.id
        })
        .to_string()
    } else {
        serde_json::json!({
            "error": format!("Example '{}' not found", example_id)
        })
        .to_string()
    }
}

// Compress using raw deflate (no headers) to match JavaScript pako.deflate()
fn compress_and_encode(input: &str) -> String {
    let input_bytes = input.as_bytes();

    // Use raw deflate compression (no headers, matches pako.deflate with raw: true)
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(input_bytes)
        .expect("Failed to write to deflate encoder");
    let compressed = encoder
        .finish()
        .expect("Failed to finish deflate compression");

    // Encode with base64url (no padding)
    URL_SAFE_NO_PAD.encode(compressed)
}

fn sender() -> Sender<AppEvent> {
    unsafe {
        match SENDER.as_ref() {
            None => panic!("No sender in global state"),
            Some(s) => s.event_sender.clone(),
        }
    }
}

struct JsSender {
    event_sender: Sender<AppEvent>,
}

pub fn init_global_state(sender: Sender<AppEvent>) {
    unsafe {
        SENDER = Some(JsSender {
            event_sender: sender,
        });
    }
}

static mut SENDER: Option<JsSender> = None;
