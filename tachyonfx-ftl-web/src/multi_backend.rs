use eyre::{eyre, Result};
use ratatui::{Frame, Terminal as RatTerminal};
use ratzilla::{
    backend::{canvas::CanvasBackendOptions, webgl2::WebGl2BackendOptions},
    error::Error,
    event::{KeyEvent, MouseEvent},
    CanvasBackend, FontAtlasConfig, WebGl2Backend, WebRenderer,
};

#[derive(Debug)]
pub enum BackendType {
    WebGl2,
    Canvas,
}

fn get_backend_type() -> BackendType {
    let window = web_sys::window().expect("no global window");
    let location = window.location();
    let search = location.search().unwrap_or_default();

    if search.contains("backend=canvas") {
        BackendType::Canvas
    } else {
        BackendType::WebGl2
    }
}

pub enum MultiBackend {
    WebGl2(RatTerminal<WebGl2Backend>),
    Canvas(RatTerminal<CanvasBackend>),
}

impl MultiBackend {
    pub fn new(grid_id: &str) -> Result<Self> {
        match get_backend_type() {
            BackendType::WebGl2 => {
                let options = WebGl2BackendOptions::new()
                    .enable_mouse_selection_with_mode(Default::default())
                    // .font_atlas_config(FontAtlasConfig::dynamic(&["Hack"], 15.0))
                    .grid_id(grid_id);

                let backend = WebGl2Backend::new_with_options(options).map_err(|e| eyre!("{e}"))?;
                let terminal = RatTerminal::new(backend)?;
                Ok(MultiBackend::WebGl2(terminal))
            }
            BackendType::Canvas => {
                let backend =
                    CanvasBackend::new_with_options(CanvasBackendOptions::new().grid_id(grid_id))
                        .map_err(|e| eyre!("{e}"))?;
                let terminal = RatTerminal::new(backend)?;
                Ok(MultiBackend::Canvas(terminal))
            }
        }
    }
}

impl WebRenderer for MultiBackend {
    fn draw_web<F>(self, render_callback: F)
    where
        F: FnMut(&mut Frame) + 'static,
    {
        match self {
            MultiBackend::WebGl2(terminal) => terminal.draw_web(render_callback),
            MultiBackend::Canvas(terminal) => terminal.draw_web(render_callback),
        }
    }

    fn on_key_event<F>(&mut self, callback: F) -> Result<(), Error>
    where
        F: FnMut(KeyEvent) + 'static,
    {
        match self {
            MultiBackend::WebGl2(terminal) => terminal.on_key_event(callback),
            MultiBackend::Canvas(terminal) => terminal.on_key_event(callback),
        }
    }

    fn on_mouse_event<F>(&mut self, callback: F) -> Result<(), Error>
    where
        F: FnMut(MouseEvent) + 'static,
    {
        match self {
            MultiBackend::WebGl2(terminal) => terminal.on_mouse_event(callback),
            MultiBackend::Canvas(terminal) => terminal.on_mouse_event(callback),
        }
    }
}
