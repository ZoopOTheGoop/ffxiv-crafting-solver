//! This module and its submodules define the rules for actions that
//! act on a [`CraftingState`](crate::CraftingState) and output the change to that state.

use std::ops::{Add, AddAssign};

use derivative::Derivative;
use rand::Rng;

use crate::{buffs::BuffState, conditions::Condition, quality_map::QualityMap, CraftingState};

pub mod buffs;
pub mod collection;
pub mod errors;
pub mod failure;
pub mod misc;
pub mod progress;
pub mod quality;

use self::{
    buffs::BuffAction,
    errors::{ActionError, ActionResult},
    progress::ProgressAction,
    quality::QualityAction,
};

mod prelude {
    pub use crate::{
        actions::{
            buffs::BuffAction, progress::ProgressAction, quality::QualityAction, Action,
            CanExecute, CpCost, DurabilityFactor, RandomAction,
        },
        CraftingState,
    };
}

/// A `delta` that encodes a change to a given [`CraftingState`]. It can be added to
/// the state with the add operator to yield a new one (or modify it in-place with `+=`).
///
/// This does not check if the state it's being added to matches the state it was generated from,
/// so it's up to the user to keep these values together.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Derivative)]
#[derivative(Default)]
pub struct StateDelta {
    added_quality: u32,
    added_progress: u32,
    new_buffs: BuffState,
    action_durability: i8,
    buff_repair: i8,
    added_cp: i16,
    #[derivative(Default(value = "true"))]
    time_passed: bool,
    final_appraisal_triggered: bool,
}

impl StateDelta {
    /// Creates a new `StateDelta` that sets its buff state to the state of
    /// the given input, with all else being default.
    fn inherit_buffs(buffs: BuffState) -> Self {
        Self {
            new_buffs: buffs,
            ..Default::default()
        }
    }

    /// Returns a version of `self` where the repair due to buffs is zerod out.
    ///
    /// Useful because [`Manipulation`]'s buff only repairs after a durability check.
    ///
    /// [`Manipulation`]: crate::buffs::durability::Manipulation
    fn no_repair(self) -> Self {
        Self {
            buff_repair: 0,
            ..self
        }
    }
}

impl<'a, C, M> Add<StateDelta> for CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    type Output = CraftingState<'a, C, M>;
    fn add(mut self, other: StateDelta) -> Self::Output {
        self += other;
        self
    }
}

impl<'a, C, M> Add<StateDelta> for &'a CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    type Output = CraftingState<'a, C, M>;
    fn add(self, other: StateDelta) -> Self::Output {
        CraftingState {
            curr_quality: self.curr_quality + other.added_quality,
            curr_progress: self.curr_progress + other.added_progress,
            buffs: other.new_buffs,
            curr_durability: (self.curr_durability + other.buff_repair + other.action_durability)
                .min(self.problem_def.recipe.max_durability),
            curr_cp: (self.curr_cp + other.added_cp).min(self.problem_def.character.max_cp),
            first_step: self.first_step && !other.time_passed,
            ..*self
        }
    }
}

impl<'a, C, M> AddAssign<StateDelta> for CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    fn add_assign(&mut self, rhs: StateDelta) {
        self.curr_quality += rhs.added_quality;
        self.curr_progress += rhs.added_progress;
        self.curr_cp += rhs.added_cp;
        self.curr_cp = self.curr_cp.min(self.problem_def.character.max_cp);
        self.curr_durability += rhs.buff_repair + rhs.action_durability;
        self.curr_durability = self
            .curr_durability
            .min(self.problem_def.recipe.max_durability);
        self.buffs = rhs.new_buffs;
        self.first_step = self.first_step && !rhs.time_passed
    }
}

/// The outcome of executing an [`Action`] on a given [state](CraftingState).
///
/// To extract the eventual outcome (e.g. HQ chance or collectability), please
/// use the `map_quality` method for convenience.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionOutcome {
    /// The craft has failed, i.e. durability has hit 0 (or below) without progress hitting
    /// its required amount. This will be generated even if [`Manipulation`] is active and would repair
    /// it above 0, as it does in-game. The resulting [`StateDelta`] will have this quantity zerod out in that
    /// case.
    ///
    /// [`Manipulation`]: crate::buffs::durability::Manipulation
    Failure(StateDelta),
    /// The craft has finished, i.e. its progress hit the required value. To extract the HQ chance or
    /// collectability use `map_quality`.
    ///
    /// Like [`Failure`](ActionOutcome::Failure), this will zero out any buff-based durability repair
    /// that applies after breaking is checked, for adequate analysis of whether an action would have failed
    /// had the goal not been met.
    Completed(StateDelta),
    /// The craft has neither completed nor failed.
    InProgress(StateDelta),
}

