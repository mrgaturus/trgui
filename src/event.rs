use std::collections::VecDeque;

pub type EventID = u32;
static mut EVENT_QUEUE: Option<VecDeque<EventID>> = None;

pub fn queue_event_id(id: EventID) {
    unsafe {
        if let Some(ref mut queue) = EVENT_QUEUE {
            if !queue.contains(&id) {
                queue.push_back(id);
            }
        } else {
            let mut queue: VecDeque<EventID> = VecDeque::with_capacity(4);
            queue.push_back(id);

            EVENT_QUEUE = Some(queue);
        }
    }
}

pub fn next_event() -> Option<EventID> {
    unsafe {
        if let Some(ref mut queue) = EVENT_QUEUE {
            queue.pop_front()
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum EventType {
    Any,
    ID(EventID),
    None,
}
