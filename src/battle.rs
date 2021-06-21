use crate::{moves::MoveTrait, party::Party};

pub struct Battlefield {
    parties: (Party, Party),
}

impl Battlefield {
    pub fn new(party_a: Party, party_b: Party) -> Self {
        Self {
            parties: (party_a, party_b)
        }
    }

    pub fn party(&self, id: u8) -> &Party {
        match id {
            0 => &self.parties.0,
            1 => &self.parties.1,
            _ => panic!("Invalid party identifier: {}", id)
        }
    }

    pub fn party_mut(&mut self, id: u8) -> &mut Party {
        match id {
            0 => &mut self.parties.0,
            1 => &mut self.parties.1,
            _ => panic!("Invalid party identifier: {}", id)
        }
    }

    pub fn party_and_opposing(&self, id: u8) -> (&Party, &Party) {
        match id {
            0 => (&self.parties.0, &self.parties.1),
            1 => (&self.parties.1, &self.parties.0),
            _ => panic!("Invalid party identifier while there are two opposing parties: {}", id)
        }
    }

    pub fn party_and_opposing_mut(&mut self, id: u8) -> (&mut Party, &mut Party) {
        match id {
            0 => (&mut self.parties.0, &mut self.parties.1),
            1 => (&mut self.parties.1, &mut self.parties.0),
            _ => panic!("Invalid party identifier while there are two opposing parties: {}", id)
        }
    }

    pub fn attack<T: MoveTrait>(&mut self, party_id: u8, attack: T) {
        let (party, opposing) = self.party_and_opposing_mut(party_id);
        attack.attack_opponent(opposing.active_mut(), party.active());
        attack.apply_to_user(party.active_mut(), opposing.active());
    }
}