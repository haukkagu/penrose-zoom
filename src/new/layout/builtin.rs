//! Layout functions
//!
//! Each of the following is a layout function that can be passed to Layout::new.
//! No checks are carried out to ensure that clients are tiled correctly (i.e. that
//! they are non-overlapping) so when adding additional layout functions you are
//! free to tile them however you wish. Xmonad for example has a 'circle' layout
//! that deliberately overlaps clients under the main window.
use crate::new::{client::Client, data_types::Region, workspace::ResizeAction, xconnection::Xid};

/// A no-op floating layout that simply satisfies the type required for Layout
pub fn floating(_: &[&Client], _: Option<Xid>, _: &Region, _: u32, _: f32) -> Vec<ResizeAction> {
    vec![]
}

// ignore paramas and return pairs of window ID and index in the client vec
#[cfg(test)]
pub(crate) fn mock_layout(
    clients: &[&Client],
    _: Option<Xid>,
    region: &Region,
    _: u32,
    _: f32,
) -> Vec<ResizeAction> {
    clients
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let (x, y, w, h) = region.values();
            let _k = i as u32;
            (c.id(), Some(Region::new(x + _k, y + _k, w - _k, h - _k)))
        })
        .collect()
}

/// A simple layout that places the main region on the left and tiles remaining
/// windows in a single column to the right.
pub fn side_stack(
    clients: &[&Client],
    _: Option<Xid>,
    monitor_region: &Region,
    max_main: u32,
    ratio: f32,
) -> Vec<ResizeAction> {
    let n = clients.len() as u32;

    if n <= max_main || max_main == 0 {
        return monitor_region
            .as_rows(n)
            .iter()
            .zip(clients)
            .map(|(r, c)| (c.id(), Some(*r)))
            .collect();
    }

    let split = ((monitor_region.w as f32) * ratio) as u32;
    let (main, stack) = monitor_region.split_at_width(split).unwrap();

    main.as_rows(max_main)
        .into_iter()
        .chain(stack.as_rows(n.saturating_sub(max_main)))
        .zip(clients)
        .map(|(r, c)| (c.id(), Some(r)))
        .collect()
}

/// A simple layout that places the main region at the top of the screen and tiles
/// remaining windows in a single row underneath.
pub fn bottom_stack(
    clients: &[&Client],
    _: Option<Xid>,
    monitor_region: &Region,
    max_main: u32,
    ratio: f32,
) -> Vec<ResizeAction> {
    let n = clients.len() as u32;

    if n <= max_main || max_main == 0 {
        return monitor_region
            .as_columns(n)
            .iter()
            .zip(clients)
            .map(|(r, c)| (c.id(), Some(*r)))
            .collect();
    }

    let split = ((monitor_region.h as f32) * ratio) as u32;
    let (main, stack) = monitor_region.split_at_height(split).unwrap();

    main.as_columns(max_main)
        .into_iter()
        .chain(stack.as_columns(n.saturating_sub(max_main)))
        .zip(clients)
        .map(|(r, c)| (c.id(), Some(r)))
        .collect()
}

/// A simple monolve layout that places uses the maximum available space for the focused client and
/// unmaps all other windows.
pub fn monocle(
    clients: &[&Client],
    focused: Option<Xid>,
    monitor_region: &Region,
    _: u32,
    _: f32,
) -> Vec<ResizeAction> {
    if let Some(fid) = focused {
        let (mx, my, mw, mh) = monitor_region.values();
        clients
            .iter()
            .map(|c| {
                let cid = c.id();
                if cid == fid {
                    (cid, Some(Region::new(mx, my, mw, mh)))
                } else {
                    (cid, None)
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}
