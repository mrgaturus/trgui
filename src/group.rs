//! Communication between widgets
use std::collections::VecDeque;

pub type GroupID = usize;

#[derive(PartialEq)]
pub enum GroupEvent {
    Signal(GroupID),
    Layout(GroupID),
    LayoutAll,
}

static mut EVENT_QUEUE: Option<VecDeque<GroupEvent>> = None;

/// Push a Group ID to the Group Queue
pub fn push_event(event: GroupEvent) {
    unsafe {
        if let Some(ref mut queue) = EVENT_QUEUE {
            if !queue.contains(&event) {
                queue.push_back(event);
            }
        } else {
            let mut queue: VecDeque<GroupEvent> = VecDeque::with_capacity(4);
            queue.push_back(event);

            EVENT_QUEUE = Some(queue);
        }
    }
}

/// Pop a Signal from the Signal queue
pub fn next_event() -> Option<GroupEvent> {
    unsafe {
        if let Some(ref mut queue) = EVENT_QUEUE {
            queue.pop_front()
        } else {
            None
        }
    }
}

/// A Group with options to store ID/s
pub enum Group {
    Any,
    // Signal types works only for signal handling
    SignalSingle(GroupID),
    SignalSlice(&'static [GroupID]),
    // Layout types work as Any for signal handling
    LayoutSingle(GroupID),
    LayoutSlice(&'static [GroupID]),
    // Dual types work for both (signals and layouts)
    DualSingle(GroupID),
    DualSlice(&'static [GroupID]),
}

impl Group {
    /// Check if the group is member of the requested id
    #[inline]
    pub fn signal_check(&self, id: GroupID) -> bool {
        match *self {
            Group::Any | Group::LayoutSingle(_) | Group::LayoutSlice(_) => true,
            Group::SignalSingle(group) | Group::DualSingle(group) => group == id,
            Group::SignalSlice(groups) | Group::DualSlice(groups) => groups.contains(&id),
        }
    }

    #[inline]
    pub fn layout_check(&self, id: GroupID) -> bool {
        match *self {
            Group::Any => true,
            Group::LayoutSingle(group) | Group::DualSingle(group) => group == id,
            Group::LayoutSlice(groups) | Group::DualSlice(groups) => groups.contains(&id),
            _ => false,
        }
    }

    #[inline]
    pub fn is_any(&self) -> bool {
        match *self {
            Group::Any => true,
            _ => false,
        }
    }
}