impl ActionOutcome {
    /// Determines the outcome from the given state/delta pair. Taking into account the small
    /// rules such as post-repair and order of failure/completion evaluation.
    fn from_delta_state<C, M>(delta: StateDelta, state: &CraftingState<C, M>) -> Self
    where
        C: Condition,
        M: QualityMap,
    {
        let finished = state.problem_def.recipe.max_progress..=u32::MAX;
        let unfinished = 0..state.problem_def.recipe.max_progress;

        match (state.curr_durability + delta.action_durability, state.curr_progress + delta.added_progress) {
            (_, progress) if finished.contains(&progress) => Self::Completed(delta.no_repair()),
            (i8::MIN..=0, _) => Self::Failure(delta.no_repair()),
            (1..=i8::MAX, progress) if unfinished.contains(&progress) => Self::InProgress(delta),
            _ => unreachable!("Rust can't tell that the patterns are complete, and runtime patterns aren't allowed")
        }
    }

    /// A convenience method for getting the HQ chance out of an outcome and [state](CraftingState).
    ///
    /// This will return [`None`] if this enum is not [`Complete`](ActionOutcome::Completed), but otherwise
    /// will map the outcome's quality to HQ Chance or Collectability, as defined by the generic type.
    pub fn map_quality<C: Condition, M: QualityMap>(
        self,
        state: &CraftingState<C, M>,
    ) -> Option<M::Outcome> {
        match self {
            Self::Completed(StateDelta { added_quality, .. }) => Some(M::convert(
                added_quality + state.curr_quality,
                state.problem_def.recipe.max_quality,
            )),
            _ => None,
        }
    }
}

/// All the components of an [`Action`] -- this is autoimplemented for the implementors. Doing it
/// this was allows you to develop "meta" actions which encapsulate multiple actions and still
/// override [`act`] and such for dispatch with less branching
///
/// [`act`]: Action::act
pub trait ActionComponents:
    BuffAction + ProgressAction + QualityAction + DurabilityFactor + CpCost + CanExecute + TimePassing
{
}

impl<T> ActionComponents for T where
    T: BuffAction
        + ProgressAction
        + QualityAction
        + DurabilityFactor
        + CpCost
        + CanExecute
        + TimePassing
{
}

/// A crafting action. This generates a description of how to get from the current state to the next state,
/// and the outcome of taking that action (if any). The only value not encoded by the resulting outcome is the
/// [`Condition`] value, which is left up to the caller for finer-grained control over the distribution and rng state.
///
/// This is a general `trait` implementation, but must be manually derived so that wrapper types can efficiently override
/// the default [`act`] implementation (and similar) to defer to the underlying action's [`act`] instead of
/// making a separate branch for subtrait method call.
///
/// The default implementation reflects the rules of FFXIV crafting, and all actions should be able to be represented by
/// the algorithms implemented by default by this trait. Only overriding the methods on the requirements such as
/// [`ProgressAction`] should be needed to control the actual behavior (barring FFXIV adding a very crazy ability).
///
/// [`act`]: Action::act
pub trait Action: Sized + ActionComponents {
    /// Prospectively executes an action. This means that even if the action cannot be executed due to
    /// e.g. not having enough CP or it not being available in that state, it will still compute it as
    /// if it had succeeded, returning the outcome and a marker indicating why it cannot execute.
    ///
    /// If you need to take into account an action's rng, use [`Action::propective_act_random`] or
    /// [`Action::prospective_act_and_fail`].
    fn prospective_act<C, M>(self, state: &CraftingState<C, M>) -> ActionResult
    where
        C: Condition,
        M: QualityMap,
    {
        let mut delta = StateDelta::inherit_buffs(state.buffs);

        // Don't check CP or viability yet, prospectively execute
        delta.added_cp = self.cp_cost(state);

        delta.added_progress = self.progress(state);
        let appraised = state.buffs.progress.final_appraisal.handle_progress(
            state,
            delta.added_progress,
            &mut delta.new_buffs,
        );

        if appraised < delta.added_progress {
            delta.final_appraisal_triggered = true;
        }
        delta.added_progress = appraised;

        delta.added_quality = self.quality(state);
        delta.action_durability = self.durability(&state.buffs, &state.condition);

        self.deactivate_buff(state, &mut delta.new_buffs);

        if self.time_passed(state) {
            delta.new_buffs.decay();
        } else {
            // Combo actions still fail to trigger after using
            // time-agnostic actions
            delta.new_buffs.combo.decay();

            // Repair isn't applied during a "time stop" so it's in the else rather
            // than after.
            delta.buff_repair = state.buffs.durability.repair();
        }
        self.buff(state, &mut delta.new_buffs);

        let prospective_outcome = ActionOutcome::from_delta_state(delta, state);

        ActionError::from_delta_state(delta, state, &self, prospective_outcome)
    }

