use crate::dragon::BattleDragon;

#[derive(Clone, Copy)]
pub struct MoveStats {
    pub accuracy: u32,
    pub base_power: u32,
    pub crit_calc: u8,
}

impl MoveStats {
    pub fn new(base_power: u32, accuracy: u32) -> Self {
        Self {
            accuracy,
            base_power,
            crit_calc: 0,
        }
    }
}

pub enum MoveResult {
    Succeeded,
    Failed,
    Missed,
}

pub struct MoveData {
    pub(crate) stats: MoveStats,
    pub(crate) offender: Box<Move>,
}

impl MoveData {
    pub fn new_damaging(base_power: u32, accuracy: u32) -> Self {
        Self {
            stats: MoveStats {
                accuracy,
                base_power,
                crit_calc: 0,
            },
            offender: Box::new(|dragon, stats| {
                match dragon.offend(*stats) {
                    Some(_) => { MoveResult::Succeeded },
                    None => MoveResult::Failed
                }
            }),
        }
    }

    pub fn new_special(offender: Box<Move>) -> Self {
        Self {
            stats: MoveStats::new(0, 100),
            offender
        }
    }
}

/// The first BattleDragon instance is the user, the second is the
/// opponent.
pub type Move = dyn Fn(&mut BattleDragon, &MoveStats) -> MoveResult;