use std::env;
use sysinfo::System;

fn format_value(value: u64, divisor: u64) -> u64 {
    value / divisor
}

fn get_color(used: u64, total: u64) -> (&'static str, &'static str) {
    let percentage = (used as f64 / total as f64) * 100.0;
    match percentage {
        p if p < 70.0 => ("\x1b[32m", "\x1b[0m"),
        p if p < 90.0 => ("\x1b[33m", "\x1b[0m"),
        _ => ("\x1b[31m", "\x1b[0m"),
    }
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

    let (mem_color, mem_reset) = get_color(used_memory, total_memory);
    let (swap_color, swap_reset) = get_color(used_swap, total_swap);

    println!(
        "{:<15} {}{:>10}{} {}{:>10}{} {}{:>10}{} {}{:>10}{} {}{:>12}{} {}{:>12}{}",
        "", bold, "total", reset, bold, "used", reset, bold, "free", reset, bold, "shared", reset, bold, "buff/cache", reset, bold, "available", reset
    );
    println!(
        "{}{:<15}{} {:>10} {}{:>10}{} {}{:>10}{} {:>10} {:>12} {:>12}",
        bold, "Mem.:", reset, total_memory, mem_color, used_memory, mem_reset, mem_color, free_memory, mem_reset, 0, buff_cache, available_memory
    );
    println!(
        "{}{:<15}{} {:>10} {}{:>10}{} {}{:>10}{}",
        bold, "Swap:", reset, total_swap, swap_color, used_swap, swap_reset, swap_color, free_swap, swap_reset
    );
}
