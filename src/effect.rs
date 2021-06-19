use std::u16;

use crate::{
    dragon::{BattleDragon, StatStages},
    moves::MoveStats,
};

/// A long-term effect that is applied on a fighting dragon
/// in each round, or, each event. The event is given in the
/// first parameter, and the effect has the power to modify
/// allowed stats, or outright cancel the action.
///
/// The second parameter is the amount of rounds the effect
/// has been attached to the dragon.
///
/// The effect returns `None` when it should be detached from
/// the dragon, and returns `Some(bool)` otherwise, indicating
/// whether the action should be allowed to happen.
///pub type LongTermEffect = dyn Fn(&BattleStatus, u16) -> Option<bool>;

/// `BattleStatus` indicates an event that has happened in the
/// battle, and now `LongTermEffect`s have the opportunity to
/// modify the stats via the mutable references given in the
/// enum.
pub enum BattleStatus<'a> {
    /// The turn is starting, and the system is calculating the
    /// momentary stats of the dragon, taking stages into account.
    StatCalculation(&'a mut StatStages),
    /// The dragon is using a move to attack the opponent,
    /// or the opponent is attacking the dragon.
    /// The first parameter is the stat stages of the user,
    /// the second is the move's stats, the third is the stat
    /// stages of the opponent.
    Offending(&'a mut StatStages, &'a mut MoveStats, &'a mut StatStages),
    Defending(&'a mut StatStages, &'a mut MoveStats, &'a mut StatStages),
    Switching,
}

pub trait LongTermEffectTrait {
    /// Return a lowercase string corresponding to the type
    /// of the longterm effect.
    fn get_name(&self) -> &str;

    /// Called when the effect is first added to the dragon.
    /// May mutate state.
    fn apply(&self, _dragon: &mut BattleDragon) -> Option<()> {
        Some(())
    }

    /// Called each turn, may add effects to the dragon via the effect
    /// queue passed in the second parameter
    fn turn(
        &mut self,
        _turn: u16,
        _effect_queue: &mut Vec<Box<dyn LongTermEffectTrait>>,
    ) -> Option<()> {
        Some(())
    }

    /// Called when stats are being calculated.
    fn stat_calculation(&self, _stages: &mut StatStages) -> Option<bool> {
        Some(true)
    }
    /// Called when the dragon (user) is attacking an opponent.
    fn offending(
        &self,
        _stages: &mut StatStages,
        _move_stats: &mut MoveStats,
        _opponent_stages: &mut StatStages,
    ) -> Option<bool> {
        Some(true)
    }
    /// Called when the opponent is attacking the dragon.
    fn defending(
        &self,
        _stages: &mut StatStages,
        _move_stats: &mut MoveStats,
        _opponent_stages: &mut StatStages,
    ) -> Option<bool> {
        Some(true)
    }
    /// Called when the dragon is being switched.
    fn switching(&self) -> Option<bool> {
        Some(true)
    }
}

pub mod effects {
    use crate::dragon::{BattleDragon, StatStages};

    use super::LongTermEffectTrait;

    /// Modifies the attack stat stage.
    pub struct AttackStageModifier(i8);
    impl AttackStageModifier {
        pub fn new(amount: i8) -> Self {
            Self(amount)
        }
    }
    impl LongTermEffectTrait for AttackStageModifier {
        fn stat_calculation(&self, stages: &mut StatStages) -> Option<bool> {
            stages.attack += self.0;
            Some(true)
        }

        fn get_name(&self) -> &str {
            "attack_modifier"
        }
    }

    /// Modifies the defense stat stage
    pub struct DefenseStageModifier(i8);
    impl DefenseStageModifier {
        pub fn new(amount: i8) -> Self {
            Self(amount)
        }
    }
    impl LongTermEffectTrait for DefenseStageModifier {
        fn stat_calculation(&self, stages: &mut StatStages) -> Option<bool> {
            stages.defense += self.0;
            Some(true)
        }
        fn get_name(&self) -> &str {
            "defense_modifier"
        }
    }

    /// Calls a closure once it is attached to a BattleDragon, then
    /// removes itself.
    pub struct OneshotEffect<T>(T)
    where
        T: Fn(&mut BattleDragon);
    impl<T> OneshotEffect<T>
    where
        T: Fn(&mut BattleDragon),
    {
        pub fn new(modifier: T) -> Self {
            Self(modifier)
        }
    }

    impl<T> LongTermEffectTrait for OneshotEffect<T>
    where
        T: Fn(&mut BattleDragon),
    {
        fn get_name(&self) -> &str {
            "oneshot"
        }

        fn apply(&self, dragon: &mut BattleDragon) -> Option<()> {
            (self.0)(dragon);
            None
        }
    }
}
