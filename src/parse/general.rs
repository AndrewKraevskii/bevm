use crate::parse::{CommandInfo, Parser};

use core::ops::{BitAnd, BitOr, BitXor};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CommandType {
    Simple,
    Address,
    IO,
}

#[derive(Clone, Copy)]
struct Command {
    name: &'static str,
    description: &'static str,
    mask: u16,
    command_type: CommandType,
}

impl Command {
    fn mnemonic(&self) -> &str {
        self.name
    }

    fn new_simple(mask: u16, name: &'static str, description: &'static str) -> Self {
        Command {
            mask,
            name,
            description,
            command_type: CommandType::Simple,
        }
    }

    fn new_io(mask: u16, name: &'static str, description: &'static str) -> Self {
        Command {
            mask,
            name,
            description,
            command_type: CommandType::IO,
        }
    }

    fn new_address(mask: u16, name: &'static str, description: &'static str) -> Self {
        Command {
            mask,
            name,
            description,
            command_type: CommandType::Address,
        }
    }

    fn matching(&self, cmd: u16) -> bool {
        self.mask.bitand(cmd).bitand(self.mask) == self.mask
    }

    fn file_string(&self, cmd: u16) -> String {
        match self.command_type {
            CommandType::Simple => {
                let excessive = cmd.bitand(self.mask.bitxor(0xFFFF));

                if excessive == 0 {
                    self.mnemonic().to_owned()
                } else {
                    format!("{cmd:0>4X}")
                }
            }
            CommandType::Address | CommandType::IO => self.parse(cmd),
        }
    }

    fn parse(&self, data: u16) -> String {
        if data.bitand(self.mask).bitand(self.mask) != self.mask {
            panic!();
        }

        match self.command_type {
            CommandType::Simple => self.name.to_string(),
            CommandType::Address => {
                let address = data.bitand(0x7FF);
                if data.bitand(0x0800) != 0 {
                    format!("{} ({:0>3X})", self.name, address)
                } else {
                    format!("{} {:0>3X}", self.name, address)
                }
            }
            CommandType::IO => {
                let address = data.bitand(0xF);
                if data.bitand(0x0800) != 0 {
                    format!("{} ({:0>3X})", self.name, address)
                } else {
                    format!("{} {:0>3X}", self.name, address)
                }
            }
        }
    }

