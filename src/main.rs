use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Tasks {
    list: Vec<String>,
    latest_task_day: usize,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks = get_tasks(contents);
    println!("{}", tasks.list[tasks.latest_task_day..].join("\n"));
    Ok(())
}

fn get_tasks(raw_content: String) -> Tasks {
    let mut tasks = vec![];
    let lines = raw_content.lines();
    let mut reading_day = false;
    let mut day_indices = vec![];
    for line in lines {
        if line.starts_with('#') {
            if entering_day_section(line) {
                reading_day = true;
                day_indices.push(tasks.len());
            } else {
                reading_day = false;
            }
        }

        if reading_day && line.starts_with("- ") {
            tasks.push(line.to_owned());
        }
    }

    // It's possible for days to exist with no tasks. If this happens,
    // there could be one or more indices on the day_indices stack that have indices
    // past the end of the vector. For example:
    // ## Monday, 1st
    // - Task 1 (day_index gets pushed as 0 as the length before this was 0)
    // ## Tuesday, 2nd
    // <eof>
    // As Tuesday is a new day, the length of 1 would get pushed onto the day_indices stack).
    // So we need to continue popping until we have an index that reflects the last day entry with
    // task entries in it (0 in the above case).
    let mut day_index = 0;
    while let Some(idx) = day_indices.pop() {
        day_index = idx;
        if day_index < tasks.len() {
            break;
        }
    }

    Tasks {
        list: tasks,
        latest_task_day: day_index,
    }
}

fn entering_day_section(line: &str) -> bool {
    line.starts_with("## Monday")
        || line.starts_with("## Tuesday")
        || line.starts_with("## Wednesday")
        || line.starts_with("## Thursday")
        || line.starts_with("## Friday")
        || line.starts_with("## Saturday")
        || line.starts_with("## Sunday")
}
