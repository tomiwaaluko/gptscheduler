use std::collections::VecDeque;
use std::env;
use std::fs;

// ANSI color codes
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const MAGENTA: &str = "\x1b[35m";
const DIM: &str = "\x1b[2m";

struct Process {
    name: String,
    arrival: usize,
    burst: usize,
    remaining: usize,
    first_run: Option<usize>,
    finish_time: Option<usize>,
    input_order: usize,
    vruntime: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: scheduler <input-file> [--enhanced]");
        std::process::exit(1);
    }

    let enhanced = args.iter().any(|a| a == "--enhanced");
    let contents = fs::read_to_string(&args[1]).expect("Failed to read input file");

    let mut process_count = 0;
    let mut run_for = 0;
    let mut algorithm = String::new();
    let mut quantum = 0;
    let mut processes: Vec<Process> = Vec::new();

    for line in contents.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "processcount" => process_count = tokens[1].parse().unwrap(),
            "runfor" => run_for = tokens[1].parse().unwrap(),
            "use" => algorithm = tokens[1].to_string(),
            "quantum" => quantum = tokens[1].parse().unwrap(),
            "process" => {
                let order = processes.len();
                processes.push(Process {
                    name: tokens[2].to_string(),
                    arrival: tokens[4].parse().unwrap(),
                    burst: tokens[6].parse().unwrap(),
                    remaining: tokens[6].parse().unwrap(),
                    first_run: None,
                    finish_time: None,
                    input_order: order,
                    vruntime: 0.0,
                });
            }
            "end" => break,
            _ => {}
        }
    }

    // CFS always runs enhanced (no expected output to match)
    let color = enhanced || algorithm == "cfs";

    // Header
    if color {
        print!("{BOLD}");
    }
    println!("{:>3} processes", process_count);
    match algorithm.as_str() {
        "fcfs" => println!("Using First-Come First-Served"),
        "sjf" => println!("Using preemptive Shortest Job First"),
        "rr" => {
            println!("Using Round-Robin");
            println!("Quantum {:>3}", quantum);
            if color {
                print!("{RESET}");
            }
            println!();
        }
        "cfs" => println!("Using Completely Fair Scheduler"),
        _ => {}
    }
    if color && algorithm != "rr" {
        print!("{RESET}");
    }

    // Simulate
    match algorithm.as_str() {
        "fcfs" => simulate_fcfs(&mut processes, run_for, color),
        "sjf" => simulate_sjf(&mut processes, run_for, color),
        "rr" => simulate_rr(&mut processes, run_for, quantum, color),
        "cfs" => simulate_cfs(&mut processes, run_for, color),
        _ => {}
    }

    // Statistics
    println!();
    processes.sort_by_key(|p| p.input_order);
    for p in &processes {
        let finish = p.finish_time.unwrap();
        let turnaround = finish - p.arrival;
        let wait = turnaround - p.burst;
        let response = p.first_run.unwrap() - p.arrival;
        println!(
            "{} wait {:>3} turnaround {:>3} response {:>3}",
            p.name, wait, turnaround, response
        );
    }

    // Enhanced summary table
    if color {
        print_summary_table(&processes);
    }
}

// ── Color helpers ────────────────────────────────────────────────────────────

fn print_arrived(t: usize, name: &str, color: bool) {
    if color {
        println!("{GREEN}Time {:>3} : {} arrived{RESET}", t, name);
    } else {
        println!("Time {:>3} : {} arrived", t, name);
    }
}

fn print_selected(t: usize, name: &str, burst: usize, color: bool) {
    if color {
        println!(
            "{CYAN}Time {:>3} : {} selected (burst {:>3}){RESET}",
            t, name, burst
        );
    } else {
        println!("Time {:>3} : {} selected (burst {:>3})", t, name, burst);
    }
}

fn print_finished(t: usize, name: &str, color: bool) {
    if color {
        println!("{YELLOW}Time {:>3} : {} finished{RESET}", t, name);
    } else {
        println!("Time {:>3} : {} finished", t, name);
    }
}

fn print_idle(t: usize, color: bool) {
    if color {
        println!("{DIM}{RED}Time {:>3} : Idle{RESET}", t);
    } else {
        println!("Time {:>3} : Idle", t);
    }
}

