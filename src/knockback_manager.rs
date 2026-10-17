use time::{SignedDuration, Timestamp};

use crate::{constants::KNOCKBACK_DURATION_MS, util::MoveDirection};

pub struct Knockback {
    pub direction: MoveDirection,
    pub amount: f32,
    expires_at: Timestamp
}

impl Knockback {
    pub fn new(direction: MoveDirection, amount: f32) -> Knockback {
        let expires_at: Timestamp = Timestamp::now().checked_add(
            SignedDuration::milliseconds(KNOCKBACK_DURATION_MS as i64)
        ).expect("gonna just expect this checked add should work ig");
        Knockback {
            direction,
            amount,
            expires_at
        }
    }

    pub fn being_knocked_back(&self) -> bool {
        Timestamp::now() <= self.expires_at
    }
}