    /// Executes an action. In debug mode this will panic early if the action is not able to be
    /// executed (e.g. low CP or just a plain inexecutable action). If you want to speculatively
    /// execute, use [`Action::prospective_act`].
    ///
    /// If you need to take into account an action's rng, use [`Action::act_random`] or
    /// [`Action::act_and_fail`].
    fn act<C, M>(self, state: &CraftingState<C, M>) -> ActionOutcome
    where
        C: Condition,
        M: QualityMap,
    {
        let mut delta = StateDelta::inherit_buffs(state.buffs);

        delta.added_cp = self.cp_cost(state);

        #[cfg(debug_assertions)]
        let can_act = self.can_execute(state);

        #[cfg(debug_assertions)]
        match(state.curr_cp + delta.added_cp < 0, can_act) {
            (true, true) => panic!("Attempted to use action with not enough CP and \
            in a state where that action is impossible, to prospectively execute use `prospective_act`"),
            (true, false) => panic!("Attempted to use action with not enough CP, \
            to prospectively execute use `prospective_act`"),
            (false, true) => panic!("Attempted to use action in an invalid state, \
            to prospectively execute use `prospective_act`"),
            _ => {},
        };

        delta.added_progress = self.progress(state);
        let appraised = state.buffs.progress.final_appraisal.handle_progress(
            state,
            delta.added_progress,
            &mut delta.new_buffs,
        );

        if appraised < delta.added_progress {
            delta.final_appraisal_triggered = true;
        }
        delta.added_progress = appraised;

        delta.added_quality = self.quality(state);
        delta.action_durability = self.durability(&state.buffs, &state.condition);

        delta.buff_repair = state.buffs.durability.repair();

        self.deactivate_buff(state, &mut delta.new_buffs);

        if self.time_passed(state) {
            delta.new_buffs.decay();
        } else {
            // Combo actions still fail to trigger after using
            // time-agnostic actions
            delta.new_buffs.combo.decay();

            // Repair isn't applied during a "time stop" so it's in the else rather
            // than after.
            delta.buff_repair = state.buffs.durability.repair();
        }
        self.buff(state, &mut delta.new_buffs);

        ActionOutcome::from_delta_state(delta, state)
    }

    /// Takes into account a [`RandomAction`]'s chance to fail, and does
    /// a roll on the provided [`Rng`], returning a [`RollOutcome`] that either
    /// executed an action, or its failure form with [`Action::act`].
    fn act_random<R: Rng, C: Condition, M: QualityMap>(
        self,
        rng: &mut R,
        state: &CraftingState<C, M>,
    ) -> RollOutcome<ActionOutcome, ActionOutcome>
    where
        Self: RandomAction,
    {
        match self.roll(rng, state) {
            RollOutcome::Failure(fail) => RollOutcome::Failure(fail.act(state)),
            RollOutcome::Success(me) => RollOutcome::Success(me.act(state)),
        }
    }

    /// Takes into account a [`RandomAction`]'s chance to fail, and does
    /// a roll on the provided [`Rng`], returning a [`RollOutcome`] that either
    /// executed an action, or its failure form with [`Action::prospective_act`].
    fn propective_act_random<R: Rng, C: Condition, M: QualityMap>(
        self,
        rng: &mut R,
        state: &CraftingState<C, M>,
    ) -> RollOutcome<ActionResult, ActionResult>
    where
        Self: RandomAction,
    {
        match self.roll(rng, state) {
            RollOutcome::Failure(fail) => RollOutcome::Failure(fail.prospective_act(state)),
            RollOutcome::Success(me) => RollOutcome::Success(me.prospective_act(state)),
        }
    }

