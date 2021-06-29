use std::u16;

use crate::{
    dragon::{BattleDragon, StatStages},
    moves::MoveStats,
};

/// If a function returns None, the action is stopped. In
/// other cases, the returned data is used in the action,
/// possibly passing through other effects before.
pub trait LongTermEffectTrait: Send + Sync {
    /// Return a lowercase string corresponding to the type
    /// of the longterm effect.
    fn get_name(&self) -> &str;

    /// Called when the effect is first added to the dragon.
    /// May mutate state.
    fn apply(&self, dragon: BattleDragon) -> (bool, BattleDragon) {
        (true, dragon)
    }

    /// Called each turn, may add effects to the dragon via returning
    /// effects in a vector. Returns false if it should be detached.
    /// Even if the effect is detaching, the other ones in the vector
    /// will be applied.
    fn turn(&mut self, _turn: u16) -> (bool, Option<Vec<Box<dyn LongTermEffectTrait>>>) {
        (true, None)
    }

    /// Called when stats are being calculated.
    fn stat_calculation(&self, stages: StatStages) -> StatStages {
        stages
    }
    /// Called when the dragon (user) is attacking an opponent. May return
    fn offending(
        &self,
        stages: StatStages,
        move_stats: MoveStats,
        opponent_stages: StatStages,
    ) -> Option<(StatStages, MoveStats, StatStages)> {
        Some((stages, move_stats, opponent_stages))
    }
    /// Called when the opponent is attacking the dragon.
    fn defending(
        &self,
        stages: StatStages,
        move_stats: MoveStats,
        opponent_stages: StatStages,
    ) -> Option<(StatStages, MoveStats, StatStages)> {
        Some((stages, move_stats, opponent_stages))
    }
    /// Called when the dragon is being switched.
    fn switching(&self) -> Option<()> {
        Some(())
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
        fn stat_calculation(&self, stages: StatStages) -> StatStages {
            StatStages {
                attack: stages.attack + self.0,
                ..stages
            }
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
        fn stat_calculation(&self, stages: StatStages) -> StatStages {
            StatStages {
                defense: stages.defense + self.0,
                ..stages
            }
        }
        fn get_name(&self) -> &str {
            "defense_modifier"
        }
    }

    /// Calls a closure once it is attached to a BattleDragon, then
    /// removes itself.
    pub struct OneshotEffect<T: Fn(BattleDragon) -> BattleDragon + Send + Sync>(T);
    impl<T: Fn(BattleDragon) -> BattleDragon + Send + Sync> OneshotEffect<T> {
        pub fn new(modifier: T) -> Self {
            Self(modifier)
        }
    }

    impl<T: Fn(BattleDragon) -> BattleDragon + Send + Sync> LongTermEffectTrait for OneshotEffect<T> {
        fn get_name(&self) -> &str {
            "oneshot"
        }

        fn apply(&self, dragon: BattleDragon) -> (bool, BattleDragon) {
            let dragon = self.0(dragon);
            (false, dragon)
        }
    }
}
