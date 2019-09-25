use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Items {
    list: Vec<String>,
    recent_start: usize,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let milestones = parse_section(&contents, &is_milestone_header);
    let backlog = parse_section(&contents, &is_backlog_header);
    let goals = parse_repeating_section(&contents, &is_week_header);
    let tasks = parse_repeating_section(&contents, &is_day_header);
    println!("Milestones: \n{}\n", milestones.join("\n"));
    println!("Backlog: \n{}\n", backlog.join("\n"));
    println!(
        "Recent goals (of {}): \n{}\n",
        goals.list.len(),
        goals.list[goals.recent_start..].join("\n")
    );
    println!(
        "Recent tasks (of {}): \n{}\n",
        tasks.list.len(),
        tasks.list[tasks.recent_start..].join("\n")
    );
    Ok(())
}

fn parse_section(raw_content: &str, is_section_header: impl Fn(&str) -> bool) -> Vec<String> {
    let mut items = vec![];
    let lines = raw_content.lines();
    let mut in_backlog = false;
    for line in lines {
        if is_section_header(line) {
            in_backlog = true;
        } else if in_backlog && is_item_entry(line) {
            items.push(line.to_owned());
        } else {
            in_backlog = false;
        }
    }
    items
}

fn parse_repeating_section(raw_content: &str, is_section_header: impl Fn(&str) -> bool) -> Items {
    let mut items = vec![];
    let lines = raw_content.lines();
    let mut in_section = false;
    let mut section_indices = vec![];
    for line in lines {
        if is_section_header(line) {
            in_section = true;
            section_indices.push(items.len());
        } else if in_section && is_item_entry(line) {
            items.push(line.to_owned());
        } else {
            in_section = false;
        }
    }

    // It's possible for sections to exist with no items. If this happens,
    // there could be one or more indices on the section_indices stack that have indices
    // past the end of the items. For example:
    // ## Monday, 1st
    // - Task 1 (day_index gets pushed as 0 as the length before this was 0)
    // ## Tuesday, 2nd
    // <eof>
    // As Tuesday is a new day, the length of 1 would get pushed onto the day_indices stack).
    // So we need to continue popping until we have an index that reflects the last day entry with
    // task entries in it (0 in the above case).
    let mut recent_start = 0;
    while let Some(idx) = section_indices.pop() {
        recent_start = idx;
        if recent_start < items.len() {
            break;
        }
    }

    Items {
        list: items,
        recent_start,
    }
}

fn is_item_entry(line: &str) -> bool {
    line.starts_with("- ")
}

fn is_milestone_header(line: &str) -> bool {
    line.eq("# Milestones")
}

fn is_backlog_header(line: &str) -> bool {
    line.eq("# Backlog")
}

fn is_week_header(line: &str) -> bool {
    line.starts_with("# Weekly")
}

fn is_day_header(line: &str) -> bool {
    line.starts_with("## Monday")
        || line.starts_with("## Tuesday")
        || line.starts_with("## Wednesday")
        || line.starts_with("## Thursday")
        || line.starts_with("## Friday")
        || line.starts_with("## Saturday")
        || line.starts_with("## Sunday")
}
