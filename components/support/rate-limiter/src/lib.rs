/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Simple token bucket rate-limiter.

#[derive(Clone, Copy)]
pub struct RateLimiter {
    capacity: u8,
    tokens: u8,
    renewal_rate: f32, // per ms.
    last_refill: u64,  // in ms.
}

impl RateLimiter {
    pub fn new(capacity: u8, renewal_rate: f32) -> Self {
        Self {
            capacity,
            tokens: capacity,
            renewal_rate,
            last_refill: Self::now(),
        }
    }

    pub fn check(&mut self) -> bool {
        self.refill();
        if self.tokens == 0 {
            return false;
        }
        self.tokens -= 1;
        true
    }

    fn refill(&mut self) {
        let now = Self::now();
        let new_tokens = ((now - self.last_refill) as f64 * self.renewal_rate as f64) as u8; // `as` is a truncating/saturing cast.
        if new_tokens > 0 {
            self.last_refill = now;
            self.tokens = std::cmp::min(self.capacity, self.tokens.saturating_add(new_tokens));
        }
    }

    #[cfg(not(test))]
    #[inline]
    fn now() -> u64 {
        // util::now()
        1600000000000
    }

    #[cfg(test)]
    fn now() -> u64 {
        1600000000000
    }
}

#[cfg(test)]
mod tests {
    use crate::RateLimiter;
    #[test]
    fn test_auth_circuit_breaker_unit_recovery() {
        let capacity = 10;
        let renewal_rate = 1.0 / 1000.0;
        let mut breaker = RateLimiter::new(capacity, renewal_rate);
        // AuthCircuitBreaker::now is fixed for tests, let's assert that for sanity.
        assert_eq!(RateLimiter::now(), 1600000000000);
        for _ in 0..capacity {
            assert!(breaker.check());
        }
        assert!(!breaker.check());
        // Jump back in time (1 min).
        breaker.last_refill -= 60 * 1000;
        let expected_tokens_before_check: u8 = (renewal_rate * 60.0 * 1000.0) as u8;
        assert!(breaker.check());
        assert_eq!(breaker.tokens, expected_tokens_before_check - 1);
    }
}
