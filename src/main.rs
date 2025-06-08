use std::env::args;
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
    let args: Vec<String> = args().collect();
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
    let dim = "\x1b[2m";

    let (mem_color, mem_reset) = get_color(used_memory, total_memory);
    let (swap_color, swap_reset) = get_color(used_swap, total_swap);

    let headers = ["", "total", "used", "free", "shared", "buff/cache", "available"];
    let mem_row = [
        "Memory".to_string(),
        total_memory.to_string(),
        used_memory.to_string(),
        free_memory.to_string(),
        "0".to_string(),
        buff_cache.to_string(),
        available_memory.to_string(),
    ];
    let swap_row = [
        "Swap".to_string(),
        total_swap.to_string(),
        used_swap.to_string(),
        free_swap.to_string(),
        String::new(),
        String::new(),
        String::new(),
    ];

    let mut col_widths = vec![0; headers.len()];
    for (i, h) in headers.iter().enumerate() {
        col_widths[i] = h.len();
    }
    for (i, v) in mem_row.iter().enumerate() {
        col_widths[i] = col_widths[i].max(v.len());
    }
    for (i, v) in swap_row.iter().enumerate() {
        col_widths[i] = col_widths[i].max(v.len());
    }

    fn build_border(left: &str, sep: &str, right: &str, col_widths: &[usize]) -> String {
        let mut s = String::new();
        s.push_str(left);
        for (i, w) in col_widths.iter().enumerate() {
            if i > 0 { s.push_str(sep); }
            s.push_str(&"─".repeat(*w + 2));
        }
        s.push_str(right);
        s
    }

    fn build_row(row: &[String], col_widths: &[usize], colors: Option<(&str, &str)>, bold: &str, reset: &str, bold_indices: &[usize], dim_indices: &[usize], dim: &str) -> String {
        let mut s = String::new();
        s.push('│');
        for (i, cell) in row.iter().enumerate() {
            s.push(' ');
            if bold_indices.contains(&i) {
                s.push_str(bold);
            } else if dim_indices.contains(&i) {
                s.push_str(dim);
            }
            if let Some((color, color_reset)) = colors {
                if (i == 2 || i == 3) && !cell.is_empty() {
                    s.push_str(color);
                    s.push_str(&format!("{:>width$}", cell, width = col_widths[i]));
                    s.push_str(color_reset);
                } else {
                    s.push_str(&format!("{:<width$}", cell, width = col_widths[i]));
                }
            } else {
                s.push_str(&format!("{:<width$}", cell, width = col_widths[i]));
            }
            if bold_indices.contains(&i) || dim_indices.contains(&i) {
                s.push_str(reset);
            }
            s.push(' ');
            s.push('│');
        }
        s
    }

    let top = build_border("┌", "┬", "┐", &col_widths);
    let sep = build_border("├", "┼", "┤", &col_widths);
    let bottom = build_border("└", "┴", "┘", &col_widths);

    println!("{}", top);
    let header_row: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    let bold_indices: Vec<usize> = (0..headers.len()).collect();
    let dim_indices: Vec<usize> = vec![];
    println!("{}", build_row(&header_row, &col_widths, None, bold, reset, &bold_indices, &dim_indices, dim));
    println!("{}", sep);
    println!("{}", build_row(&mem_row, &col_widths, Some((mem_color, mem_reset)), bold, reset, &[0], &[4,5], dim));
    println!("{}", build_row(&swap_row, &col_widths, Some((swap_color, swap_reset)), bold, reset, &[0], &[4,5], dim));
    println!("{}", bottom);
}
