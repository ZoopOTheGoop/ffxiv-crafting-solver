//! [`Error`] values for use in executing actions in a prospective manner.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

use super::{Action, ActionComponents, ActionOutcome, StateDelta};

use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// A [`Result`] alias that can either the outcome of crafting, or an explanation
/// as to why the action could not be used. This is mainly used for "prospecitve execution",
/// i.e. the executing an action you know can't complete in order to see the benefit you'd gain
/// if you could.
pub type ActionResult = Result<ActionOutcome, ActionError>;

/// An [`Error`] encoding the reason an action could not be executed. Each variant
/// includes an outcome that encodes what would have happened had the action succeeded.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ActionError {
    /// There was not enough CP to execute the action.
    TooLittleCp(ActionOutcome),
    /// The action was not allowed in this state for some reason.
    ActionInvalid(ActionOutcome),
    /// There was both not enough CP, and it couldn't execute anyway.
    NoCpAndInvalid(ActionOutcome),
}

impl ActionError {
    /// Determines if an action that was prospectively executed is really
    /// executable or not, and either returns this error, or an `Ok`.
    pub(super) fn from_delta_state<C, M, A>(
        delta: StateDelta,
        state: &CraftingState<C, M>,
        action: &A,
        outcome: ActionOutcome,
    ) -> ActionResult
    where
        C: Condition,
        M: QualityMap,
        A: Action + ActionComponents,
    {
        match (state.curr_cp + delta.added_cp, action.can_execute(state)) {
            (0..=i16::MAX, true) => Ok(outcome),
            (i16::MIN..=-1, false) => Err(Self::NoCpAndInvalid(outcome)),
            (i16::MIN..=-1, true) => Err(Self::TooLittleCp(outcome)),
            (0..=i16::MAX, false) => Err(Self::ActionInvalid(outcome)),
        }
    }
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (err_msg, outcome) = match self {
            Self::TooLittleCp(ref outcome) => ("Too little CP to execute this action ", outcome),
            Self::ActionInvalid(ref outcome) => {
                ("Cannot execute this action in the current state ", outcome)
            }
            Self::NoCpAndInvalid(ref outcome) => (
                "Too little CP to execute this action, \
            and even if there was enough CP the action isn't avilable in this state anyway",
                outcome,
            ),
        };

        write!(
            f,
            "{}, but if it were executed the resulting change would be: {:?}",
            err_msg, outcome
        )
    }
}

impl Error for ActionError {}
