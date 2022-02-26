//! Rewrite and simplification of internal APIs

pub mod bindings;
pub mod client;
pub mod config;
pub mod data_types;
pub mod event;
pub mod hooks;
pub mod layout;
pub mod manager;
pub mod ring;
pub mod screen;
pub mod state;
pub mod workspace;
pub mod xconnection;

use xconnection::Xid;

/// Top level penrose Result type
pub type Result<T> = std::result::Result<T, Error>;

/// A function that can be registered to handle errors that occur during [WindowManager] operation
pub type ErrorHandler = Box<dyn FnMut(Error)>;

/// Enum to store the various ways that operations can fail in Penrose
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Something went wrong using the [draw] module.
    ///
    /// See [DrawError][crate::draw::DrawError] for variants.
    #[error(transparent)]
    Draw(#[from] crate::draw::DrawError),

    /// Something was inconsistant when attempting to re-create a serialised [WindowManager]
    #[error("unable to rehydrate from serialized state: {0}")]
    HydrationState(String),

    /// Something was inconsistant when attempting to re-create a serialised [WindowManager]
    #[error("the following serialized client IDs were not known to the X server: {0:?}")]
    MissingClientIds(Vec<Xid>),

    /// A conversion to utf-8 failed
    #[error("UTF-8 error")]
    NonUtf8Prop(#[from] std::string::FromUtf8Error),

    #[doc(hidden)]
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),

    /// An [IO Error][std::io::Error] was encountered
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Wm(Normal)Hints received from the X server were invalid
    #[error("Invalid window hints property: {0}")]
    InvalidHints(String),

    /// No elements match the given predicate
    #[error("No elements match the given predicate")]
    NoMatchingElement,

    /// Attempting to construct a penrose data type from an int failed.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// A generic error type for use in user code when needing to construct
    /// a simple [Error].
    #[error("Unhandled error: {0}")]
    Raw(String),

    /// An attempt to spawn an external process failed
    #[error("unable to get stdout handle for child process: {0}")]
    SpawnProc(String),

    /// Parsing an [Atom][core::xconnection::Atom] from a str failed.
    ///
    /// This happens when the atom name being requested is not a known atom.
    #[error(transparent)]
    Strum(#[from] strum::ParseError),

    /// An attempt was made to reference a client that is not known to penrose
    #[error("{0} is not a known client")]
    UnknownClient(Xid),

    /// A user specified key binding contained an invalid modifier key
    #[error("Unknown modifier key: {0}")]
    UnknownModifier(String),

    /// Something went wrong using the [xcb] module.
    ///
    /// See [XcbError][crate::xcb::XcbError] for variants.
    #[cfg(feature = "xcb")]
    #[error(transparent)]
    Xcb(#[from] crate::xcb::XcbError),

    /// Something went wrong using the [x11rb] module.
    ///
    /// See [X11rbError][crate::x11rb::X11rbError] for variants.
    #[cfg(feature = "x11rb")]
    #[error(transparent)]
    X11rb(#[from] crate::x11rb::X11rbError),

    /// Something went wrong when communicating with the X server
    #[error(transparent)]
    X(#[from] crate::new::xconnection::XError),
}