fn print_finish_line(run_for: usize, color: bool) {
    if color {
        println!("{BOLD}Finished at time {:>3}{RESET}", run_for);
    } else {
        println!("Finished at time {:>3}", run_for);
    }
}

// ── Summary table ────────────────────────────────────────────────────────────

fn print_summary_table(processes: &[Process]) {
    let max_name = processes.iter().map(|p| p.name.len()).max().unwrap_or(4).max(7);

    println!();
    println!("{BOLD}{MAGENTA}╔═{:═<w$}═╦════════╦══════════════╦════════════╗{RESET}", "", w = max_name);
    println!(
        "{BOLD}{MAGENTA}║ {:^w$} ║  Wait  ║  Turnaround  ║  Response  ║{RESET}",
        "Process",
        w = max_name
    );
    println!("{MAGENTA}╠═{:═<w$}═╬════════╬══════════════╬════════════╣{RESET}", "", w = max_name);

    let mut total_wait: f64 = 0.0;
    let mut total_turn: f64 = 0.0;
    let mut total_resp: f64 = 0.0;
    let n = processes.len() as f64;

    for p in processes {
        let finish = p.finish_time.unwrap();
        let turnaround = finish - p.arrival;
        let wait = turnaround - p.burst;
        let response = p.first_run.unwrap() - p.arrival;
        total_wait += wait as f64;
        total_turn += turnaround as f64;
        total_resp += response as f64;
        println!(
            "{MAGENTA}║{RESET} {:<w$} {MAGENTA}║{RESET} {:>6} {MAGENTA}║{RESET} {:>12} {MAGENTA}║{RESET} {:>10} {MAGENTA}║{RESET}",
            p.name, wait, turnaround, response,
            w = max_name
        );
    }

    println!("{MAGENTA}╠═{:═<w$}═╬════════╬══════════════╬════════════╣{RESET}", "", w = max_name);
    println!(
        "{BOLD}{MAGENTA}║{RESET}{BOLD} {:^w$} {MAGENTA}║{RESET}{BOLD} {:>6.2} {MAGENTA}║{RESET}{BOLD} {:>12.2} {MAGENTA}║{RESET}{BOLD} {:>10.2} {MAGENTA}║{RESET}",
        "Average",
        total_wait / n,
        total_turn / n,
        total_resp / n,
        w = max_name
    );
    println!("{MAGENTA}╚═{:═<w$}═╩════════╩══════════════╩════════════╝{RESET}", "", w = max_name);
}

// ── FCFS ─────────────────────────────────────────────────────────────────────

fn simulate_fcfs(processes: &mut [Process], run_for: usize, color: bool) {
    let mut ready: VecDeque<usize> = VecDeque::new();
    let mut current: Option<usize> = None;
    let mut current_end: usize = 0;

    for t in 0..run_for {
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                print_arrived(t, &processes[i].name, color);
                ready.push_back(i);
            }
        }

        if let Some(idx) = current {
            if t == current_end {
                print_finished(t, &processes[idx].name, color);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        if current.is_none() {
            if let Some(idx) = ready.pop_front() {
                if processes[idx].first_run.is_none() {
                    processes[idx].first_run = Some(t);
                }
                print_selected(t, &processes[idx].name, processes[idx].remaining, color);
                current_end = t + processes[idx].remaining;
                current = Some(idx);
            } else {
                print_idle(t, color);
            }
        }
    }

    print_finish_line(run_for, color);
}

// ── SJF (preemptive) ────────────────────────────────────────────────────────

fn simulate_sjf(processes: &mut [Process], run_for: usize, color: bool) {
    let mut ready: Vec<usize> = Vec::new();
    let mut current: Option<usize> = None;

    for t in 0..run_for {
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                print_arrived(t, &processes[i].name, color);
                ready.push(i);
            }
        }

        if let Some(idx) = current {
            if processes[idx].remaining == 0 {
                print_finished(t, &processes[idx].name, color);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        let mut best: Option<usize> = current;
        for &idx in &ready {
            match best {
                Some(b)
                    if processes[idx].remaining < processes[b].remaining
                        || (processes[idx].remaining == processes[b].remaining
                            && processes[idx].input_order < processes[b].input_order) =>
                {
                    best = Some(idx);
                }
                None => best = Some(idx),
                _ => {}
            }
        }

        if let Some(best_idx) = best {
            if current != Some(best_idx) {
                if let Some(old) = current {
                    ready.push(old);
                }
                ready.retain(|&x| x != best_idx);
                current = Some(best_idx);
                if processes[best_idx].first_run.is_none() {
                    processes[best_idx].first_run = Some(t);
                }
                print_selected(
                    t,
                    &processes[best_idx].name,
                    processes[best_idx].remaining,
                    color,
                );
            }
        } else if current.is_none() {
            print_idle(t, color);
        }

        if let Some(idx) = current {
            processes[idx].remaining -= 1;
        }
    }

    print_finish_line(run_for, color);
}

