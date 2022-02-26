//! Metadata around X clients and manipulating them
use crate::new::xconnection::{Atom, Prop, WmHints, WmNormalHints, XClientProperties, Xid};

mod clients;
pub use clients::Clients;

/**
 * Meta-data around a client window that we are handling.
 *
 * Primarily state flags and information used when determining which clients
 * to show for a given monitor and how they are tiled.
 */
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Client {
    pub(crate) id: Xid,
    pub(crate) wm_name: String,
    pub(crate) wm_class: Vec<String>, // should always be two elements but that's not enforced?
    pub(crate) wm_type: Vec<String>,  // Can't use Atom as it could be something arbitrary
    pub(crate) wm_protocols: Vec<String>, // Can't use Atom as it could be something arbitrary
    pub(crate) wm_hints: Option<WmHints>,
    pub(crate) wm_normal_hints: Option<WmNormalHints>,
    // state flags
    pub(crate) accepts_focus: bool,
    pub(crate) floating: bool,
    pub(crate) fullscreen: bool,
    pub(crate) mapped: bool,
    pub(crate) urgent: bool,
    pub(crate) wm_managed: bool,
}

impl Client {
    /// Only for use in test functions
    #[allow(dead_code)]
    pub(crate) fn stub(id: Xid) -> Self {
        Self {
            id,
            wm_name: "stub".into(),
            wm_class: vec!["stub".into()],
            wm_type: vec!["stub".into()],
            wm_protocols: vec!["stub".into()],
            wm_hints: None,
            wm_normal_hints: None,
            floating: false,
            accepts_focus: true,
            fullscreen: false,
            mapped: false,
            urgent: false,
            wm_managed: true,
        }
    }

    /// The X window ID of this client
    pub fn id(&self) -> Xid {
        self.id
    }

    /// The WM_CLASS property of this client
    pub fn wm_class(&self) -> &str {
        &self.wm_class[0]
    }

    /// The WM_NAME property of this client
    pub fn wm_name(&self) -> &str {
        &self.wm_name
    }

    /// Whether or not this client is currently fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Set the floating state of this client
    pub fn set_floating(&mut self, floating: bool) {
        self.floating = floating
    }

    /// Mark this client as not being managed by the WindowManager directly
    pub fn externally_managed(&mut self) {
        self.wm_managed = false;
    }

    /// Mark this client as being managed by the WindowManager directly
    pub fn internally_managed(&mut self) {
        self.wm_managed = true;
    }
}

/// Track a new client window
///
/// This uses the provided [`XClientProperties`] to query state from the X server about the
/// client and cache that for later use. If any of the requests fail then we set defaults
/// rather than erroring as we always need to be able to track clients when they are mapped.
pub(crate) fn new_from_x_state<X>(conn: &X, id: Xid, floating_classes: &[&str]) -> Client
where
    X: XClientProperties,
{
    let floating = conn.client_should_float(id, floating_classes);
    let accepts_focus = conn.client_accepts_focus(id);
    let wm_name = conn.client_name(id).unwrap_or("unknown".into());

    macro_rules! prop {
        ($atom:expr; $default:expr; $prop:tt) => {
            match conn.get_prop(id, $atom.as_ref()) {
                Ok(Prop::$prop(val)) => val,
                _ => $default,
            }
        };

        ($atom:expr; $prop:tt) => {
            match conn.get_prop(id, $atom.as_ref()) {
                Ok(Prop::$prop(val)) => Some(val),
                _ => None,
            }
        };
    }

    let wm_class = prop!(Atom::WmClass; vec!["unknown".into()]; UTF8String);
    let wm_protocols = prop!(Atom::WmProtocols; vec![]; Atom);
    let wm_type = prop!(Atom::WmClass; vec![Atom::NetWindowTypeNormal.as_ref().to_string()]; Atom);
    let wm_hints = prop!(Atom::WmHints; WmHints);
    let wm_normal_hints = prop!(Atom::WmNormalHints; WmNormalHints);

    Client {
        id,
        wm_name,
        wm_class,
        wm_type,
        wm_protocols,
        wm_hints,
        wm_normal_hints,
        floating,
        accepts_focus,
        fullscreen: false,
        mapped: false,
        urgent: false,
        wm_managed: true,
    }
}
