pub mod battle;
pub mod dragon;
pub mod effect;
pub use effect::effects;
pub mod data;
pub mod moves;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        dragon::{BattleDragon, DragonData, Stats},
        effects,
        moves::MoveData,
    };

    #[test]
    fn it_works() {
        let mew = BattleDragon::new(DragonData::new("Mew", Stats::new()), vec![]);
        let enemy_mew = BattleDragon::new(DragonData::new("Mew", Stats::new()), vec![]);

        let (mew, enemy_mew) = BattleDragon::create_bounded_pair(mew, enemy_mew);
        let mewtwo = BattleDragon::new_with_rc(
            DragonData::new("Mewtwo", Stats::new_exact(110, 90, 106)),
            vec![],
        );
        {
            let mut mew_b = mew.borrow_mut();
            let pound = MoveData::new_damaging(40, 100);
            mew_b.attack(&pound).unwrap();
            mew_b.switch(&mewtwo).unwrap();
        }
        println!("enemy_mew.hp = {}", enemy_mew.borrow().hp);
        {
            let mut mewtwo_b = mewtwo.borrow_mut();
            let confusion = MoveData::new_damaging(50, 100);
            mewtwo_b.attack(&confusion).unwrap();
        }
        println!("enemy_mew.hp = {}", enemy_mew.borrow().hp);
    }
}
