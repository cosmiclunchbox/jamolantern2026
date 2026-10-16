use time::{Timestamp, SignedDuration};

use crate::constants::{IFRAME_DURATION_MS, IFRAME_VISUAL_DAMAGE_TICK_LENGTH_MS};

#[derive(Debug)]
pub struct IFrame {
    damage_tick_ends_at: Timestamp,
    expires_at: Timestamp
}

impl IFrame {
    pub fn new() -> IFrame {
        let damage_tick_ends_at = Timestamp::now().checked_add(
            SignedDuration::milliseconds(IFRAME_VISUAL_DAMAGE_TICK_LENGTH_MS as i64)
        ).expect("I'm gonna just assume the timestamp add should work lol");
        let expires_at = Timestamp::now().checked_add(
            SignedDuration::milliseconds(IFRAME_DURATION_MS as i64)
        ).expect("I'm gonna just assume the timestamp add should work lol");
        IFrame {
            damage_tick_ends_at,
            expires_at
        }
    }

    pub fn is_expired(&self) -> bool {
        Timestamp::now() > self.expires_at
    }

    pub fn is_during_damage_tick(&self) -> bool {
        Timestamp::now() <= self.damage_tick_ends_at
    }
}