    /// Takes into account a [`RandomAction`]'s chance to fail, executing
    /// [`Action::prospective_act`] for each possibility. This will return an
    /// array with both outcomes, prepended by their probability to fail out of 100.
    ///
    /// The [`Failure`](RollOutcome::Failure) branch will always be first.
    fn prospective_act_and_fail<C: Condition, M: QualityMap>(
        self,
        state: &CraftingState<C, M>,
    ) -> [(u8, RollOutcome<ActionResult, ActionResult>); 2]
    where
        Self: RandomAction,
    {
        let fail_rate = self.fail_rate(state);
        [
            (
                fail_rate,
                RollOutcome::Failure(self.fail_action().prospective_act(state)),
            ),
            (
                100 - fail_rate,
                RollOutcome::Success(self.prospective_act(state)),
            ),
        ]
    }

    /// Takes into account a [`RandomAction`]'s chance to fail, executing
    /// [`Action::act`] for each possibility. This will return an
    /// array with both outcomes, prepended by their probability to fail out of 100.
    ///
    /// The [`Failure`](RollOutcome::Failure) branch will always be first.
    fn act_and_fail<C: Condition, M: QualityMap>(
        self,
        state: &CraftingState<C, M>,
    ) -> [(u8, RollOutcome<ActionOutcome, ActionOutcome>); 2]
    where
        Self: RandomAction,
    {
        let fail_rate = self.fail_rate(state);
        [
            (
                fail_rate,
                RollOutcome::Failure(self.fail_action().act(state)),
            ),
            (100 - fail_rate, RollOutcome::Success(self.act(state))),
        ]
    }
}

/// A trait that denotes an action's ability to determine if it can execute in the current state.
/// Note that this is specifically for actions such as [`MuscleMemory`] that can only be executed
/// on the first action, CP execution availability is determined separately and you should not
/// implement CP cost checks in this trait.
///
/// [`MuscleMemory`]: crate::actions::progress::MuscleMemory
pub trait CanExecute {
    /// Determines whether the action can be executed in the given state.
    #[allow(unused_variables)]
    fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        true
    }
}

/// Defines the amount of durability an [`Action`] uses. Like other action qualities, the
/// [`DURABILITY_USAGE`](DurabilityFactor::DURABILITY_USAGE) factor shouldn't be relied upon
/// directly, instead [`durability`](DurabilityFactor::durability) is used to determine if
/// a different amount of restoration or breakage should take place based on the current state.
///
/// This takes into account the current [`Condition`] in expert crafting, as well as
/// the [`WasteNot`](crate::buffs::durability::WasteNot) buff.
///
/// Currently, aside from the constant, this should not need overriding for any current action
/// and should monomorphize well.
pub trait DurabilityFactor {
    /// How much durability this action uses. Most actions are negative or 0. This is added
    /// to the current state's durability to that means negative means it does damage to the
    /// durability.
    const DURABILITY_USAGE: i8 = -10;

    /// Determines the amount of durability this action will restore or use given the current [`Condition`].
    fn durability<C>(&self, buffs: &BuffState, condition: &C) -> i8
    where
        C: Condition,
    {
        if Self::DURABILITY_USAGE >= 0 {
            return Self::DURABILITY_USAGE;
        }

        let condition_mod = condition.to_durability_modifier() as u64 as f64 / 100.;
        let buff_mod = buffs.durability.durability_cost_mod() as f64 / 100.;

        // TODO: Verify where floors/ceilings might be for sure. I think this is right
        // since when using Prudent Touch during Sturdy I used 3 durability.
        //
        // There may be another floor or ceiling present, but I don't think mathematically
        // any current combination of durabilities can use them since Prudent Touch is the only
        // odd durability action and it can't be used during Waste Not (the only other modifier).
        //
        // NOTE: Floor is used since durability usage is negative - we're actually rounding "away"
        // from zero.
        (Self::DURABILITY_USAGE as f64 * condition_mod * buff_mod).floor() as i8
    }
}

/// Defines the amount of CP an [`Action`] uses. Like other action qualities, the
/// [`CP_COST`](CpCost::CP_COST) factor shouldn't be relied upon
/// directly, instead [`cp_cost`](CpCost::cp_cost) is used to determine if
/// a different amount of CP should be used or restored in the current state.
///
/// This takes into account the current [`Condition`] in expert crafting.
pub trait CpCost {
    /// The amount of CP the action costs under normal circumstances. This is added onto the
    /// state's CP value, so most actions will be negative, and positive means you gain CP.
    const CP_COST: i16 = 0;

    /// Determines the amount of CP this action will restore or use given the current [`Condition`].
    fn cp_cost<C, M>(&self, state: &CraftingState<C, M>) -> i16
    where
        C: Condition,
        M: QualityMap,
    {
        if Self::CP_COST == 0 {
            return 0;
        }

        if Self::CP_COST > 0 {
            return if state.condition.is_good() || state.condition.is_excellent() {
                Self::CP_COST
            } else {
                0
            };
        }

        let condition_mod = state.condition.to_cp_usage_modifier() as u64 as f64 / 100.;

        // Todo: verify where floor/ceil might be
        (Self::CP_COST as f64 * condition_mod) as i16
    }
}

