#[derive(Debug)]
pub struct Hand {
    pub price: f64,      // Entry price of SOL
    pub size_sol: f64,   // Amount of SOL in this hand
    pub locked: bool,    // Whether this hand is locked
}

impl Hand {
    pub fn new(price: f64, size_sol: f64, locked: bool) -> Self {
        Self { price, size_sol, locked }
    }
}

pub struct HandManager {
    pub hands: Vec<Hand>,
    pub free_hands: usize,
    pub batch_size: usize,
}

impl HandManager {
    pub fn new() -> Self {
        Self { 
            hands: Vec::new(), 
            free_hands: 0,
            batch_size: 10,
        }
    }

    pub fn open_hand(&mut self, price: f64, size_sol: f64) {
        let locked = self.should_lock();

        let hand = Hand::new(price, size_sol, locked);
        self.hands.push(hand);

        println!(
            "🖐 Hand opened: {:.6} SOL @ ${:.6} | Locked: {} | Total hands: {}",
            size_sol,
            price,
            locked,
            self.hands.len()
        );

        if self.hands.len() % self.batch_size == 0 {
            println!("📦 Batch ready to sell {} hands!", self.batch_size);
        }
    }

    fn should_lock(&mut self) -> bool {
        if self.free_hands < 2 {
            self.free_hands += 1;
            false
        } else {
            true
        }
    }

    pub fn total_locked(&self) -> usize {
        self.hands.iter().filter(|h| h.locked).count()
    }

    /// 🖨️ Print all hands (for console logging)
    pub fn print_hands(&self) {
        if self.hands.is_empty() {
            println!("  No hands currently open.");
            return;
        }

        println!("  Current hands:");
        for (i, hand) in self.hands.iter().enumerate() {
            println!(
                "    Hand {} → {:.6} SOL @ ${:.6} | Locked: {}",
                i + 1,
                hand.size_sol,
                hand.price,
                hand.locked
            );
        }

        println!("  Total locked hands: {}", self.total_locked());
    }
}
