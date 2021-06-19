use std::cell::RefCell;
use std::cmp::{max, min};
use std::ops::Mul;
use std::rc::{Rc, Weak};

use rand::Rng;

use crate::effect::LongTermEffectTrait;
use crate::moves::{MoveData, MoveResult, MoveStats};

pub struct DragonData {
    pub name: String,
    pub base_stats: Stats,
}

impl DragonData {
    pub fn new(name: &str, base_stats: Stats) -> Self {
        Self {
            name: name.to_string(),
            base_stats,
        }
    }
}

pub struct Stats {
    pub attack: u32,
    pub defense: u32,
    pub hp: u32,
}

fn apply_stat_stage(base_value: u32, stage: i8) -> u32 {
    (base_value as f32 * (max(2, 2 + stage) as f32 / max(2, 2 - stage) as f32)) as u32
}

impl Stats {
    pub fn new() -> Self {
        Self {
            attack: 100,
            defense: 100,
            hp: 100,
        }
    }

    pub fn new_exact(attack: u32, defense: u32, hp: u32) -> Self {
        Self {
            attack,
            defense,
            hp,
        }
    }

    pub fn apply_stages(&self, stages: StatStages) -> Self {
        Self {
            attack: apply_stat_stage(self.attack, stages.attack),
            defense: apply_stat_stage(self.defense, stages.defense),
            hp: self.hp,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Mul<StatStages> for Stats {
    type Output = Stats;
    fn mul(self, rhs: StatStages) -> Self::Output {
        self.apply_stages(rhs)
    }
}

#[derive(Clone, Copy)]
pub struct StatStages {
    pub attack: i8,
    pub defense: i8,
    pub accuracy: i8,
    pub evasion: i8,
}

impl StatStages {
    pub fn new() -> Self {
        Self {
            attack: 0,
            defense: 0,
            accuracy: 0,
            evasion: 0,
        }
    }
}

impl Default for StatStages {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BattleDragon {
    data: DragonData,
    pub opponent: Option<Weak<RefCell<BattleDragon>>>,
    effects: Vec<(Box<dyn LongTermEffectTrait>, u16)>,
    pub hp: u32,
}

impl BattleDragon {
    pub fn new(data: DragonData, abilities: Vec<Box<dyn LongTermEffectTrait>>) -> BattleDragon {
        let mut out = Self {
            data,
            opponent: None,
            effects: Vec::<(Box<dyn LongTermEffectTrait>, u16)>::with_capacity(abilities.len()),
            hp: 0,
        };
        for a in abilities {
            out.effects.push((a, 0));
        }
        out.hp = out.data.base_stats.hp + 5;
        out
    }

    pub fn new_with_rc(data: DragonData, abilities: Vec<Box<dyn LongTermEffectTrait>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(data, abilities)))
    }

    pub fn create_bounded_pair(
        dragon1: BattleDragon,
        mut dragon2: BattleDragon,
    ) -> (Rc<RefCell<BattleDragon>>, Rc<RefCell<BattleDragon>>) {
        let dragon1_rc = Rc::new(RefCell::new(dragon1));
        dragon2.opponent = Some(Rc::downgrade(&dragon1_rc));
        let dragon2_rc = Rc::new(RefCell::new(dragon2));
        {
            let mut dragon1 = dragon1_rc.borrow_mut();
            dragon1.opponent = Some(Rc::downgrade(&dragon2_rc));
        }
        (dragon1_rc, dragon2_rc)
    }

    fn notify_effects<T>(&mut self, mut caller: T) -> Option<()>
    where
        T: FnMut(&Box<dyn LongTermEffectTrait>) -> Option<bool>,
    {
        let mut result = true;
        let mut removed: Vec<usize> = vec![];
        for (i, (e, _)) in self.effects.iter().enumerate() {
            match caller(e) {
                Some(c) => result = result && c,
                None => removed.push(i),
            }
        }
        for r in removed.iter().rev() {
            let (_, _) = self.effects.remove(*r);
        }
        if result {
            Some(())
        } else {
            None
        }
    }

    pub fn turn(&mut self) {
        let mut removed = Vec::new();
        let mut queue = vec![];
        for (i, (e, d)) in self.effects.iter_mut().enumerate() {
            *d += 1;
            if e.turn(*d, &mut queue).is_none() {
                removed.push(i);
            }
        }

        for r in removed.iter().rev() {
            let _ = self.effects.remove(*r);
        }

        for e in queue {
            self.add_effect(e);
        }
    }

    pub fn calculate_stats(&mut self) -> Option<StatStages> {
        let mut stages = StatStages::new();
        self.notify_effects(|e| e.stat_calculation(&mut stages))?;
        Some(stages)
    }

    pub fn defend(
        &mut self,
        attack: &mut MoveStats,
        opponent_stages: &mut StatStages,
    ) -> Option<StatStages> {
        let mut stages = self.calculate_stats()?;
        self.notify_effects(|e| e.defending(&mut stages, attack, opponent_stages))?;
        Some(stages)
    }

    pub fn offend(&mut self, mut attack_stats: MoveStats) -> Option<()> {
        let mut stages = self.calculate_stats()?;

        let opponent_rc = self.opponent.as_ref()?.upgrade()?;
        let mut opponent = opponent_rc.borrow_mut();

        let mut opponent_stages = opponent.defend(&mut attack_stats, &mut stages)?;
        self.notify_effects(|e| e.offending(&mut stages, &mut attack_stats, &mut opponent_stages))?;

        let accuracy_stage = stages.accuracy - opponent_stages.evasion;
        let move_accuracy = apply_stat_stage(attack_stats.accuracy, accuracy_stage);
        if rand::thread_rng().gen_range(0..100) >= move_accuracy {
            return None;
        }

        let stats = self.data.base_stats.apply_stages(stages);
        let opponent_stats = opponent.data.base_stats.apply_stages(opponent_stages);

        let crit_chance = (&[24., 8., 2., 1.])[min(3, attack_stats.crit_calc) as usize]; // rust devs come and fix this!!!!!!!

        let crit = if rand::thread_rng().gen_bool(1. / crit_chance) {
            1.5
        } else {
            1.
        };

        let damage = ((22 * stats.attack * attack_stats.base_power / opponent_stats.defense / 50
            + 2) as f64
            * crit)
            .floor();
        opponent.damage(damage as u32);

        Some(())
    }

    pub fn attack(&mut self, attack: &MoveData) -> Option<MoveResult> {
        Some((attack.offender)(self, &attack.stats))
    }

    pub fn damage(&mut self, amount: u32) {
        if amount > self.hp {
            self.hp = 0;
        } else {
            self.hp -= amount;
        }
    }

    pub fn add_effect(&mut self, effect: Box<dyn LongTermEffectTrait>) {
        if effect.apply(self).is_some() {
            self.effects.push((effect, 0));
        }
    }

    pub fn switch(&mut self, another: &Rc<RefCell<BattleDragon>>) -> Option<()> {
        self.notify_effects(|e| e.switching())?;
        let opponent_rc = self.opponent.as_ref()?.upgrade()?;
        let mut opponent = opponent_rc.borrow_mut();
        {
            let mut switched = another.borrow_mut();
            switched.opponent = Some(Rc::downgrade(&opponent_rc));
        }
        opponent.opponent = Some(Rc::downgrade(another));
        Some(())
    }
}
