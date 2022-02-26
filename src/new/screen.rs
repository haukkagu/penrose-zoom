use crate::new::{
    data_types::Region, event::EventAction, hooks::HookName, xconnection::XState, Result,
};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default)]
pub struct Screens {
    pub(crate) focused: usize,
    pub(crate) workspace_indices: Vec<usize>,
    inner: Vec<Region>,
}

impl Deref for Screens {
    type Target = Vec<Region>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Screens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Index<usize> for Screens {
    type Output = Region;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Screens {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl Screens {
    pub fn indexed_screen_for_workspace(&self, wix: usize) -> Option<(usize, Region)> {
        self.workspace_indices
            .iter()
            .position(|&i| i == wix)
            .map(|i| (i, self.inner[i]))
    }

    pub fn effective_region(&self, ix: usize, bar_height: u32, top_bar: bool) -> Region {
        let (x, y, w, h) = self.inner[ix].values();
        if top_bar {
            Region::new(x, y + bar_height, w, h - bar_height)
        } else {
            Region::new(x, y, w, h - bar_height)
        }
    }

    pub fn update_known_screens<S>(
        &mut self,
        state: &S,
        n_workspaces: usize,
    ) -> Result<Vec<EventAction>>
    where
        S: XState,
    {
        let detected = state.current_screens()?;
        for r in detected.iter() {
            info!(w = r.w, h = r.h, "screen detected");
        }

        let actions = if self.inner != detected {
            self.inner = detected;

            let n = self.inner.len();
            let m = self.workspace_indices.len();

            if n < m {
                self.workspace_indices.resize(n, 0);
            } else if n > m {
                self.workspace_indices.append(
                    &mut (0..n_workspaces)
                        .filter(|w| !self.workspace_indices.contains(w))
                        .take(m - n)
                        .collect(),
                );
            }

            vec![
                EventAction::LayoutVisible,
                EventAction::RunHook(HookName::ScreenUpdated),
            ]
        } else {
            vec![]
        };

        Ok(actions)
    }
}
