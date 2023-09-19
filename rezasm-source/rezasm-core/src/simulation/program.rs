use crate::parser::line::Line;
use crate::util::error::SimulatorError;
use bimap::BiHashMap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    file_identifiers: BiHashMap<String, i64>,
    file_id_to_file: HashMap<i64, Vec<Line>>,
    // label -> (file_id, line_number)
    label_to_line: HashMap<String, (i64, i64)>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            file_identifiers: BiHashMap::new(),
            file_id_to_file: HashMap::new(),
            label_to_line: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.file_identifiers.clear();
        self.file_id_to_file.clear();
        self.label_to_line.clear();
    }

    pub fn get_line(&self, file_id: i64, line_number: i64) -> Result<&Line, SimulatorError> {
        if line_number < 0 {
            return Err(SimulatorError::InvalidLineNumber(line_number));
        }

        match self.file_id_to_file.get(&file_id) {
            None => Err(SimulatorError::InvalidFileIdentifier(file_id)),
            Some(file) => match file.get(line_number as usize) {
                None => Err(SimulatorError::InvalidProgramCounterError(line_number)),
                Some(line) => Ok(line),
            },
        }
    }

    pub fn add_line(&mut self, line: Line, file: String) -> Result<(), SimulatorError> {
        let file_id: i64 = match self.file_identifiers.get_by_left(file.as_str()) {
            None => {
                let id = self.file_identifiers.len() as i64;
                self.file_identifiers.insert(file, id);
                self.file_id_to_file.insert(id, Vec::new());
                id
            }
            Some(id) => id.clone(),
        };
        match &line {
            Line::Label(label) => {
                if self.label_to_line.contains_key(label) {
                    return Err(SimulatorError::LabelInUseError(label.to_string()));
                } else {
                    self.label_to_line.insert(
                        label.to_string(),
                        (
                            file_id,
                            match self.file_id_to_file.get(&file_id) {
                                None => return Err(SimulatorError::InvalidFileIdentifier(file_id)),
                                Some(lines) => lines.len() as i64,
                            },
                        ),
                    );
                }
            }
            _ => {}
        };

        match self.file_id_to_file.get_mut(&file_id) {
            None => {
                self.file_id_to_file.insert(file_id, vec![line]);
            }
            Some(file) => file.push(line),
        }

        Ok(())
    }

    pub fn resolve_label(&self, label: &String) -> Option<&(i64, i64)> {
        self.label_to_line.get(label.as_str())
    }

    pub fn is_error(&self, fid: i64, pc: i64) -> bool {
        match self.file_id_to_file.get(&fid) {
            None => false,
            Some(file) => pc > file.len() as i64,
        }
    }

    pub fn is_done(&self, fid: i64, pc: i64) -> bool {
        match self.file_id_to_file.get(&fid) {
            None => false,
            Some(file) => (pc == file.len() as i64) || (pc == 0 && file.is_empty()),
        }
    }

    pub fn end_pc(&self, fid: i64) -> usize {
        match self.file_id_to_file.get(&fid) {
            None => 0,
            Some(file) => file.len(),
        }
    }
}
