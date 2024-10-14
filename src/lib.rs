//! A simple state machine lib
//!
//! # simple usage:
//!
//! ```
//! use machine::{Machine, State};
//!
//! #[derive(Debug, PartialEq)]
//! enum SwitchState {
//!   On,
//!   Off,
//! }
//!
//! #[derive(Debug, PartialEq)]
//! enum SwitchEvent {
//!   Toggle,
//! }
//!
//! impl State<SwitchEvent> for SwitchState {
//!   fn apply(&self, event: SwitchEvent) -> Self {
//!     match self {
//!       Self::On => Self::Off,
//!       Self::Off => Self::On,
//!     }
//!   }
//! }
//!
//! let mut machine = Machine::new(SwitchState::Off);
//!
//! // it starts at given state value
//! assert_eq!(machine.state, SwitchState::Off);
//!
//! // change state by dispatching an event
//! machine = machine.dispatch(SwitchEvent::Toggle);
//! assert_eq!(machine.state, SwitchState::On);
//! machine = machine.dispatch(SwitchEvent::Toggle);
//! assert_eq!(machine.state, SwitchState::Off);
//! ```

use std::marker::PhantomData;

pub struct Machine<S, E>
where
    S: State<E>,
{
    pub state: S,
    // FIXME: remove this phantom data when possible
    _event: PhantomData<E>,
}

impl<S, E> Machine<S, E>
where
    S: State<E>,
{
    pub fn new(initial_state: S) -> Self {
        Self {
            state: initial_state,
            _event: PhantomData,
        }
    }

    pub fn dispatch(self, event: E) -> Self {
        Self {
            state: self.state.apply(event),
            ..self
        }
    }
}

pub trait State<Event> {
    fn apply(&self, event: Event) -> Self;
}
