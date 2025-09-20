mod event_handler;
mod examples;
mod interop;
mod multi_backend;

use crate::event_handler::{convert_key_event, EventHandler};
use crate::interop::init_global_state;
use crate::multi_backend::MultiBackend;
use console_error_panic_hook::set_once as set_panic_hook;
use eyre::Result;
use ratzilla::WebRenderer;
use tfxed_core::{App, AppEvent, Dispatcher};

fn main() -> Result<()> {
    set_panic_hook();
    let events = EventHandler::new();
    let sender = events.sender();

    // globally set the sender for the JS interop functions
    init_global_state(sender.clone());

    let terminal = terminal()?;
    terminal.on_key_event(move |e| {
        if !e.alt && !e.ctrl {
            sender.dispatch(AppEvent::KeyPress(convert_key_event(e)));
        }
    });

    let mut app = App::new(events.sender());

    terminal.draw_web(move |f| {
        events.receive_events(|event| {
            app.apply_event(event);
        });

        app.update_time();
        app.render_ui(f);
    });

    Ok(())
}

fn terminal() -> Result<MultiBackend> {
    MultiBackend::new("content")
}
