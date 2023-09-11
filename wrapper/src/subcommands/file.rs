use std::{fs, collections::HashMap};

use ayudame_core_rs::ayu_events::*;

use super::DUMMY_MEMADDR;

pub (crate) enum FileError {
    Io(&'static str),
    Syntax(&'static str),
}

impl ToString for FileError {
    fn to_string(&self) -> String {
        let m = match self {
            FileError::Io(m) => m,
            FileError::Syntax(m) => m,
        };
        m.to_string()
    }
}

pub (crate) fn from_file(filename: &str) -> Result<(), FileError> {
    let file = fs::read_to_string(filename).map_err(|_| FileError::Io("Unable to open file..."))?;
    let edges = parse_inputs(file)?;
    let mut ids = HashMap::new();
    let mut id_count = 1;

    // initialize with temanejo
    ayu_event_preinit(0);
    ayu_event_init(2);

    // build the graph
    for (from, to) in edges {
        if !ids.contains_key(&from) {
            ids.insert(from.clone(), id_count);
            ayu_event_addtask(id_count, 0, 0, 0);
            id_count += 1;
            // send out event
        } 
        if !ids.contains_key(&to) {
            ids.insert(to.clone(), id_count);
            ayu_event_addtask(id_count, 0, 0, 0);
            id_count += 1;
        }

        // send add dependency event
        let from_id = *ids.get(&from).unwrap();
        let to_id = *ids.get(&to).unwrap();
        ayu_event_adddependency(to_id, from_id, DUMMY_MEMADDR, DUMMY_MEMADDR);
    }

    Ok(())
}

fn parse_inputs(file: String) -> Result<Vec<(String, String)>, FileError> {
    let mut edges = Vec::new();

    for mut line in file.lines() {
        // remove comments
        if line.contains('#') {
            let comment = line.find('#').unwrap();
            line = &line[..comment];
        }
        // if line is empty, continue
        if line.len() == 0 {
            continue;
        }
        // try to parse edges
        let split = line.split_whitespace().collect::<Vec<_>>();
        match &split[..] {
            [from, "->", to] => edges.push((from.to_string(), to.to_string())),
            _ => return Err(FileError::Syntax("Invalid syntax in file, needs to be in the form of `from` -> `to`")),
        }
    }

    Ok(edges)
}