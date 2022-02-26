//! The state required for running Penrose
use crate::new::{
    bindings::{KeyBindings, MouseBindings},
    client::Clients,
    config::Config,
    screen::Screens,
    workspace::Workspaces,
    xconnection::XConn,
    Error, ErrorHandler,
};
use nix::sys::signal::{signal, SigHandler, Signal};
use tracing::trace;

/// The internal state of Penrose
#[derive(Debug, Default)]
pub struct State {
    pub(crate) config: Config,
    pub(crate) clients: Clients,
    pub(crate) screens: Screens,
    pub(crate) workspaces: Workspaces,
}

pub trait Hook {}
pub struct Hooks {
    inner: Vec<Box<dyn Hook>>,
}

/// Run your window manager
pub fn run<X: XConn>(
    conn: X,
    mut state: State,
    hooks: Vec<Box<dyn Hook>>,
    error_handler: ErrorHandler,
    mut key_bindings: KeyBindings<X>,
    mut mouse_bindings: MouseBindings<X>,
) -> Result<(), Error> {
    trace!("Initialising XConn");
    conn.init()?;

    let wss = &state.config.workspaces;

    trace!("Attempting initial screen detection");
    state.screens.update_known_screens(&conn, wss.len())?;

    trace!("Setting EWMH properties");
    conn.set_wm_properties(wss)?;

    trace!("Forcing cursor to first screen");
    conn.warp_cursor(None, &state.screens[0])?;

    // ignore SIGCHILD and allow child / inherited processes to be inherited by pid1
    trace!("registering SIGCHILD signal handler");
    if let Err(e) = unsafe { signal(Signal::SIGCHLD, SigHandler::SigIgn) } {
        panic!("unable to set signal handler: {}", e);
    }

    trace!("grabbing key and mouse bindings");
    conn.grab_keys(&key_bindings, &mouse_bindings)?;

    Ok(())
}
