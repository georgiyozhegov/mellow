use crate::assembly::Assembly;

pub fn optimize(assembly: Vec<Assembly>) -> Vec<Assembly> {
    let mut output = Vec::new();
    let mut assembly = assembly.iter().peekable();
    while let Some(instruction) = assembly.next() {
        match instruction {
            Assembly::Mov(ref to, ref from) => {
                if to == from {
                    continue;
                } else {
                    output.push(instruction.clone());
                }
            }
            Assembly::Jmp(ref label) => {
                match assembly.peek() {
                    Some(Assembly::Label(id)) => {
                        if id == label {
                            continue;
                        } else {
                            output.push(instruction.clone());
                        }
                    },
                    _ => output.push(instruction.clone()),
                }
            }
            instruction => {
                output.push(instruction.clone());
            },
        }
    }
    output
}
