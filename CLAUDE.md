# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a CPU Process Scheduling Simulator (Programming Assignment 1). The goal is to implement a simulator that reads process definitions from an input file and simulates three scheduling algorithms: FCFS, SJF (preemptive), and Round-Robin.

## Build & Run

**Environment setup (required — adds rustup GNU toolchain and MinGW GCC to PATH):**
```bash
export PATH="/c/Users/gokug/.cargo/bin:/c/Users/gokug/scoop/apps/mingw/current/bin:$PATH"
```

```bash
cargo build                                          # debug build
cargo build --release                                # release build
./target/debug/scheduler pa1-testfiles/c2-fcfs.in   # run
```

## Testing

Use the `/run-tests` skill to run all 9 test cases, or `/run-test c2-fcfs` for a single case.

To validate manually:
```bash
./target/debug/scheduler pa1-testfiles/c2-fcfs.in > /tmp/out.txt
diff /tmp/out.txt pa1-testfiles/c2-fcfs.out
```

Test files are in `pa1-testfiles/`. Each algorithm has test suites for 2, 5, and 10 processes:
- `c2-fcfs.in` / `c2-fcfs.out`
- `c2-sjf.in` / `c2-sjf.out`
- `c2-rr.in` / `c2-rr.out`
- (same pattern for `c5-*` and `c10-*`)

## Input/Output Format

**Input file format:**
```
processcount N
runfor X
use [fcfs|sjf|rr]
quantum X        # required only for rr
process name <name> arrival <A> burst <B>
...
end
```

**Output format:**
```
  N processes
Using [First-Come First-Served | preemptive Shortest Job First | Round-Robin]
[Quantum   X]    # only for rr

Time   0 : <name> arrived
Time   0 : <name> selected (burst   B)
Time   X : <name> finished
Time   X : Idle
...
Finished at time  X

<name> wait   W turnaround   T response   R
```

Output must match expected files **exactly** (including spacing and formatting).

## Scheduling Algorithms

- **FCFS**: Non-preemptive FIFO. On tie in arrival time, maintain input order.
- **SJF**: Preemptive shortest-job-first (shortest remaining burst). On tie, maintain input order.
- **RR**: Round-Robin with configurable time quantum. Processes cycle through a ready queue.

## Key Metrics

- **Wait time**: total time spent waiting (not running)
- **Turnaround time**: completion time − arrival time
- **Response time**: time of first CPU selection − arrival time
