use crate::new::{client::Client, workspace::Workspace, xconnection::Xid};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default)]
pub struct Clients {
    inner: HashMap<Xid, Client>,
}

impl Clients {
    #[inline]
    pub fn clients_for_workspace(&self, ws: &Workspace) -> Vec<&Client> {
        ws.client_ids().iter().flat_map(|id| self.get(id)).collect()
    }

    #[inline]
    pub fn partitioned_clients_for_workspace(
        &self,
        ws: &Workspace,
    ) -> (Vec<&Client>, Vec<&Client>) {
        ws.client_ids()
            .iter()
            .flat_map(|id| self.get(id))
            .partition(|&c| c.floating)
    }
}

impl Deref for Clients {
    type Target = HashMap<Xid, Client>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Clients {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
