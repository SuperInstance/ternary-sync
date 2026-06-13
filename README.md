# ternary-sync

Synchronization primitives for ternary agents using **Z₃ cyclic rotation**. Models multi-agent phase synchronization where each agent's state ∈ {-1, 0, +1} and the goal is to measure and drive the system toward coherent or anti-coherent configurations.

## Why It Matters

Coordinating multiple agents (e.g., GPU kernels, distributed workers, oscillator arrays) requires knowing how synchronized they are and how long convergence takes. Binary synchronization models (synced/unsynced) lose the intermediate state. Ternary synchronization captures three-way phase alignment:

| State | Value | Phase |
|-------|-------|-------|
| Leading | `+1` | Ahead of consensus |
| Aligned | `0` | At consensus |
| Trailing | `-1` | Behind consensus |

The Z₃ rotation operator `rotate(state, offset)` cyclically shifts state: -1 → 0 → +1 → -1. This is isomorphic to addition mod 3 on {0, 1, 2}, making it algebraically clean for analysis.

## How It Works

### Z₃ Cyclic Rotation

Each agent's state is updated by a rotation influenced by coupling:

```
influence_i = coupling · sin(tick · phase_offset_i)
rot_i = +1 if influence > 0.3, -1 if < -0.3, else 0
state_i = rotate(state_i, rot_i)
```

where `rotate(state, offset)` performs cyclic permutation:

```
rotate(-1, +1) = 0
rotate( 0, +1) = +1
rotate(+1, +1) = -1   (wraps around)
rotate(state, 0) = state
```

This is addition modulo 3 on the mapped domain {-1, 0, +1} → {0, 1, 2}.

### Coupling and Consensus

After individual rotation, a consensus pull brings agents closer together:

```
avg = Σ stateᵢ / N
if |avg - stateᵢ| > 0.5 and coupling > 0.3:
    stateᵢ += sign(avg - stateᵢ)
    clamp to {-1, 0, +1}
```

This is a **Kuramoto-style** coupling: each agent is pulled toward the mean state proportional to the coupling strength κ.

### Coherence Metric

```
coherence = max(count(s)) / N   for s ∈ {-1, 0, +1}
```

`coherence = 1.0` means all agents are in the same state (perfect sync). `coherence ≈ 1/3` means uniform distribution (maximum diversity).

### Sync Time

The `sync_time()` method simulates up to 1000 ticks and returns the first tick where `coherence > 0.99`:

```
for tick in 0..1000:
    step(tick)
    if coherence() > 0.99:
        return tick
```

Returns `None` if no convergence within 1000 ticks.

**Complexity:** O(N · T) where T = ticks to convergence (or 1000).

### Anti-Synchronization

The `anti_sync()` method produces a maximally desynchronized distribution:

```
state_i = [-1, 0, +1, -1, 0, +1, ...][i mod 3]
```

This evenly distributes agents across the three states, achieving `coherence ≈ ⌈N/3⌉ / N`.

### Phase Offsets

Each agent has a random phase offset ∈ [0, 1) drawn from a seeded PRNG. These offsets determine the agent's natural oscillation frequency, creating diversity that the coupling must overcome.

## Quick Start

```rust
use ternary_sync::SyncGroup;

let mut group = SyncGroup::new(n_agents: 10, coupling: 0.5);

// Simulate 100 ticks
for tick in 0..100 {
    let states = group.step(tick);
}

let c = group.coherence();
assert!(c > 0.0 && c <= 1.0);

// Find time to sync
let mut group2 = SyncGroup::new(10, 0.7);
let sync_time = group2.sync_time();
// Strong coupling → faster convergence
```

## API

| Type | Key Methods |
|------|-------------|
| `SyncGroup` | `new(n, coupling)`, `step(tick)`, `coherence()`, `sync_time()`, `anti_sync()` |
| `rotate(state, offset)` | Free function: Z₃ cyclic rotation |
| `SimpleRng` | Internal PRNG for deterministic initialization |

## Architecture Notes

The **γ + η = C** invariant governs the entire synchronization process. *Generation* (γ) is the rotation operator producing new states. *Entropy* (η) is the coherence loss — when agents diverge, the state distribution spreads across {-1, 0, +1}, increasing Shannon entropy. *Conservation* (C) is the invariant that `Σ states` is preserved modulo the coupling correction — the total "phase momentum" of the system is conserved, and coupling only redistributes it. Strong coupling (high κ) drives η → 0 (coherence) rapidly; weak coupling allows high entropy (diversity). The sync_time metric quantifies the γ-η convergence rate.

## References

- **Kuramoto model:** Kuramoto, Y. *Chemical Oscillations, Waves, and Turbulence* (1984)
- **Z₃ group theory:** Serre, J.-P. *Linear Representations of Finite Groups* (1977)
- **Coupled oscillator synchronization:** Strogatz, S. "From Kuramoto to Crawford" (2000)
- **Phase synchronization in distributed systems:** Baldoni, R. & Raynal, M. "Fundamentals of Distributed Computing" (2017)

## License

MIT