/// The outcome of rolling between two possibilities.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum RollOutcome<A, B> {
    #[allow(missing_docs)]
    Success(A),
    #[allow(missing_docs)]
    Failure(B),
}

/// An action that has a random chance to fail. Like other action qualities, the
/// [`FAIL_RATE`](RandomAction::FAIL_RATE) factor shouldn't be relied upon
/// directly, instead [`fail_rate`](RandomAction::fail_rate) is used to determine if
/// its failure chance is different in the current state (e.g. [`Observe`] combo actions).
///
/// Any action that fails has another dummy action specified by [`FailAction`] that will
/// perform the effects of the failure within a normal [`act`] application (typically an
/// effective no-op, with a notable exception at the moment).
///
/// [`FailAction`]: RandomAction::FailAction
/// [`act`]: Action::act
/// [`Observe`]: crate::actions::misc::Observe
pub trait RandomAction: Sized + Action {
    /// The probability of an action failing under unmodified circumstances. Lower is better.
    /// I'm honestly not sure why I implemented it this way given FFXIV lists success
    /// chance, but it'd be annoying to change it now for not much benefit.
    const FAIL_RATE: u8 = 0;

    /// The dummy action this defers to if it rolls a failure. This action can be executed
    /// in the current action's stead. Deriving this trait by default provides a
    /// [`NullFailure`] that will automatically take on the parent action's CP, durability
    /// cost etc for the typical case where failure essentially just damages the item and
    /// wastes buff duration and CP.
    ///
    /// [`NullFailure`]: self::failure::NullFailure
    type FailAction: Action + ActionComponents;

    /// Rolls a number and in the range `[1,100]` and checks if it's lower than
    /// [`fail_rate`](RandomAction::fail_rate). Then returns a [`RollOutcome`] with
    /// either this [`Action`] or its associated [`FailAction`](RandomAction::FailAction).
    fn roll<R: Rng, C: Condition, M: QualityMap>(
        self,
        rng: &mut R,
        state: &CraftingState<C, M>,
    ) -> RollOutcome<Self, Self::FailAction> {
        let fail_rate = self.fail_rate(state);

        if fail_rate == 0 {
            return RollOutcome::Success(self);
        } else if fail_rate == 100 {
            return RollOutcome::Failure(self.fail_action());
        }

        if rng.gen_range(1..=100) <= fail_rate {
            RollOutcome::Failure(self.fail_action())
        } else {
            RollOutcome::Success(self)
        }
    }

    /// The actual chance of this action failing given the current state. Lower => fails less often.
    ///
    /// By default this is the same as [`FAIL_RATE`](RandomAction::FAIL_RATE), but discounted
    /// by improved failure rate due to conditions.
    #[allow(unused_variables)]
    fn fail_rate<C: Condition, M: QualityMap>(&self, state: &CraftingState<C, M>) -> u8 {
        Self::FAIL_RATE - (state.condition.to_success_rate_modifier() as u8).min(Self::FAIL_RATE)
    }

    /// Constructs the associated [`FailAction`](RandomAction::FailAction).
    fn fail_action(&self) -> Self::FailAction;
}

/// The level the action is learned at. Nothing should need to be overridden in this.
pub trait ActionLevel: Action {
    /// The level this action is learned at.
    const LEVEL: u16 = 0;

    /// A function version that just returns [`LEVEL`], reserved just in
    /// case something weird is done with this later.
    ///
    /// [`LEVEL`]: ActionLevel::LEVEL
    fn level(&self) -> u16 {
        Self::LEVEL
    }
}

/// Denotes whether time passes when an action is taken. This basically means whether
/// or not buffs tick down and whether you lose your first turn bonus. It does not
/// preserve any active combos, however. This is a niche trait largely useful for
/// two abilities.
pub trait TimePassing {
    /// Whether this action passes time as per the rules given in the trait
    /// definition.
    const TIME_PASSED: bool = false;

    /// Currently just defers to [`TIME_PASSED`], but is reserved in case
    /// FFXIV adds actions that stop time only when certain criteria are met.
    ///
    /// [`TIME_PASSED`]: TimePassing::TIME_PASSED
    #[allow(unused_variables)]
    fn time_passed<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        Self::TIME_PASSED
    }
}
