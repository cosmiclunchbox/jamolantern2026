use macroquad::prelude::*;

pub trait MayHaveHitbox {
    fn get_hitbox(&self) -> Option<Rect>;
    fn is_touching_hitbox<T: MayHaveHitbox>(&self, other: &T) -> bool {
        let Some(own_hitbox) = self.get_hitbox() else {
            return false;
        };
        let Some(other_hitbox) = other.get_hitbox() else {
            return false;
        };
        own_hitbox.overlaps(&other_hitbox)
    }
}