use crate::{
    events::MoveEvent,
    moves::{MoveResult, MoveTrait},
    party::{Party, PartyId},
};

pub struct Battlefield<T: Messenger> {
    parties: (Party, Party),
    messenger: T,
}

impl<T: Messenger> Battlefield<T> {
    pub fn new(party_a: Party, party_b: Party, msg: T) -> Self {
        Self {
            parties: (party_a, party_b),
            messenger: msg,
        }
    }

    pub fn party(&self, id: PartyId) -> &Party {
        match id {
            PartyId::Party1 => &self.parties.0,
            PartyId::Party2 => &self.parties.1,
        }
    }

    pub fn party_mut(&mut self, id: PartyId) -> &mut Party {
        match id {
            PartyId::Party1 => &mut self.parties.0,
            PartyId::Party2 => &mut self.parties.1,
        }
    }

    pub fn party_and_opposing(&self, id: PartyId) -> (&Party, &Party) {
        (self.party(id), self.party(id.opposing()))
    }

    pub fn party_and_opposing_mut(&mut self, id: PartyId) -> (&mut Party, &mut Party) {
        match id {
            PartyId::Party1 => (&mut self.parties.0, &mut self.parties.1),
            PartyId::Party2 => (&mut self.parties.1, &mut self.parties.0),
        }
    }

    fn party_opposing_messenger_mut(&mut self, id: PartyId) -> (&mut Party, &mut Party, &T) {
        match id {
            PartyId::Party1 => (&mut self.parties.0, &mut self.parties.1, &self.messenger),
            PartyId::Party2 => (&mut self.parties.1, &mut self.parties.0, &self.messenger),
        }
    }

    pub fn attack(&mut self, party_id: PartyId, attack: &dyn MoveTrait<T>) {
        self.messenger.on_attack(&self, party_id, attack.get_name());
        let attack_result;
        let user_apply_result;
        {
            let (party, opposing, messenger) = self.party_opposing_messenger_mut(party_id);
            attack_result =
                attack.attack_opponent(opposing.active_mut(), party.active(), messenger);
            user_apply_result =
                attack.apply_to_user(party.active_mut(), opposing.active(), messenger);
        }
        if let MoveResult::Succeeded(Some(event)) = attack_result {
            send_move_event_to_messenger(&self.messenger, event, party_id, &self);
        }
        if let Some(event) = user_apply_result {
            send_move_event_to_messenger(&self.messenger, event, party_id, &self);
        }
    }

    pub fn turn(&mut self) {
        self.parties.0.active_mut().turn();
        self.parties.1.active_mut().turn();
    }
}

fn send_move_event_to_messenger<T: Messenger>(
    messenger: &T,
    move_event: MoveEvent,
    user_party_id: PartyId,
    battlefield: &Battlefield<T>,
) {
    match move_event {
        MoveEvent::Damaged {
            rel_party_id,
            damage_amount,
        } => {
            messenger.on_damage(
                battlefield,
                user_party_id.relative(rel_party_id),
                damage_amount,
            );
        }
        MoveEvent::Effected {
            rel_party_id,
            description,
        } => {
            messenger.on_effect_applied(
                battlefield,
                user_party_id.relative(rel_party_id),
                &description,
            );
        }
    }
}

pub trait Messenger: Send + Sync + Sized {
    fn on_attack(&self, field: &Battlefield<Self>, party: PartyId, move_name: &str);
    fn on_damage(&self, field: &Battlefield<Self>, party: PartyId, amount: u32);
    fn on_switch(&self, field: &Battlefield<Self>, party: PartyId, original: u8, switched: u8);
    fn on_effect_applied(&self, field: &Battlefield<Self>, party: PartyId, effect_desc: &str);
}

pub struct NopMessenger;
impl Messenger for NopMessenger {
    fn on_attack(&self, _field: &Battlefield<Self>, _party: PartyId, _move_name: &str) {}
    fn on_damage(&self, _field: &Battlefield<Self>, _party: PartyId, _amount: u32) {}
    fn on_switch(&self, _field: &Battlefield<Self>, _party: PartyId, _origin: u8, _next: u8) {}
    fn on_effect_applied(&self, _field: &Battlefield<Self>, _party: PartyId, _effect_desc: &str) {}
}

pub(crate) struct TestMessenger;
impl Messenger for TestMessenger {
    fn on_attack(&self, _field: &Battlefield<Self>, party: PartyId, move_name: &str) {
        println!("Party {} attacks with {}!", party, move_name);
    }
    fn on_damage(&self, _field: &Battlefield<Self>, party: PartyId, amount: u32) {
        println!("Party {} damaged by {}!", party, amount);
    }
    fn on_switch(&self, _field: &Battlefield<Self>, party: PartyId, origin: u8, next: u8) {
        println!("Party {}: switch {} to {}", party, origin, next);
    }
    fn on_effect_applied(&self, _field: &Battlefield<Self>, party: PartyId, effect_desc: &str) {
        println!("Party {} got effect: {}!", party, effect_desc);
    }
}
