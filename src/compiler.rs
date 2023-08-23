use crate::parser::*;
use std::collections::HashMap;

pub fn compile_to_assembly(abstract_syntax_tree: &Vec<Statement>) -> String {
    let mut assembly : String = "global _start\n_start:\n".to_string();
    let mut iter = abstract_syntax_tree.iter().peekable();
    let mut stack_size : i32 = 0;
    let mut variables = HashMap::<String, i32>::new(); // Identifier -> stack location

    while let Some(statement) = iter.next() {
        match statement {
            Statement::Exit(expression) => {
                match expression {
                    Expression::IntLiteral(exit_value) => {
                        let to_append = format!("mov rax, 60\n mov rdi, {}\n syscall", exit_value);
                        assembly.push_str(&to_append);
                    },
                    Expression::Identifier(identifier) => {
                        if !variables.contains_key(identifier) {
                            panic!("Undeclared identifier: \"{}\"", identifier);
                        }
                        let stack_location = variables.get(identifier).unwrap();
                        let to_append = format!("push QWORD [rsp + {}]\n mov rax, 60\n pop rdi\n syscall", (stack_size - stack_location - 1) * 8);
                        assembly.push_str(&to_append);
                    }
                }
            },
            Statement::VariableDefinition { identifier, value } => {
                if variables.contains_key(identifier) {
                    panic!("Variable \"{}\" already exists!", identifier);
                }

                variables.insert(identifier.clone(), stack_size);
                let to_append = format!("mov rax, {}\n push rax\n", value);
                assembly.push_str(&to_append);
                stack_size += 1;
            }
        }
    }

    return assembly;
}
