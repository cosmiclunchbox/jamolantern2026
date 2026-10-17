use time::{SignedDuration, Timestamp};

pub struct AttackManager {
    cooldown_length_ms: i32,
    cooldown_expires_at: Option<Timestamp> 
}

impl AttackManager {
    pub fn new(cooldown_length_ms: i32) -> AttackManager {
        AttackManager {
            cooldown_length_ms,
            cooldown_expires_at: None
        }
    }

    pub fn start_cooldown(&mut self) {
        let cooldown_expires_at = Timestamp::now().checked_add(
            SignedDuration::milliseconds(self.cooldown_length_ms as i64)
        ).expect("I'm gonna just assume the timestamp add should work lol");
        self.cooldown_expires_at = Some(cooldown_expires_at);
    }

    pub fn on_cooldown(&self) -> bool {
        let Some(cooldown_expires_at) = self.cooldown_expires_at else {
            return false;
        };
        Timestamp::now() <= cooldown_expires_at

    }
}