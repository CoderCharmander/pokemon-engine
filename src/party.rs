use crate::{
    dragon::{BattleDragon, StatStages, Stats},
    effect::LongTermEffectTrait,
    moves::{calculate_static_damage, MoveStats},
};

pub struct PartyItem {
    pub(crate) dragon: BattleDragon,
    pub(crate) effects: Vec<Box<dyn LongTermEffectTrait>>,
}

impl PartyItem {
    pub fn new(dragon: BattleDragon) -> Self {
        Self {
            dragon,
            effects: vec![],
        }
    }
    pub fn calc_stages(&self) -> StatStages {
        self.effects
            .iter()
            .fold(StatStages::new(), |s, e| e.stat_calculation(s))
    }

    pub fn calc_stats(&self) -> Stats {
        self.dragon.stats().apply_stages(self.calc_stages())
    }

    pub fn calc_damage(&self, base_power: u32, opponent_defense: u32) -> u32 {
        calculate_static_damage(self.calc_stats().attack, opponent_defense, base_power)
    }

    /// Returns None if an effect does not allow the operation to progress.
    pub fn defend(
        &self,
        move_stats: MoveStats,
        opponent_stages: StatStages,
    ) -> Option<(StatStages, MoveStats, StatStages)> {
        self.effects.iter().fold(
            Some((StatStages::new(), move_stats, opponent_stages)),
            |s, e| s.and_then(|(u, m, o)| e.defending(u, m, o)),
        )
    }

    /// Calculate (potentially modify) the stat stages and the move stats for
    /// attacking. Also calls the closure given in `defender` on the calculated
    /// stats. Usually, `offend` should be called like this:
    /// ```text
    /// user.offend(move_stats, opponent.calc_stages(), |(m, o)| opponent.defend(m, o))
    /// ```
    ///
    pub fn offend<T>(
        &self,
        move_stats: MoveStats,
        opponent_stages: StatStages,
        defender: T,
    ) -> Option<(StatStages, MoveStats, StatStages)>
    where
        T: FnOnce(MoveStats, StatStages) -> Option<(StatStages, MoveStats, StatStages)>,
    {
        let (stages, move_stats, opponent_stages) = self.effects.iter().fold(
            Some((self.calc_stages(), move_stats, opponent_stages)),
            |s, e| s.and_then(|(u, m, o)| e.offending(u, m, o)),
        )?;
        let (powered_opponent_stages, move_stats, stages) = defender(move_stats, stages)?;
        Some((
            stages,
            move_stats,
            opponent_stages + powered_opponent_stages,
        ))
    }

    pub fn may_switch(&self) -> bool {
        self.effects
            .iter()
            .fold(Some(()), |s, e| s.and_then(|_| e.switching()))
            .is_some()
    }

    pub fn add_effect(&mut self, effect: Box<dyn LongTermEffectTrait>) {
        let (attach, dragon) = effect.apply(self.dragon);
        self.dragon = dragon;
        if attach {
            self.effects.push(effect);
        }
    }

    /// Reduces the HP of the dragon by `amount`. Returns
    /// true if the dragon has fainted as the result from
    /// the damage.
    pub fn damage(&mut self, amount: u32) -> bool {
        if amount < self.dragon.hp {
            self.dragon.hp -= amount;
            false
        } else {
            self.dragon.hp = 0;
            true
        }
    }
}

pub struct Party {
    pub(crate) items: Vec<PartyItem>,
    pub(crate) active: usize,
}

impl Party {
    pub fn new_empty() -> Self {
        Self {
            items: vec![],
            active: 0,
        }
    }

    pub fn new_from_vec(items: Vec<PartyItem>) -> Self {
        Self { items, active: 0 }
    }

    /// Gets a reference to the active dragon.
    pub fn active(&self) -> &PartyItem {
        &self.items[self.active]
    }

    /// Gets a mutable reference to the active dragon.
    pub fn active_mut(&mut self) -> &mut PartyItem {
        &mut self.items[self.active]
    }

    /// Returns false if the switch operation was canceled by an active effect.
    pub fn switch(&mut self, next: usize) -> bool {
        if self.active().may_switch() {
            self.active = next;
            true
        } else {
            false
        }
    }

    pub fn add_dragon(&mut self, dragon: BattleDragon) {
        self.items.push(PartyItem::new(dragon));
    }
}
