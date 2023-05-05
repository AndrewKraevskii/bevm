use crate::model::Computer;
use crate::parse::mc::ExecutionResult;

type Tracer = (
    &'static str,
    fn(computer: &mut Computer, len: usize) -> Tracing,
);

struct Tracing {
    pub header: Vec<String>,
    pub tracing: Vec<Vec<String>>,
}

fn mc_tracing(computer: &mut Computer, len: usize) -> Tracing {
    let mut steps_left = len;

    let mut result = Vec::new();

    while steps_left > 0 {
        let pos = computer.registers.r_micro_command_counter;
        let code = computer
            .mc_memory
            .data
            .get(pos as usize)
            .unwrap()
            .get();

        computer.registers.set_execute_by_tick(false);
        computer.registers.set_lever(false);
        computer.registers.set_program_mode(false);

        computer.micro_step();

        result.push(vec![
            format!("{pos:0>3X}"),
            format!("{code:0>4X}"),
            format!("{:0>3X}", computer.registers.r_command_counter),
            format!("{:0>3X}", computer.registers.r_address),
            format!("{:0>4X}", computer.registers.r_command),
            format!("{:0>4X}", computer.registers.r_data),
            format!("{:0>4X}", computer.registers.r_counter),
            if computer.registers.get_overflow() {
                "1".to_owned()
            } else {
                "0".to_owned()
            },
            format!("{:0>4X}", computer.registers.r_buffer),
            if computer.registers.get_negative() {
                "1".to_owned()
            } else {
                "0".to_owned()
            },
            if computer.registers.get_null() {
                "1".to_owned()
            } else {
                "0".to_owned()
            },
            format!("{:0>3X}", computer.registers.r_micro_command_counter),
        ]);

        if computer.registers.r_command == 0xF000 {
            break;
        }

        steps_left -= 1;
    }

    Tracing {
        header: vec![
            "СчМК до выборки МК",
            "ВМК",
            "СК",
            "РА",
            "РК",
            "РД",
            "А",
            "С",
            "БР",
            "N",
            "Z",
            "СчМК",
        ]
        .iter()
        .map(|a| a.to_owned().to_owned())
        .collect(),
        tracing: result,
    }
}
fn general_tracing(computer: &mut Computer, len: usize) -> Tracing {
    let mut steps_left = len;

    let mut result = Vec::new();

    while steps_left > 0 {
        let pos = computer.registers.r_command_counter;
        let code = computer
            .general_memory
            .data
            .get(pos as usize)
            .unwrap()
            .get();
        let mem_before = computer.general_memory.data.clone();

        computer.registers.set_execute_by_tick(false);
        computer.registers.set_lever(false);
        computer.registers.set_program_mode(false);
        computer.find(|&res| res == ExecutionResult::Halted);

        let mut line = vec![
            format!("{pos:0>3X}"),
            format!("{code:0>4X}"),
            format!("{:0>4X}", computer.registers.r_command_counter),
            format!("{:0>4X}", computer.registers.r_address),
            format!("{:0>4X}", computer.registers.r_command),
            format!("{:0>4X}", computer.registers.r_data),
            format!("{:0>4X}", computer.registers.r_counter),
            if computer.registers.get_overflow() {
                "1".to_owned()
            } else {
                "0".to_owned()
            },
        ];
        for i in 0..mem_before.len() {
            if computer.general_memory.data.get(i).unwrap().get()
                != mem_before.get(i).unwrap().get()
            {
                line.push(format!("{i:0>3X}"));
                line.push(format!(
                    "{:0>4X}",
                    computer.general_memory.data.get(i).unwrap().get()
                ));
            }
        }
        result.push(line);

        steps_left -= 1;

        if computer.registers.r_command == 0xF000 {
            break;
        }
    }

    Tracing {
        header: vec![
            "Адресс",
            "Код",
            "СК",
            "РА",
            "РК",
            "РД",
            "А",
            "С",
            "Адрес",
            "Новый код",
        ]
        .iter()
        .map(|a| a.to_owned().to_owned())
        .collect(),
        tracing: result,
    }
}
