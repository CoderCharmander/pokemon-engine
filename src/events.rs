use crate::party::RelativePartyId;

pub enum Event {
    Damaged {
        party_id: u8,
        damage_amount: u32,
    },
    Attacked {
        offending_party_id: u8,
        move_name: String,
    },
    Switched {
        party_id: u8,
        old_index: u8,
        new_index: u8,
    },
}

pub enum MoveEvent {
    Damaged {
        rel_party_id: RelativePartyId,
        damage_amount: u32,
    },
    Effected {
        rel_party_id: RelativePartyId,
        description: String,
    },
}
