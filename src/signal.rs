use std::collections::VecDeque;

pub type SignalID = u32;
static mut SIGNAL_QUEUE: Option<VecDeque<SignalID>> = None;

pub fn emit_id(id: SignalID) {
    unsafe {
        if let Some(ref mut queue) = SIGNAL_QUEUE {
            if !queue.contains(&id) {
                queue.push_back(id);
            }
        } else {
            let mut queue: VecDeque<SignalID> = VecDeque::with_capacity(4);
            queue.push_back(id);

            SIGNAL_QUEUE = Some(queue);
        }
    }
}

pub fn next_signal() -> Option<SignalID> {
    unsafe {
        if let Some(ref mut queue) = SIGNAL_QUEUE {
            queue.pop_front()
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum SignalType {
    Any,
    ID(SignalID),
    None,
}
