use crate::new::{workspace::Workspace, xconnection::Xid};

/// The current workspace state of Penrose
#[derive(Debug, Default)]
pub struct Workspaces {
    pub(crate) workspaces: Vec<Workspace>,
    pub(crate) focused_ws: usize,
    pub(crate) prev_ws: usize,
}

impl Workspaces {
    pub(crate) fn focused_client_id(&self) -> Option<Xid> {
        self.workspaces[self.focused_ws].focused_client()
    }
}
