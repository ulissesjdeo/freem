use std::env;
use sysinfo::System;

fn format_value(value: u64, divisor: u64) -> u64 {
    value / divisor
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let divisor = match args.get(1).map(|s| s.as_str()) {
        Some("-m") => 1024 * 1024,
        Some("-g") => 1024 * 1024 * 1024,
        _ => 1024,
    };

    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_memory = format_value(sys.total_memory(), divisor);
    let used_memory = format_value(sys.used_memory(), divisor);
    let free_memory = total_memory.saturating_sub(used_memory);
    let available_memory = format_value(sys.available_memory(), divisor);
    let buff_cache = available_memory.saturating_sub(free_memory);

    let total_swap = format_value(sys.total_swap(), divisor);
    let used_swap = format_value(sys.used_swap(), divisor);
    let free_swap = total_swap.saturating_sub(used_swap);

    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    println!(
        "{:<15} {}{:>10}{} {}{:>10}{} {}{:>10}{} {}{:>10}{} {}{:>12}{} {}{:>12}{}",
        "", bold, "total", reset, bold, "used", reset, bold, "free", reset, bold, "shared", reset, bold, "buff/cache", reset, bold, "available", reset
    );
    println!(
        "{}{:<15}{} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12}",
        bold, "Mem.:", reset, total_memory, used_memory, free_memory, 0, buff_cache, available_memory
    );
    println!(
        "{}{:<15}{} {:>10} {:>10} {:>10}",
        bold, "Swap:", reset, total_swap, used_swap, free_swap
    );
}
