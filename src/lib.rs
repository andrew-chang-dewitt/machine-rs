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
//!   fn apply(&self, event: TurnstileEvent) -> Result<Self, String> {
//!     match self {
//!       Self::Locked => match event {
//!         TurnstileEvent::PaymentReceived => Ok( Self::Unlocked ),
//!         _ => Err("Payment required for entry.".to_owned())
//!       }
//!       Self::Unlocked => match event {
//!         TurnstileEvent::PersonEntering => Ok( Self::Locked ),
//!         _ => Err("Payment already received, unable to accept payment at this time.".to_owned())
//!       }
//!     }
//!   }
//! }
//!
//! let mut machine = Machine::new(TurnstileState::Locked);
//! // it starts at given state value
//! assert_eq!(machine.state, TurnstileState::Locked);
//!
//! // and now, when we try to dispatch an event when the machine is in an invalid state for that
//! // event, the machine instead returns an error
//!
//! // try to enter w/out paying
//! machine.dispatch(TurnstileEvent::PersonEntering).expect_err("Shouldn't be able to enter without paying");
//! // turnstile remains locked
//! assert_eq!(machine.state, TurnstileState::Locked);
//! // so we pay as we're instructed to
//! machine.dispatch(TurnstileEvent::PaymentReceived);
//! assert_eq!(machine.state, TurnstileState::Unlocked); // and the turnstile unlocks
//! // then we can enter just fine
//! machine.dispatch(TurnstileEvent::PersonEntering); // no error => we're allowed through
//! assert_eq!(machine.state, TurnstileState::Locked); // then state returns to locked
//!
//! // or if we try to pay twice, we also get a helpful error
//! machine.dispatch(TurnstileEvent::PaymentReceived);
//! machine.dispatch(TurnstileEvent::PaymentReceived).expect_err("Shouldn't be able to enter without paying");
//! // turnstile remains unlocked
//! assert_eq!(machine.state, TurnstileState::Unlocked);
//! ```

use std::marker::PhantomData;

pub struct Machine<StateType, Event>
where
    StateType: State<Event>,
{
    pub state: StateType,
    // FIXME: remove this phantom data when possible
    _event: PhantomData<Event>,
}

impl<StateType, Event> Machine<StateType, Event>
where
    StateType: State<Event>,
{
    pub fn new(initial_state: StateType) -> Self {
        Self {
            state: initial_state,
            _event: PhantomData,
        }
    }

    pub fn dispatch(&mut self, event: Event) -> Result<(), String> {
        self.state = self.state.apply(event)?;

        // FIXME: not sure if it'd be more helpful to return a value here
        Ok(())
    }
}

pub trait State<Event> {
    // FIXME: can probably do better than String as the error type
    fn apply(&self, event: Event) -> Result<Self, String>
    where
        Self: Sized;
}
