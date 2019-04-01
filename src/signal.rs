//! Communication between widgets

use std::collections::VecDeque;
use std::mem;

pub type SignalID = usize;
static mut SIGNAL_QUEUE: Option<VecDeque<SignalID>> = None;

/// Push a Signal ID to the Signal Queue
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

/// Pop a Signal ID from the Signal Queue
pub fn next_signal() -> Option<SignalID> {
    unsafe {
        if let Some(ref mut queue) = SIGNAL_QUEUE {
            queue.pop_front()
        } else {
            None
        }
    }
}

/// A Signal with options to store Signal IDs
#[derive(Copy, Clone)]
pub enum Signal {
    Any,
    Single(SignalID),
    DisabledSingle(SignalID),
    Slice(&'static [SignalID]),
    DisabledSlice(&'static [SignalID]),
    None,
}

impl Signal {
    /// Enables the Signal, enabled signals are used by Containers for search widgets and 
    /// call the function of these widgets
    pub fn enable(&mut self) {
        mem::replace(
            self,
            match *self {
                Signal::None => Signal::Any,
                Signal::DisabledSingle(signal) => Signal::Single(signal),
                Signal::DisabledSlice(signal_slice) => Signal::Slice(signal_slice),
                _ => *self,
            },
        );
    }

    /// Disables the Signal, hides the signal from Containers
    pub fn disable(&mut self) {
        mem::replace(
            self,
            match *self {
                Signal::Any => Signal::None,
                Signal::Single(signal) => Signal::DisabledSingle(signal),
                Signal::Slice(signal_slice) => Signal::DisabledSlice(signal_slice),
                _ => *self,
            },
        );
    }

    /// Check if the signal is enabled
    pub fn enabled(&self) -> bool {
        match self {
            Signal::Any | Signal::Single(_) | Signal::Slice(_) => true,
            Signal::None | Signal::DisabledSingle(_) | Signal::DisabledSlice(_) => false,
        }
    }

    /// Check if the signal is a member of an ID
    #[inline]
    pub fn check(&self, id: SignalID) -> bool {
        match self {
            Signal::Any => true,
            Signal::Single(signal) => *signal == id,
            Signal::Slice(signal_slice) => signal_slice.contains(&id),
            _ => false,
        }
    }
}
