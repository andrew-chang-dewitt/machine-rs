//! A simple state machine lib
//!
//! # slightly more complex
//!
//! let's model a turnstile instead
//!
//! ```
//! use machine::{Machine, MachineError, State};
//!
//! #[derive(Clone, Copy, Debug, PartialEq)]
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
//!   fn apply(&self, event: TurnstileEvent) -> Result<Self, MachineError<Self, TurnstileEvent>> {
//!     match self {
//!       Self::Locked => match event {
//!         TurnstileEvent::PaymentReceived => Ok( Self::Unlocked ),
//!         // replace string error
//!         // _ => Err("Payment required for entry.".to_owned())
//!         // w/ actual error type
//!         _ => Err(MachineError::InvalidEvent(*self, event))
//!       }
//!
//!       Self::Unlocked => match event {
//!         TurnstileEvent::PersonEntering => Ok( Self::Locked ),
//!         // replace string error
//!         // _ => Err("Payment already received, unable to accept payment at this time.".to_owned())
//!         // w/ actual error type
//!         _ => Err(MachineError::InvalidEvent(*self, event))
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
//! let locked_err = machine.dispatch(TurnstileEvent::PersonEntering).expect_err("Shouldn't be able to enter without paying");
//! // descriptive error is returned
//! assert_eq!(
//!     locked_err,
//!     MachineError::InvalidEvent(TurnstileState::Locked, TurnstileEvent::PersonEntering));
//! // and turnstile remains locked
//! assert_eq!(machine.state, TurnstileState::Locked);
//! // so we pay as we're instructed to
//! machine.dispatch(TurnstileEvent::PaymentReceived);
//! assert_eq!(machine.state, TurnstileState::Unlocked); // and the turnstile unlocks
//! // then we can enter just fine
//! machine.dispatch(TurnstileEvent::PersonEntering); // no error => we're allowed through
//! assert_eq!(machine.state, TurnstileState::Locked); // then state returns to locked
//!
//! // or if we try to pay twice, we also get a helpful error
//! machine.dispatch(TurnstileEvent::PaymentReceived); // pay once here, then again below
//! let paid_err = machine.dispatch(TurnstileEvent::PaymentReceived).expect_err("Shouldn't be able to enter without paying");
//! assert_eq!(
//!     paid_err,
//!     MachineError::InvalidEvent(TurnstileState::Unlocked, TurnstileEvent::PaymentReceived));
//! // turnstile remains unlocked
//! assert_eq!(machine.state, TurnstileState::Unlocked);
//! ```

use std::{error, fmt, marker::PhantomData};

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

    pub fn dispatch(&mut self, event: Event) -> Result<(), MachineError<StateType, Event>> {
        self.state = self.state.apply(event).map_err(|e| e.into())?;

        // FIXME: not sure if it'd be more helpful to return a value here
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum MachineError<State, Event> {
    InvalidEvent(State, Event),
}

impl<State, Event> fmt::Display for MachineError<State, Event>
where
    State: fmt::Display,
    Event: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MachineError::InvalidEvent(ref s, ref e) => {
                write!(f, "Invalid Event, {e} for State {s}")
            }
        }
    }
}

impl<State, Event> error::Error for MachineError<State, Event>
where
    State: fmt::Display + fmt::Debug,
    Event: fmt::Display + fmt::Debug,
{
    fn description(&self) -> &str {
        todo!()
    }

    // FIXME: looks like we only need this if any of our MachineError variants encapsulate another
    // Error type
    // fn cause(&self) -> Option<&dyn error::Error> {
    //     todo!()
    // }
}

pub trait State<Event, ErrorType = MachineError<Self, Event>> {
    fn apply(&self, event: Event) -> Result<Self, ErrorType>
    where
        Self: Sized;
}
