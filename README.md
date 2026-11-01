# Dependency Graph

A Rust style exercise: it validates CSR (*conflict serializable*) schedules and determines whether they are serializable by building a dependency graph and looking for cycles in it.

A schedule is CSR if and only if its dependency graph contains no cycles.

## How it works

1. **Schedule parsing.** Operations are separated by commas. Each one consists of an action (`r` for read, `w` for write) and a transaction id. For example, `w0,r1` is a write by transaction 0 followed by a read by transaction 1.
2. **Graph construction.** Each transaction is a node. Two operations from different transactions conflict if at least one of them is a write, in which case an edge is added from the earlier operation's transaction to the later one's.
3. **Cycle detection.** A depth-first search stops at the first cycle it finds, which is enough to conclude that the schedule is not CSR.
4. **Printing.** The graph is drawn in the terminal as an arc diagram, with the cycle highlighted in red.

The format doesn't specify which resource an operation accesses, so all operations are treated as accessing the same resource.

## Usage

The schedule is hardcoded in `src/main.rs`:

```rust
DependencyGraph::build("w0,w1,w2,w3,w0")
```

```sh
cargo run
cargo test
```

For this schedule, the output is:

```text
   ┌───────────────────────────────────────┐
   │             ┌───────────────────────┐ │
   │ ┌───────────┼───────────┐           │ │
   │ │ ┌───────┐ │ ┌───────┐ │ ┌───────┐ │ │
 ┌─┴─┴─┴─┐   ┌─▼─┴─┴─┐   ┌─▼─▼─┴─┐   ┌─▼─▼─▼─┐
 │   0   │   │   1   │   │   2   │   │   3   │
 └─▲─▲─▲─┘   └───┬───┘   └───┬───┘   └───┬───┘
   │ │ └─────────┘           │           │
   │ └───────────────────────┘           │
   └─────────────────────────────────────┘

▼ ▲ edge target   red cycle edge
Cycles found:
0 -> 1 -> 2 -> 3 -> 0
```

## Planned optimizations

- **Cycle detection algorithm.** The current search can be optimized: for example, it looks up each node in the current path with a linear scan and keeps the visit state inside the graph.
- **Node id and index.** A transaction's id is used as an index, but nodes receive sequential ids in creation order. As a result, ids must appear in ascending order with no gaps: `w1,w0` currently panics, and `w0,w2` labels transaction 2 as node 1 and loses the edge between them.

## Future work

I will update the repository in the future to follow a more idiomatic Rust structure.

## Credits

The console graph drawing (`src/printer.rs`) was written by [Claude](https://claude.com/claude-code), based on my graph functions.
