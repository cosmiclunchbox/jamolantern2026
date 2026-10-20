use time::{SignedDuration, Timestamp};

use crate::{constants::{KNOCKBACK_ACCELERATION_DURATION_MS}, util::MoveDirection};

#[derive(Clone)]
pub struct Knockback {
    pub direction: MoveDirection,
    pub amount: f32,
    initial_acceleration_expires_at: Timestamp,
    expires_at: Timestamp
}

impl Knockback {
    pub fn new(direction: MoveDirection, amount: f32, duration: i32) -> Knockback {
        let initial_acceleration_expires_at: Timestamp = Timestamp::now().checked_add(
            SignedDuration::milliseconds(KNOCKBACK_ACCELERATION_DURATION_MS as i64)
        ).expect("gonna just expect this checked add should work ig");
        let expires_at: Timestamp = Timestamp::now().checked_add(
            SignedDuration::milliseconds(duration as i64)
        ).expect("gonna just expect this checked add should work ig");
        Knockback {
            direction,
            amount,
            initial_acceleration_expires_at,
            expires_at
        }
    }

    pub fn being_knocked_back(&self) -> bool {
        Timestamp::now() <= self.expires_at
    }

    pub fn being_accelerated_backwards(&self) -> bool {
        Timestamp::now() <= self.initial_acceleration_expires_at
    }
}