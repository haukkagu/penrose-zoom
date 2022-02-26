//! The main window manager
use crate::new::{state::State, xconnection::XConn};
use std::ops::Deref;

/// Penrose itself
#[derive(Debug)]
pub struct WindowManager<X: XConn> {
    /// The XConn implementation being used to communicate with the X server
    pub conn: X,
    state: State,
}

impl<X: XConn> Deref for WindowManager<X> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
