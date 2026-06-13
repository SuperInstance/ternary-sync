# Ternary Sync — Synchronization Primitives for Z₃ Cyclic Agent Coordination

**Ternary Sync** implements synchronization for populations of ternary agents {-1, 0, +1} using Z₃ cyclic dynamics. Agents are coupled through Kuramoto-style phase interactions, and the system converges (or anti-converges) based on coupling strength. It provides coherence measurement, sync time estimation, and anti-sync state generation.

## Why It Matters

Synchronization is fundamental to distributed systems: agents must align their states to coordinate effectively. The Z₃ structure adds a unique dimension — unlike continuous phase oscillators, ternary agents have exactly three states, and synchronization means consensus on one of them. The coupling strength determines whether agents synchronize (high coupling), oscillate chaotically (low coupling), or achieve anti-synchronization (medium coupling with phase repulsion). Understanding these regimes is critical for fleet tuning: too much coupling causes groupthink (all agents converge to the same state), too little causes incoherence.

## How It Works

### Kuramoto-Style Coupling

Each agent has a phase offset and a ternary state. At each tick:

```
influence_i = K × sin(tick × phase_offset_i)
rotation_i  = influence_i > 0.3 ? +1 : influence_i < -0.3 ? -1 : 0
new_state_i = rotate(state_i, rotation_i)
```

where K is the coupling constant. Then, global coupling pulls agents toward the population average.

### Coherence

Coherence measures how aligned the population is:

```
coherence = max(count(-1), count(0), count(+1)) / N
```

Coherence = 1.0 when all agents agree; 0.33 when perfectly split three ways. O(N) per measurement.

### Sync Time

The `sync_time()` method runs the simulation until coherence exceeds 0.99 (or 1000 ticks timeout). Returns the number of ticks to synchronize. For N agents with coupling K, expected sync time scales as:

```
T_sync ∝ N / (K - Kc)   where Kc is the critical coupling
```

### Anti-Synchronization

`anti_sync()` returns the maximally desynchronized state: agents evenly distributed across {-1, 0, +1}. This is the maximum-entropy state and the starting point for synchronization experiments.

### Rotation in Z₃

`rotate(state, offset)`: shifts a ternary state by a continuous offset, discretized to {-1, 0, +1}. Positive offset advances toward +1, negative toward -1.

## Quick Start

```rust
use ternary_sync::SyncGroup;

let mut group = SyncGroup::new(50, 0.5); // 50 agents, coupling 0.5

// Run until synchronized
let sync_ticks = group.sync_time();
println!("Synchronized in {} ticks", sync_ticks.unwrap_or(0));

// Check coherence
let c = group.coherence();
println!("Coherence: {:.2}", c);

// Get anti-synchronized state
let desync = group.anti_sync();
```

```bash
cargo add ternary-sync
```

## API

| Type / Function | Description |
|---|---|
| `SyncGroup` | `{ agents, coupling, phase_offsets }` |
| `SyncGroup::step(tick)` | One tick of Kuramoto-style dynamics |
| `coherence()` | Maximum state fraction (O(N)) |
| `sync_time()` | Ticks to reach coherence > 0.99 |
| `anti_sync()` | Maximally distributed state |

## Architecture Notes

Sync primitives coordinate **SuperInstance** agent populations. The γ + η = C conservation manifests in the coherence-diversity trade-off: high coherence (high γ = aligned growth) means low diversity (low η), and vice versa. Optimal fleet performance requires partial synchronization — enough coherence for coordination, enough diversity for adaptation. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Kuramoto, Yoshiki. *Chemical Oscillations, Waves, and Turbulence*, Springer, 1984 — coupled oscillator model.
- Strogatz, Steven. *Sync: The Emerging Science of Spontaneous Order*, Hyperion, 2003.
| Acebrón, Juan et al. "The Kuramoto Model: A Simple Paradigm for Synchronization Phenomena," *Rev. Mod. Phys.*, 77, 2005.

## License

MIT
