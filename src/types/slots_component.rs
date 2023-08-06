use super::{Slot, SlotsComponent, Subjects};

impl SlotsComponent {

    pub fn slots(&self) -> Vec<Slot> {
        self.slots.clone()
    }

    pub fn slot(&self, subject: Subjects) -> Option<Slot> {
        for slot in self.slots() {
            if slot.subject() == subject {
                return Some(slot);
            }
        }
        None
    }
}
