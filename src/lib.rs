#![forbid(unsafe_code)]

/// Synchronization primitives for ternary agents using Z3 cyclic rotation.

pub struct SyncGroup {
    pub agents: Vec<i8>,
    pub coupling: f64,
    pub phase_offsets: Vec<f64>,
}

impl SyncGroup {
    pub fn new(n_agents: usize, coupling: f64) -> Self {
        let seed = 42u64;
        let mut rng = SimpleRng::new(seed);
        let agents: Vec<i8> = (0..n_agents).map(|_| (rng.next_f64() * 3.0).floor() as i8 - 1).collect();
        let phase_offsets: Vec<f64> = (0..n_agents).map(|_| rng.next_f64()).collect();
        Self { agents, coupling, phase_offsets }
    }

    pub fn step(&mut self, tick: u64) -> Vec<i8> {
        // Z3 rotation based on coupling and tick
        let n = self.agents.len();
        let mut new_states = Vec::with_capacity(n);
        for i in 0..n {
            let influence = self.coupling * (tick as f64 * self.phase_offsets[i]).sin();
            let rot = if influence > 0.3 { 1 } else if influence < -0.3 { -1 } else { 0 };
            new_states.push(rotate(self.agents[i], rot as f64));
        }
        // coupling: pull toward consensus
        let sum: i64 = new_states.iter().map(|&v| v as i64).sum();
        let avg = sum as f64 / n as f64;
        for i in 0..n {
            let diff = avg - new_states[i] as f64;
            if diff.abs() > 0.5 && self.coupling > 0.3 {
                new_states[i] = new_states[i] + if diff > 0.0 { 1 } else { -1 };
                new_states[i] = new_states[i].clamp(-1, 1);
            }
        }
        self.agents = new_states.clone();
        new_states
    }

    pub fn coherence(&self) -> f64 {
        let n = self.agents.len();
        if n == 0 { return 1.0; }
        let mut counts = [0usize; 3]; // -1, 0, 1
        for &v in &self.agents {
            let idx = (v + 1) as usize;
            if idx < 3 { counts[idx] += 1; }
        }
        let max_count = *counts.iter().max().unwrap();
        max_count as f64 / n as f64
    }

    pub fn sync_time(&mut self) -> Option<u64> {
        let n = self.agents.len();
        if n == 0 { return Some(0); }
        for tick in 0..1000 {
            self.step(tick);
            if self.coherence() > 0.99 {
                return Some(tick + 1);
            }
        }
        None
    }

    pub fn anti_sync(&self) -> Vec<i8> {
        let n = self.agents.len();
        // Maximally desynchronized: spread across -1, 0, 1 evenly
        (0..n).map(|i| {
            match i % 3 {
                0 => -1,
                1 => 0,
                _ => 1,
            }
        }).collect()
    }
}

pub fn rotate(state: i8, offset: f64) -> i8 {
    // Z3 rotation: state + offset mapped to Z3
    let s = state as f64;
    let mut r = (s + offset).round() as i8;
    // Map into {-1, 0, 1} via mod 3
    while r > 1 { r -= 3; }
    while r < -1 { r += 3; }
    r
}

pub fn consensus(votes: &[i8], threshold: f64) -> Option<i8> {
    if votes.is_empty() { return None; }
    let n = votes.len();
    let mut counts = [0usize; 3];
    for &v in votes {
        let idx = (v + 1) as usize;
        if idx < 3 { counts[idx] += 1; }
    }
    let max_count = *counts.iter().max().unwrap();
    if (max_count as f64 / n as f64) >= threshold {
        let max_idx = counts.iter().enumerate().max_by_key(|(_, &c)| c).map(|(i, _)| i).unwrap();
        Some(max_idx as i8 - 1)
    } else {
        None
    }
}

struct SimpleRng { state: u64 }
impl SimpleRng {
    fn new(seed: u64) -> Self { Self { state: if seed == 0 { 1 } else { seed } } }
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sync_group() {
        let g = SyncGroup::new(5, 0.5);
        assert_eq!(g.agents.len(), 5);
        assert_eq!(g.phase_offsets.len(), 5);
    }

    #[test]
    fn test_step_changes() {
        let mut g = SyncGroup::new(4, 0.8);
        let before = g.agents.clone();
        let after = g.step(0);
        assert_eq!(after.len(), 4);
        // Agents may or may not change, but step runs
    }

    #[test]
    fn test_coherence_perfect() {
        let g = SyncGroup { agents: vec![1, 1, 1, 1], coupling: 0.5, phase_offsets: vec![0.1; 4] };
        assert!((g.coherence() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_coherence_mixed() {
        let g = SyncGroup { agents: vec![-1, 0, 1], coupling: 0.5, phase_offsets: vec![0.1; 3] };
        assert!((g.coherence() - 1.0/3.0).abs() < 1e-9);
    }

    #[test]
    fn test_coherence_empty() {
        let g = SyncGroup { agents: vec![], coupling: 0.5, phase_offsets: vec![] };
        assert!((g.coherence() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_rotate_basic() {
        assert_eq!(rotate(0, 1.0), 1);
        assert_eq!(rotate(1, 1.0), -1); // wraps in Z3: 1+1=2 -> mod 3 -> -1
    }

    #[test]
    fn test_rotate_negative() {
        assert_eq!(rotate(0, -1.0), -1);
    }

    #[test]
    fn test_rotate_zero() {
        assert_eq!(rotate(1, 0.0), 1);
    }

    #[test]
    fn test_sync_time_finds() {
        let mut g = SyncGroup { agents: vec![1, 1, 1], coupling: 1.0, phase_offsets: vec![0.0; 3] };
        // Already coherent
        assert!(g.sync_time().is_some());
    }

    #[test]
    fn test_anti_sync() {
        let g = SyncGroup::new(6, 0.5);
        let anti = g.anti_sync();
        assert_eq!(anti.len(), 6);
        // Should have all three values
        assert!(anti.iter().any(|&v| v == -1));
        assert!(anti.iter().any(|&v| v == 0));
        assert!(anti.iter().any(|&v| v == 1));
    }

    #[test]
    fn test_consensus_clear() {
        let result = consensus(&[1, 1, 1, 0], 0.7);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_consensus_unclear() {
        let result = consensus(&[-1, 0, 1], 0.8);
        assert!(result.is_none());
    }

    #[test]
    fn test_consensus_empty() {
        assert!(consensus(&[], 0.5).is_none());
    }

    #[test]
    fn test_consensus_unanimous() {
        assert_eq!(consensus(&[-1, -1, -1], 1.0), Some(-1));
    }

    #[test]
    fn test_sync_group_size() {
        // SyncGroup should be small; agents Vec is heap though
        // Just check it exists
        let g = SyncGroup::new(3, 0.5);
        assert_eq!(g.agents.len(), 3);
    }
}
