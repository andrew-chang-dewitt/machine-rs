//! A simple state machine lib
//!
//! # slightly more complex
//!
//! let's model a turnstile instead
//!
//! ```
//! use machine::{Machine, State};
//!
//! #[derive(Debug, PartialEq)]
//! enum TurnstileState {
//!   Locked,
//!   Unlocked,
//! }
//!
//! #[derive(Debug, PartialEq)]
//! enum TurnstileEvent {
//!   PaymentReceived,
//!   PersonEntering,
//! }
//!
//! impl State<TurnstileEvent> for TurnstileState {
//!   fn apply(&self, event: TurnstileEvent) -> Self {
//!     match self {
//!       Self::Locked => match event {
//!         TurnstileEvent::PaymentReceived => Self::Unlocked,
//!         _ => panic!("Payment required for entry.")
//!       }
//!       Self::Unlocked => match event {
//!         TurnstileEvent::PersonEntering => Self::Locked,
//!         _ => panic!("Payment already received, unable to accept payment at this time.")
//!       }
//!     }
//!   }
//! }
//!
//! let mut machine = Machine::new(TurnstileState::Locked);
//! // it starts at given state value
//! assert_eq!(machine.state, TurnstileState::Locked);
//! // change state by dispatching an event
//! machine = machine.dispatch(TurnstileEvent::PaymentReceived);
//! assert_eq!(machine.state, TurnstileState::Unlocked);
//! machine = machine.dispatch(TurnstileEvent::PersonEntering);
//! assert_eq!(machine.state, TurnstileState::Locked);
//! ```
//!
//! but now, if we try to dispatch an event when the machine is in an invalid state for that
//! event, the machine panics. this isn't ideal...
//!
//! ```should_panic
//! # use machine::{Machine, State};
//!
//! # #[derive(Debug, PartialEq)]
//! # enum TurnstileState {
//! #   Locked,
//! #   Unlocked,
//! # }
//!
//! # #[derive(Debug, PartialEq)]
//! # enum TurnstileEvent {
//! #   PaymentReceived,
//! #   PersonEntering,
//! # }
//!
//! # impl State<TurnstileEvent> for TurnstileState {
//! #   fn apply(&self, event: TurnstileEvent) -> Self {
//! #     match self {
//! #       Self::Locked => match event {
//! #         TurnstileEvent::PaymentReceived => Self::Unlocked,
//! #         _ => panic!("Payment required for entry.")
//! #       }
//! #       Self::Unlocked => match event {
//! #         TurnstileEvent::PersonEntering => Self::Locked,
//! #         _ => panic!("Payment already received, unable to accept payment at this time.")
//! #       }
//! #     }
//! #   }
//! # }
//! let mut machine = Machine::new(TurnstileState::Locked);
//! // try to enter w/out paying
//! machine = machine.dispatch(TurnstileEvent::PersonEntering);
//! // and it panics with "Payment required for entry"
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
