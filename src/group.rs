//! Communication between widgets, it's based on Groups by IDs (an usize) that
//! WidgetInternal will store and dispatched by a Container
use std::collections::VecDeque;

pub type GroupID = usize;

#[derive(PartialEq)]
pub enum GroupEvent {
    Signal(GroupID),
    Layout,
}

static mut EVENT_QUEUE: Option<VecDeque<GroupEvent>> = None;

/// Push a GroupEvent to the Event Queue
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

/// Pop a GroupEvent from the Event Queue
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
    Single(GroupID),
    Slice(&'static [GroupID]),
}

impl Group {
    pub fn is_any(&self) -> bool {
        match *self {
            Group::Any => true,
            _ => false,
        }
    }

    pub fn check_id(&self, id: GroupID) -> bool {
        match *self {
            Group::Single(single_id) => single_id == id,
            Group::Slice(slice_id) => slice_id.contains(&id),
            Group::Any => true,
        }
    }
}