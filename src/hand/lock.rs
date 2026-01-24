use crate::hand::hand::Hand;

pub struct LockRules;

impl LockRules {
    pub fn is_locked(hand: &Hand, price: f64) -> bool {
        hand.locked && price < hand.price + 0.2
    }

    pub fn unlock_batch(hands: &mut Vec<Hand>, price: f64) -> Vec<&mut Hand> {
        let mut unlocked = Vec::new();

        for hand in hands.iter_mut() {
            if hand.locked && price >= hand.price + 0.5 {
                hand.locked = false;
                println!(
                    "🔓 Hand unlocked: {:.6} SOL @ ${:.6}",
                    hand.size_sol, hand.price
                );
                unlocked.push(hand);
            }
        }

        unlocked
    }
}