    fn rev_parse(&self, s: &str) -> Result<u16, String> {
        match self.command_type {
            CommandType::Simple => {
                if s.trim().to_uppercase() != self.name {
                    Err(format!(
                        "{} является безадресной командой и не принимает аргументов",
                        self.name
                    ))
                } else {
                    Ok(self.mask)
                }
            }
            CommandType::Address | CommandType::IO => {
                let splited = s.trim().split(' ').collect::<Vec<&str>>();

                if splited.len() > 2 {
                    return Err(format!(
                        "Неожиданные штуки:{}",
                        splited
                            .iter()
                            .skip(2)
                            .fold("".to_string(), |a, b| format!("{} {}", a, b))
                    ));
                }

                if splited.len() < 2 {
                    return Err("Ожидалось два параметра".to_string());
                }

                let address = splited.get(1).unwrap().trim();

                let indirect = if address.is_empty() || !address.starts_with('(') {
                    false
                } else {
                    if !address.ends_with(')') {
                        return Err("Не закрытая скобка".to_string());
                    }
                    true
                };

                let address = if indirect {
                    address.get(1..address.len() - 1).unwrap()
                } else {
                    address
                };

                if let Ok(parsed) = u16::from_str_radix(address, 16) {
                    let is_io = self.command_type == CommandType::IO;
                    let max = if is_io { 0xF } else { 0x7FF };
                    if parsed > max {
                        if is_io {
                            Err("Максимально адресуемое ВУ 0xF".to_string())
                        } else {
                            Err("Максимально адресуема память 0x7FF".to_string())
                        }
                    } else if indirect {
                        Ok(self.mask.bitor(parsed).bitor(0x0800))
                    } else {
                        Ok(self.mask.bitor(parsed))
                    }
                } else {
                    Err(format!(
                        "Ошибка во время парсинга числа {}",
                        splited.get(1).unwrap()
                    ))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct CommandParser {
    commands: Vec<Command>,
    mnemonic_map: HashMap<String, usize>,
}

impl CommandParser {
    pub fn new() -> CommandParser {
        let commands =  vec![Command::new_simple(
            0xFF00,
            "HZF",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ),
        Command::new_simple(
            0xFE00,
            "HZE",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ),
        Command::new_simple(
            0xFD00,
            "HZD",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ),
        Command::new_simple(
            0xFC00,
            "HZC",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ),
        Command::new_simple(0xF700, "ROR", "Сдвигает биты в регистре А вправо. При этом содержимое С попадает в старший бит А, а младший бит А попадает в С"),
        Command::new_simple(0xFB00, "DI", "Запрещает прерывания"),
        Command::new_simple(0xF300, "CLC", "Устанавливает С в 0"),
        Command::new_simple(
            0xF500,
            "CMC",
            "Инвертирует С. То есть, если С было равно 1, оно станет 0 и наоборот.",
        ),
        Command::new_simple(0xF600, "ROL", "Сдвигает биты в регистре А влево. При этом содержимое С попадает в младший бит А, а старший бит А попадает в С."),
        Command::new_simple(
            0xF900,
            "DEC",
            "Уменьшает значение А на 1",
        ),
        Command::new_simple(0xFA00, "EI", "Разрешает прерывания"),
        Command::new_simple(
            0xF200,
            "CLA",
            "Устанавливает значение регистра А в 0",
        ),
        Command::new_simple(0xF400, "CMA", "Инвертирует содержимое регистра А. То есть каждый бит регистра А, который равен 0, станет 1 и наоборот."),
        Command::new_simple(
            0xF800,
            "INC",
            "Увеличивает значение регистра А на 1",
        ),
        Command::new_simple(0xF100, "NOP", "Команда, которая не делает ничего. Удобно применять вместе с ISZ для инкремента какой-либо ячейки памяти."),
        Command::new_io(
            0xE300,
            "OUT",
            "Присваивает указаному ВУ значение из регистра А",
        ),
        Command::new_simple(0xF000, "HLT", "Выключает ЭВМ."),
        Command::new_io(0xE100, "TSF", "Присваивает 6 биту регистра РС статус готовности указанного ВУ. Затем, если 6 бит РС равен единице, регистр СК увеличивается на единицу."),
        Command::new_io(
            0xE200,
            "IN",
            "Берет значение из данного ВУ и кладет его в 8 младших бит регистра А",
        ),
        Command::new_address(
            0xB000,
            "BEQ",
            "Присваивает регистру СК значение X, если регистр А равен 0.",
        ),
        Command::new_io(
            0xE000,
            "CLF",
            "Устанавливает флаг готовности данного ВУ в 0.",
        ),
        Command::new_address(
            0x3000,
            "MOV",
            "Присваивает ячейке по адресу X значение из регистра А",
        ),
        Command::new_address(0x5000, "ADC", "Складывает значение из ячейки по адресу X с регистром А и добавляет 1, если С равен 1."),
        Command::new_address(
            0x6000,
            "SUB",
            "Вычитает значение ячейки по адресу X из регистра А.",
        ),
        Command::new_address(
            0x9000,
            "BPL",
            "Присваивает регистру СК значение X, если значение в регистре А больше или равно 0.",
        ),
        Command::new_address(
            0xA000,
            "BMI",
            "Присваивает регистру СК значение X, если значение в регистре А строго меньше 0.",
        ),
        Command::new_address(
            0xC000,
            "BR",
            "Присваивает регистру СК значение X",
        ),
        Command::new_address(
            0x1000,
            "AND",
            "Присваивает регистру А результат бинарного И между регистром А и значением в ячейке X",
        ),
        Command::new_address(
            0x4000,
            "ADD",
            "Присваивает регистру А результат сложения регистром А и значением в ячейке X",
        ),
        Command::new_address(
            0x8000,
            "BCS",
            "Присваивает регистру СК значение X, если С равно 1",
        ),
        Command::new_address(0x2000, "JSR", "Команда для организации логики подпрограмм. Значение регистра СК будет положено в ячейку по адресу X после чего регистру СК будет присвоенное значение X + 1"),
        Command::new_address(0x0000, "ISZ", "Увеличивает значение в ячейке по адресу X на 1. После чего, если значение в этой ячейке больше или равно 0, увеличивает СК на 1 тем самым \"перепрыгивает\" следующую команду."),
        Command::new_address(
            0x7000,
            "HZA7",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        ),
        Command::new_address(
            0xD000,
            "HZAD",
            "Команда для которой не задано поведение. То есть она ничего не делает.",
        )];

        let mut parser = CommandParser {
            commands: commands,
            mnemonic_map: HashMap::new(),
        };

        parser.commands.sort_by(|a, b| {
            let l = format!("{:b}", a.mask)
                .chars()
                .filter(|c| *c == '1')
                .count();
            let r = format!("{:b}", b.mask)
                .chars()
                .filter(|c| *c == '1')
                .count();
            r.cmp(&l)
        });

        parser.mnemonic_map.extend(
            parser
                .commands
                .iter()
                .enumerate()
                .map(|(i, command)| (command.mnemonic().to_string(), i)),
        );

        parser
    }

    fn mnemonic_to_command(&self, mnemonic: &str) -> Option<&Command> {
        self.commands.get(*self.mnemonic_map.get(mnemonic)?)
    }
}

impl Parser<GeneralCommandInfo> for CommandParser {
    fn parse(&self, v: u16) -> GeneralCommandInfo {
        for command in &self.commands {
            if command.matching(v) {
                return GeneralCommandInfo::new(command.clone(), v);
            }
        }
        panic!()
    }

    fn supports_rev_parse(&self) -> bool {
        true
    }

    fn rev_parse(&self, str: &str) -> Result<u16, String> {
        let mnemonic = str.split(' ').next();
        if mnemonic.is_none() {
            return Err("Пустая строка получается".to_string());
        }

        let mnemonic = mnemonic.unwrap().to_uppercase();
        let command = self.mnemonic_to_command(mnemonic.as_str());

        let Some(command) = command else {
            return Err(format!("Неизвестная мнемоника {mnemonic}"));
        };
        command.rev_parse(str)
    }
}

trait GeneralCommand {
    fn matching(&self, cmd: u16) -> bool {
        self.mask().bitand(cmd).bitand(self.mask()) == self.mask()
    }

    fn file_string(&self, cmd: u16) -> String;

    fn mnemonic(&self) -> &str;

    fn mask(&self) -> u16;

    fn parse(&self, data: u16) -> String;

    fn rev_parse(&self, s: &str) -> Result<u16, String>;
}

#[derive(Clone, Copy)]
pub struct GeneralCommandInfo {
    info: Command,
    opcode: u16,
}

impl GeneralCommandInfo {
    fn new(info: Command, opcode: u16) -> GeneralCommandInfo {
        GeneralCommandInfo { info, opcode }
    }
}

impl CommandInfo for GeneralCommandInfo {
    fn file_string(&self) -> String {
        self.info.file_string(self.opcode)
    }

    fn mnemonic(&self) -> String {
        self.info.parse(self.opcode)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::general::CommandParser;
    use crate::parse::{CommandInfo, Parser};

    #[test]
    fn test() {
        let parser = CommandParser::new();

        assert_eq!(parser.parse(0xF700).mnemonic(), "ROR");
        assert_eq!(parser.parse(0x3024).mnemonic(), "MOV 024");
    }
}
