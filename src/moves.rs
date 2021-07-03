use std::cmp::min;

use rand::Rng;

use crate::{
    battle::Messenger,
    events::MoveEvent,
    party::{PartyItem, RelativePartyId},
};

#[derive(Clone, Copy)]
pub struct MoveStats {
    pub accuracy: u32,
    pub base_power: u32,
    pub(crate) crit_calc: u8,
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
    Succeeded(Option<MoveEvent>),
    Failed,
    Missed,
}

pub fn calculate_static_damage(user_attack: u32, opponent_defense: u32, base_power: u32) -> u32 {
    22 * user_attack * base_power / opponent_defense / 50 + 2
}

pub fn calculate_random_damage(
    user_attack: u32,
    opponent_defense: u32,
    base_power: u32,
    crit: u8,
) -> u32 {
    let base_damage = calculate_static_damage(user_attack, opponent_defense, base_power);
    let crit_chance = (&[24., 8., 2., 1.])[min(3, crit) as usize];
    if rand::thread_rng().gen_bool(1. / crit_chance) {
        base_damage + base_damage / 2
    } else {
        base_damage
    }
}

pub trait MoveTrait<T: Messenger> {
    fn attack_opponent(
        &self,
        opponent: &mut PartyItem,
        user: &PartyItem,
        messenger: &T,
    ) -> MoveResult;
    fn apply_to_user(
        &self,
        _user: &mut PartyItem,
        _opponent: &PartyItem,
        _messenger: &T,
    ) -> Option<MoveEvent> {
        None
    }
    fn get_name(&self) -> &str;
}

pub struct SimpleDamagingMove {
    base_power: u32,
    crit_boost: u8,
    name: String,
}

impl SimpleDamagingMove {
    pub fn new(name: String, base_power: u32) -> Self {
        Self {
            base_power,
            crit_boost: 0,
            name,
        }
    }

    pub fn new_crit(name: String, base_power: u32, crit_boost: u8) -> Self {
        Self {
            base_power,
            crit_boost,
            name,
        }
    }
}

impl<T: Messenger> MoveTrait<T> for SimpleDamagingMove {
    fn attack_opponent(
        &self,
        opponent: &mut PartyItem,
        user: &PartyItem,
        _messenger: &T,
    ) -> MoveResult {
        let move_stats = MoveStats {
            accuracy: 100,
            base_power: self.base_power,
            crit_calc: self.crit_boost,
        };
        let stats = user.offend(move_stats, opponent.calc_stages(), |m, o| {
            opponent.defend(m, o)
        });
        let (user_stages, move_stats, opponent_stages) = match stats {
            None => return MoveResult::Failed,
            Some(s) => s,
        };
        let user_stats = user.dragon.stats().apply_stages(user_stages);
        let opponent_stats = opponent.dragon.stats().apply_stages(opponent_stages);

        let final_damage = calculate_random_damage(
            user_stats.attack,
            opponent_stats.defense,
            move_stats.base_power,
            move_stats.crit_calc,
        );
        opponent.damage(final_damage);

        MoveResult::Succeeded(Some(MoveEvent::Damaged {
            rel_party_id: RelativePartyId::Opposing,
            damage_amount: final_damage,
        }))
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
