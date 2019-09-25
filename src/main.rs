use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Items {
    list: Vec<String>,
    current_start: usize,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let backlog = get_backlog(&contents);
    let goals = get_all_items(&contents, &is_week_header);
    let tasks = get_all_items(&contents, &is_day_header);
    println!("Backlog: \n{}\n", backlog.join("\n"));
    println!(
        "Current goals: \n{}\n",
        goals.list[goals.current_start..].join("\n")
    );
    println!(
        "Current tasks: \n{}\n",
        tasks.list[tasks.current_start..].join("\n")
    );
    Ok(())
}

fn get_backlog(raw_content: &str) -> Vec<String> {
    let mut items = vec![];
    let lines = raw_content.lines();
    let mut in_backlog = false;
    for line in lines {
        if is_backlog_header(line) {
            in_backlog = true;
        } else if in_backlog && is_item_entry(line) {
            items.push(line.to_owned());
        } else {
            in_backlog = false;
        }
    }
    items
}

fn get_all_items(raw_content: &str, header: impl Fn(&str) -> bool) -> Items {
    let mut items = vec![];
    let lines = raw_content.lines();
    let mut in_section = false;
    let mut section_indices = vec![];
    for line in lines {
        if header(line) {
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
    let mut current_start = 0;
    while let Some(idx) = section_indices.pop() {
        current_start = idx;
        if current_start < items.len() {
            break;
        }
    }

    Items {
        list: items,
        current_start,
    }
}

fn is_item_entry(line: &str) -> bool {
    line.starts_with("- ")
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
