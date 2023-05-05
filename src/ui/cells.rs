use crate::model::{Computer, Memory, MemoryCell};
use crate::parse::{CommandInfo, Parser};

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::rc::Rc;

#[derive(PartialEq, Eq)]
enum CellRepresentation {
    Hex,
    Binary,
}

impl CellRepresentation {
    fn title(&self) -> &'static str {
        match self {
            CellRepresentation::Hex => "Шестнадцетеричное",
            CellRepresentation::Binary => "Бинарное",
        }
    }
}

pub struct CellsTool<I: CommandInfo, P: Parser<I>, F>
where
    F: Fn(&Computer) -> u16,
{
    page: Rc<RefCell<Memory<I, P>>>,
    counter_register: F,
    representation: CellRepresentation,
}

impl<I: CommandInfo, P: Parser<I>, F: Fn(&Computer) -> u16> CellsTool<I, P, F> {
    pub fn new(page: Rc<RefCell<Memory<I, P>>>, counter_register: F) -> CellsTool<I, P, F> {
        CellsTool {
            counter_register,
            page,
            representation: CellRepresentation::Hex,
        }
    }

    fn save_to_file(&mut self, file: &str) -> Result<(), String> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(false)
            .write(true)
            .truncate(true)
            .open(file)
            .map_err(|e| e.to_string())?;

        let mut s = String::new();
        let mut prev_zero = true;

        for (pos, cell) in self.page.borrow().data.iter().enumerate() {
            let v = cell.get();
            if v == 0 {
                prev_zero = true
            } else {
                if prev_zero {
                    s.push_str(format!("$pos {:X}\n", pos).as_str())
                }
                let str = self.page.borrow().parser.parse(v).file_string();
                s.push_str(str.as_str());
                s.push('\n');
                prev_zero = false;
            }
        }

        f.write(s.as_bytes()).map_err(|_| "Can't write file")?;
        f.flush().map_err(|_| "Can't write file")?;

        Ok(())
    }
}
