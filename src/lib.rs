pub mod battle;
pub mod dragon;
pub mod effect;
pub use effect::effects;
pub mod data;
pub mod moves;
pub mod party;

#[cfg(test)]
mod tests {
    use crate::{
        battle::Battlefield,
        dragon::{BattleDragon, Stats},
        effects,
        moves::SimpleDamagingMove,
        party::{Party, PartyItem},
    };

    #[test]
    fn it_works() {
        let mut mew = PartyItem::new(BattleDragon::new(Stats::new_exact(100, 100, 100)));
        mew.add_effect(Box::new(effects::AttackStageModifier::new(6)));

        let mewtwo = PartyItem::new(BattleDragon::new(Stats::new_exact(110, 90, 106)));

        let opposing_mew = PartyItem::new(BattleDragon::new(Stats::new_exact(100, 100, 100)));

        let mew_party = Party::new_from_vec(vec![mew, mewtwo]);
        let opposing_party = Party::new_from_vec(vec![opposing_mew]);

        let mut battlefield = Battlefield::new(mew_party, opposing_party);

        battlefield.attack(0, SimpleDamagingMove::new(40));
        println!("enemy_mew.hp = {}", battlefield.party(1).active().dragon.hp);

        battlefield.party_mut(0).switch(1);
        battlefield.attack(0, SimpleDamagingMove::new(50));
        println!("enemy_mew.hp = {}", battlefield.party(1).active().dragon.hp);
    }
}
