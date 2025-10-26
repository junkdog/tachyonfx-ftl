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

    // Return minimal data for UI (just id, title, category, description)
    let ui_data: Vec<_> = examples
        .iter()
        .map(|ex| {
            serde_json::json!({
                "id": ex.id,
                "title": ex.title,
                "description": ex.description,
                "category": ex.category
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

#[wasm_bindgen]
pub fn get_completions(source: &str, line: usize, column: usize) -> String {
    use tachyonfx::dsl::CompletionEngine;

    log_info(format!("source: {}, line: {}, column: {}", source, line, column));

    // Convert line/column to cursor index (byte offset)
    // Ace editor uses 0-indexed line/column, so convert to 1-indexed for our function
    let cursor_index = line_column_to_offset(source, line + 1, column + 1);

    // Get completions from the completion engine
    let engine = CompletionEngine::new();

    let echoed_source = engine.echo_source(source, cursor_index);
    log_info(format!("echoed source: '{}'", echoed_source));

    let completions = engine.completions(source, cursor_index);

    // Convert to Ace editor format and then JSON
    // Ace expects: { value: string, score: number, meta: string }
    let json_completions: Vec<_> = completions
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            serde_json::json!({
                "value": c.label,
                "score": 1000 - idx, // Higher score for earlier matches (already sorted by relevance)
                "meta": c.detail.as_str(),
            })
        })
        .collect();



    // Log as JSON object for better browser console inspection
    let js_value = serde_wasm_bindgen::to_value(&json_completions)
        .unwrap_or_else(|_| JsValue::from_str("[]"));
    web_sys::console::log_2(&JsValue::from_str("completions:"), &js_value);

    // Return as JSON string for the caller
    serde_json::to_string(&json_completions).unwrap_or_else(|_| "[]".to_string())
}

fn log_info(msg: impl Into<String>) {
    let msg = msg.into();
    web_sys::console::info_1(&JsValue::from_str(&msg));
}

/// Converts 1-indexed line/column to byte offset in source string
fn line_column_to_offset(source: &str, line: usize, column: usize) -> u32 {
    let mut current_line = 1;
    let mut current_column = 1;

    for (byte_offset, ch) in source.char_indices() {
        if current_line == line && current_column == column {
            return byte_offset as u32;
        }

        if ch == '\n' {
            current_line += 1;
            current_column = 1;
        } else {
            current_column += 1;
        }
    }

    // If we reach the end, return the source length
    source.len() as u32
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
