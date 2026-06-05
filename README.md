# ternary-sync

**Z₃ synchronization for ternary agents. The only sync that works.**

Ternary systems can't synchronize the way binary systems do. Kuramoto fails. Consensus fails. Flocking fails. The 0 state screens everything. So what *does* work?

**Z₃ cyclic rotation.** Instead of trying to make everyone agree on the same value, agents cycle through all three states in a fixed rotation: -1 → 0 → +1 → -1 → ... This is the Z₃ group — the cyclic group of order 3. Each agent has a phase offset. When phase offsets align, agents are "in sync" — not because they're at the same value, but because they're at the *same phase* of the cycle.

The coupling strength determines how quickly agents align their phase offsets. At low coupling, agents drift independently. At high coupling, they lock into the Z₃ rotation together.

## What's Inside

- **`SyncGroup`** — a group of ternary agents with coupling strength and phase offsets
- **`step(tick)`** — advance one tick. Each agent rotates based on coupling and phase
- **`rotate(value, amount)`** — Z₃ rotation: cycle through {-1, 0, +1}
- **`sync_order(agents)`** — how synchronized is the group? Based on phase alignment, not value alignment
- **`consensus(agents)`** — is there a majority value? (Usually no — that's the point)

## Quick Example

```rust
use ternary_sync::*;

// 10 agents with moderate coupling
let mut group = SyncGroup::new(10, 0.5);

// Run 100 ticks
for tick in 0..100 {
    let states = group.step(tick);
    // States rotate through {-1, 0, +1} but phase-align over time
}

// Check sync order
let order = sync_order(&group.agents);
// High order = phases aligned, low = phases scattered

// Z₃ rotation
assert_eq!(rotate(-1, 1), 0);   // -1 → 0
assert_eq!(rotate(0, 1), 1);    // 0 → +1
assert_eq!(rotate(1, 1), -1);   // +1 → -1 (wraps)
```

## The Deeper Truth

**Z₃ is the ONLY algebraic structure that works on ternary.** The spiral experiments proved this exhaustively: of all 19,683 possible binary operations on a 3-element set, only 3 form groups — and all three are Z₃ (just with different identity elements). This means cyclic rotation is *the* canonical coordination mechanism for ternary systems. It's not a design choice — it's a mathematical necessity.

The practical consequence: don't try to make ternary agents agree on a value. Make them agree on a *phase*. The value will cycle — that's fine. What matters is that the cycle is synchronized: everyone rotates together. This is how Z₃ synchronization achieves coordination without consensus.

**Use cases:**
- **Multi-agent coordination** — the only synchronization that works for ternary agents
- **Load balancing** — cycle agents through states fairly
- **Round-robin scheduling** — Z₃ rotation as a scheduling primitive
- **Music synchronization** — beat alignment in ternary rhythm systems
- **Distributed consensus** — when binary consensus fails, use Z₃ phase alignment

## See Also

- **ternary-kuramoto** — the proof that conventional synchronization fails
- **ternary-phase** — phase relationships between synchronized agents
- **ternary-rhythm** — rhythm patterns built on Z₃ timing
- **ternary-speculate** — speculative coordination when sync isn't available

## Install

```bash
cargo add ternary-sync
```

## License

MIT
