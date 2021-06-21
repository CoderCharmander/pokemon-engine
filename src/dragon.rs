use std::cmp::max;
use std::ops::Add;
use std::ops::Mul;

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

#[derive(Clone, Copy)]
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

    pub fn calculate_hp(&self) -> u32 {
        self.hp + 5
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

impl Add<StatStages> for StatStages {
    fn add(self, rhs: StatStages) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            defense: self.defense + rhs.defense,
            accuracy: self.accuracy + rhs.accuracy,
            evasion: self.evasion + rhs.evasion,
        }
    }

    type Output = StatStages;
}

#[derive(Clone, Copy)]
pub struct BattleDragon {
    base_stats: Stats,
    pub hp: u32,
}

impl BattleDragon {
    pub fn new(base_stats: Stats) -> BattleDragon {
        Self {
            hp: base_stats.calculate_hp(),
            base_stats,
        }
    }

    pub fn stats(&self) -> &Stats {
        &self.base_stats
    }
}
