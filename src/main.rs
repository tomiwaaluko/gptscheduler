use std::collections::VecDeque;
use std::env;
use std::fs;

struct Process {
    name: String,
    arrival: usize,
    burst: usize,
    remaining: usize,
    first_run: Option<usize>,
    finish_time: Option<usize>,
    input_order: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: scheduler <input-file>");
        std::process::exit(1);
    }

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
                });
            }
            "end" => break,
            _ => {}
        }
    }

    // Header
    println!("{:>3} processes", process_count);
    match algorithm.as_str() {
        "fcfs" => println!("Using First-Come First-Served"),
        "sjf" => println!("Using preemptive Shortest Job First"),
        "rr" => {
            println!("Using Round-Robin");
            println!("Quantum {:>3}", quantum);
            println!();
        }
        _ => {}
    }

    // Simulate
    match algorithm.as_str() {
        "fcfs" => simulate_fcfs(&mut processes, run_for),
        "sjf" => simulate_sjf(&mut processes, run_for),
        "rr" => simulate_rr(&mut processes, run_for, quantum),
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
}

fn simulate_fcfs(processes: &mut [Process], run_for: usize) {
    let mut ready: VecDeque<usize> = VecDeque::new();
    let mut current: Option<usize> = None;
    let mut current_end: usize = 0;

    for t in 0..run_for {
        // Arrivals
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                println!("Time {:>3} : {} arrived", t, processes[i].name);
                ready.push_back(i);
            }
        }

        // Finish check
        if let Some(idx) = current {
            if t == current_end {
                println!("Time {:>3} : {} finished", t, processes[idx].name);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        // Select if idle
        if current.is_none() {
            if let Some(idx) = ready.pop_front() {
                if processes[idx].first_run.is_none() {
                    processes[idx].first_run = Some(t);
                }
                println!(
                    "Time {:>3} : {} selected (burst {:>3})",
                    t, processes[idx].name, processes[idx].remaining
                );
                current_end = t + processes[idx].remaining;
                current = Some(idx);
            } else {
                println!("Time {:>3} : Idle", t);
            }
        }
    }

    println!("Finished at time {:>3}", run_for);
}

fn simulate_sjf(processes: &mut [Process], run_for: usize) {
    let mut ready: Vec<usize> = Vec::new();
    let mut current: Option<usize> = None;

    for t in 0..run_for {
        // Arrivals
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                println!("Time {:>3} : {} arrived", t, processes[i].name);
                ready.push(i);
            }
        }

        // Finish check
        if let Some(idx) = current {
            if processes[idx].remaining == 0 {
                println!("Time {:>3} : {} finished", t, processes[idx].name);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        // Select best: shortest remaining, then input order
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
                println!(
                    "Time {:>3} : {} selected (burst {:>3})",
                    t, processes[best_idx].name, processes[best_idx].remaining
                );
            }
        } else if current.is_none() {
            println!("Time {:>3} : Idle", t);
        }

        // Decrement running process
        if let Some(idx) = current {
            processes[idx].remaining -= 1;
        }
    }

    println!("Finished at time {:>3}", run_for);
}

fn simulate_rr(processes: &mut [Process], run_for: usize, quantum: usize) {
    let mut ready: VecDeque<usize> = VecDeque::new();
    let mut current: Option<usize> = None;
    let mut quantum_rem: usize = 0;

    for t in 0..run_for {
        // Arrivals
        for i in 0..processes.len() {
            if processes[i].arrival == t {
                println!("Time {:>3} : {} arrived", t, processes[i].name);
                ready.push_back(i);
            }
        }

        // Finish check
        if let Some(idx) = current {
            if processes[idx].remaining == 0 {
                println!("Time {:>3} : {} finished", t, processes[idx].name);
                processes[idx].finish_time = Some(t);
                current = None;
            }
        }

        // Quantum expiry
        if let Some(idx) = current {
            if quantum_rem == 0 {
                ready.push_back(idx);
                current = None;
            }
        }

        // Select next
        if current.is_none() {
            if let Some(idx) = ready.pop_front() {
                current = Some(idx);
                quantum_rem = quantum;
                if processes[idx].first_run.is_none() {
                    processes[idx].first_run = Some(t);
                }
                println!(
                    "Time {:>3} : {} selected (burst {:>3})",
                    t, processes[idx].name, processes[idx].remaining
                );
            } else {
                println!("Time {:>3} : Idle", t);
            }
        }

        // Decrement
        if let Some(idx) = current {
            processes[idx].remaining -= 1;
            quantum_rem -= 1;
        }
    }

    println!("Finished at time {:>3}", run_for);
}
