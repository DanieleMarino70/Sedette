//! Attese crescenti tra i tentativi di riconnessione.

use std::time::Duration;

/// Backoff esponenziale: ogni attesa è il doppio della precedente, fino a un
/// massimo. Dopo una connessione riuscita si torna all'attesa iniziale.
#[derive(Debug, Clone)]
pub struct Backoff {
    initial: Duration,
    max: Duration,
    next: Duration,
}

impl Backoff {
    /// Crea il backoff. Se `max` è minore di `initial`, si usa `initial` per entrambi.
    pub fn new(initial: Duration, max: Duration) -> Self {
        let max = max.max(initial);
        Self {
            initial,
            max,
            next: initial,
        }
    }

    /// Attesa da usare ora; la successiva raddoppia, fino al massimo.
    pub fn next_delay(&mut self) -> Duration {
        let current = self.next;
        self.next = current.saturating_mul(2).min(self.max);
        current
    }

    /// Torna all'attesa iniziale.
    pub fn reset(&mut self) {
        self.next = self.initial;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raddoppia_fino_al_massimo_e_si_azzera() {
        let mut b = Backoff::new(Duration::from_millis(500), Duration::from_secs(3));
        let delays: Vec<_> = (0..5).map(|_| b.next_delay().as_millis()).collect();
        assert_eq!(delays, [500, 1000, 2000, 3000, 3000]);
        b.reset();
        assert_eq!(b.next_delay(), Duration::from_millis(500));
    }

    #[test]
    fn massimo_minore_dell_iniziale() {
        let mut b = Backoff::new(Duration::from_secs(5), Duration::from_secs(1));
        assert_eq!(b.next_delay(), Duration::from_secs(5));
        assert_eq!(b.next_delay(), Duration::from_secs(5));
    }
}