// ── Round-Robin ──────────────────────────────────────────────────────────────

fn simulate_rr(processes: &mut [Process], run_for: usize, quantum: usize, color: bool) {
    let mut ready: VecDeque<usize> = VecDeque::new();
    let mut current: Option<usize> = None;
    let mut quantum_rem: usize = 0;

    for t in 0..run_for {
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                print_arrived(t, &processes[i].name, color);
                ready.push_back(i);
            }
        }

        if let Some(idx) = current {
            if processes[idx].remaining == 0 {
                print_finished(t, &processes[idx].name, color);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        if let Some(idx) = current {
            if quantum_rem == 0 {
                ready.push_back(idx);
                current = None;
            }
        }

        if current.is_none() {
            if let Some(idx) = ready.pop_front() {
                current = Some(idx);
                quantum_rem = quantum;
                if processes[idx].first_run.is_none() {
                    processes[idx].first_run = Some(t);
                }
                print_selected(t, &processes[idx].name, processes[idx].remaining, color);
            } else {
                print_idle(t, color);
            }
        }

        if let Some(idx) = current {
            processes[idx].remaining -= 1;
            quantum_rem -= 1;
        }
    }

    print_finish_line(run_for, color);
}

// ── CFS (Completely Fair Scheduler) ──────────────────────────────────────────
//
// Simplified Linux CFS:
// - Each process tracks a virtual runtime (vruntime) that increases as it runs.
// - The process with the lowest vruntime is always selected (fairness).
// - New arrivals inherit the current minimum vruntime so they don't starve or
//   get an unfair head start.
// - Ties broken by input order.

fn simulate_cfs(processes: &mut [Process], run_for: usize, color: bool) {
    let mut ready: Vec<usize> = Vec::new();
    let mut current: Option<usize> = None;
    let mut min_vruntime: f64 = 0.0;

    for t in 0..run_for {
        // Arrivals — new processes inherit current min_vruntime
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                processes[i].vruntime = min_vruntime;
                print_arrived(t, &processes[i].name, color);
                ready.push(i);
            }
        }

        // Finish check
        if let Some(idx) = current {
            if processes[idx].remaining == 0 {
                print_finished(t, &processes[idx].name, color);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        // Select process with lowest vruntime (ties: input order)
        let mut best: Option<usize> = current;
        for &idx in &ready {
            match best {
                Some(b)
                    if processes[idx].vruntime < processes[b].vruntime
                        || (processes[idx].vruntime == processes[b].vruntime
                            && processes[idx].input_order < processes[b].input_order) =>
                {
                    best = Some(idx);
                }
                None => best = Some(idx),
                _ => {}
            }
        }

        if let Some(best_idx) = best {
            if current != Some(best_idx) {
                if let Some(old) = current {
                    ready.push(old);
                }
                ready.retain(|&x| x != best_idx);
                current = Some(best_idx);
                if processes[best_idx].first_run.is_none() {
                    processes[best_idx].first_run = Some(t);
                }
                print_selected(
                    t,
                    &processes[best_idx].name,
                    processes[best_idx].remaining,
                    color,
                );
            }
        } else if current.is_none() {
            print_idle(t, color);
        }

        // Tick: running process accumulates vruntime
        if let Some(idx) = current {
            processes[idx].remaining -= 1;
            processes[idx].vruntime += 1.0;

            // Update global min_vruntime
            min_vruntime = processes[idx].vruntime;
            for &r in &ready {
                if processes[r].vruntime < min_vruntime {
                    min_vruntime = processes[r].vruntime;
                }
            }
        }
    }

    print_finish_line(run_for, color);
}
