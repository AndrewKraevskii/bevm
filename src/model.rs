use std::time::SystemTime;
use core::ops::*;
use crate::parse::mc::{ExecutionResult, parse, McParser};
use crate::parse::Parser;
use crate::parse::general::GeneralParser;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Eq, PartialEq)]
pub enum Register {
    // McCounter
    // it probably should be here but it has type u8 and
    // therefore I cannot treat this register the same way
    // so I just don't include it here because actually I don't need to update it
    // via user's code

    Status,

    MicroCommand,

    // Buffer,
    // It probably should be here too. But it has unusual behaviour
    // namely it's type is u17

    Address,
    Command,
    Data,
    CommandCounter,
    Counter
}

impl Register {

    pub fn mnemonic(&self) -> String {
        match self {
            Register::Status => "РС",

            Register::MicroCommand => "РМК",
            Register::Address => "РА",
            Register::Command => "РК",
            Register::Data => "РД",
            Register::CommandCounter => "СК",
            Register::Counter => "А"
        }.to_string()

    }

    pub fn set(&self, computer: &mut Computer, data: u16) {
        match self {
            Register::Status => computer.registers.r_status = data,
            Register::MicroCommand => computer.registers.r_micro_command = data,
            Register::Address => computer.registers.r_address = data,
            Register::Command => computer.registers.r_command = data,
            Register::Data => computer.registers.r_data = data,
            Register::CommandCounter => computer.registers.r_command_counter = data.bitand(0x7FF),
            Register::Counter => computer.registers.r_counter = data
        }
    }

    pub fn get(&self, computer: &Computer) -> u16 {
        match self {
            Register::Status => computer.registers.r_status,
            Register::MicroCommand => computer.registers.r_micro_command,
            Register::Address => computer.registers.r_address,
            Register::Command => computer.registers.r_command,
            Register::Data => computer.registers.r_data,
            Register::CommandCounter => computer.registers.r_command_counter,
            Register::Counter => computer.registers.r_counter
        }
    }
}

pub struct Registers {
    pub r_micro_command_counter: u8, // СчМК. текущая микрокомана
    pub r_status: u16, // РС - регистр состояния. в разрядах биты статуса

    pub r_micro_command: u16, // РМК. регистр микро команды.
    // type is actually u17
    pub r_buffer: u32, // БР. буфферный регистр. мк
    pub r_address: u16, // РА - регистр адреса. мк
    pub r_command: u16, // РК - регистр команды. мк
    pub r_data: u16, // РД - регистр данных. мк

    pub r_command_counter: u16, // СК - счетчик команд. текущая команда эвм
    pub r_counter: u16 // А
}

macro_rules! status_flag {
    ($pos:expr, $set:ident, $get:ident) => {
        pub fn $set(&mut self, v: bool) {
            if v {
                self.r_status = self.r_status.bitor(1u16.shl($pos as u16) as u16);
            } else {
                self.r_status = self.r_status.bitand(1u16.shl($pos as u16).bitxor(0xFFFF) as u16);
            }
        }


        pub fn $get(&self) -> bool {
            self.r_status.bitand(1u16.shl($pos)) != 0u16
        }
    };
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            r_micro_command_counter: 0,
            r_status: 0,
            r_command_counter: 0,

            r_micro_command: 0,
            r_buffer: 0,
            r_address: 0,
            r_command: 0,
            r_data: 0,
            r_counter: 0
        }
    }

    status_flag!(0, set_overflow, get_overflow);
    status_flag!(1, set_null, get_null);
    status_flag!(2, set_negative, get_negative);
    status_flag!(4, set_allow_interrupt, get_allow_interupt);


}

pub struct Memory<P: Parser> {
    pub parser: P,
    pub data: Vec<Rc<RefCell<MemoryCell>>>,
}

#[derive(Clone)]
pub struct MemoryCell {
    data: u16,
    last_touched: SystemTime,
    pub mnemonic: Option<String>,
    pub name: Option<String>
}

impl MemoryCell {
    pub fn new() -> MemoryCell
    {
        MemoryCell {
            data: 0,
            last_touched: SystemTime::UNIX_EPOCH,
            mnemonic: None,
            name: None
        }
    }

    pub fn set(&mut self, data: u16) {
        self.data = data;
        self.last_touched = SystemTime::now();
    }

    pub fn get(&self) -> u16 {
        return self.data
    }
}

pub struct LogEntry {
    pub command_counter: u16,
    pub micro_counter: u8,
    pub micro_command: bool,
    pub info: String
}

pub struct Computer {
    pub registers: Registers,
    pub general_memory: Rc<RefCell<Memory<GeneralParser>>>,
    pub mc_memory: Rc<RefCell<Memory<McParser>>>,

    logs: Vec<LogEntry>
}

impl Computer {
    pub fn new() -> Computer {
        return Computer {

            registers: Registers::new(),
            general_memory: Rc::new(RefCell::new(Memory {
                data: vec![Rc::new(RefCell::new(MemoryCell::new())); 2048],
                parser: GeneralParser::new()
            })),
            mc_memory: Rc::new(RefCell::new(Memory {
                data: vec![Rc::new(RefCell::new(MemoryCell::new())); 256],
                parser: McParser::new()
            })),
            logs: Vec::<LogEntry>::new()
        }
    }

    pub fn log(&mut self, micro_command: bool, info: String) {
        self.logs.push(
            LogEntry {
                micro_counter: self.registers.r_micro_command_counter,
                command_counter: self.registers.r_command_counter,
                micro_command,
                info
            }
        )
    }

    pub fn logs(&self) -> &Vec<LogEntry> {
        &self.logs
    }

    pub fn micro_step(&mut self) -> ExecutionResult {
        let cmd = parse(self.mc_memory.borrow_mut().data.get(self.registers.r_micro_command_counter as usize).unwrap().borrow().get());
        let result = cmd.run(self);
        if !matches!(result, ExecutionResult::JUMPED) {
            self.registers.r_micro_command_counter += 1;
        }
        result
    }

}