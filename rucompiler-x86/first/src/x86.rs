use std::collections::HashMap;

pub fn gen_x86(llvm_ir: &String) -> String {
    let mut assembly_code = String::new();
    let mut lines = llvm_ir.split("\n");
    let mut first_line = lines.next().unwrap();
    let mut args: HashMap<String, String> = HashMap::new();
    let arg_registers = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    if let Some(start) = first_line.find("@") {
        if let Some(end) = first_line.find("(") {
            let func_name = &first_line[start + 1..end];
            assembly_code.push_str(&format!(".text\n\t.global {}\n\n{}:\n\tpushq %rbp\n\tmovq %rsp, %rbp\n", func_name, func_name));
            let mut idx = 0;
            for arg in first_line[end+1..first_line.len() - 3].split_whitespace() {
                if let Some(start) = arg.find("%") {
                    if let Some(end) = arg.find(",") {
                        args.insert(arg[start..end].to_string(), arg_registers[idx].to_string());
                        idx += 1;
                    }
                    else {
                        args.insert(arg[start..].to_string(), arg_registers[idx].to_string());
                    }
                }
            }
            // println!("{:?}", args);
        }
    }
    for line in lines.collect::<Vec<_>>() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let mut src = String::new();
        let mut dest = String::new();
        let mut inst = String::new();
        if tokens.len() > 3 {
            if tokens[2] == "mul" {
                inst = "imulq".to_string();
            }
            else if tokens[2] == "add" {
                inst = "addq".to_string();
            }
            if tokens[4].contains("%") {
                dest = args[&tokens[4][..tokens[4].len()-1]].to_string();
                if dest == "%rax" || dest == "%r10" {
                    assembly_code.push_str("\tpopq %r10\n");
                    dest = "%r10".to_string();
                }
                if tokens[5].contains("%") {
                    src = args[tokens[5]].to_string();
                    if src == "%rax" || src == "%r10" {
                        assembly_code.push_str(&format!("\tpopq %rax\n"));
                        assembly_code.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
                        assembly_code.push_str(&format!("\tpushq {}\n", dest));
                    }
                    else {
                        if dest == "%rax" || dest == "%r10" {
                            let tmp = src;
                            src = dest;
                            dest = tmp;
                        }
                        assembly_code.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
                    }
                }
                else {
                    src = format!("${}", tokens[5].to_string());
                    assembly_code.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
                }
            }
            else {
                src = format!("${}", tokens[4][..tokens[4].len()-1].to_string());
                if tokens[5].contains("%") {
                    dest = args[tokens[5]].to_string();
                    if dest == "%rax" || dest == "%r10" {
                        assembly_code.push_str(&format!("\tpopq %rax\n"));
                        assembly_code.push_str(&format!("\t{} {}, %rax\n", inst, src));
                        assembly_code.push_str(&format!("\tpushq %rax\n"));
                    }
                    else {
                        assembly_code.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
                    }
                }
                else {
                    // src = "%rax".to_string();
                    dest = "%rax".to_string();
                    assembly_code.push_str(&format!("\tmovq {}, {}\n", src, dest));
                    assembly_code.push_str(&format!("\t{} ${}, {}\n", inst, tokens[5], dest));
                    assembly_code.push_str(&format!("\tpushq {}\n", dest))
                }
                // args.insert(tokens[0].to_string(), &dest.clone());
            }
            args.insert(tokens[0].to_string(), dest.clone());
            // assembly_code.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
        }
        else if tokens.len() > 1 {
            assembly_code.push_str(&format!("\tmovq {}, %rax\n", args[tokens[2]]));
        }
        else if tokens.len() > 0 {
            assembly_code.push_str(&"\tmovq %rbp, %rsp\n\tpopq %rbp\n\tret\n");
        }
        // println!();
    }
    // println!("{:?}", lines.collect::<Vec<_>>());

    // assembly_code.push_str(&format!("\n.section .note.GNU-stack,{},@progbits", ""));
    assembly_code.push_str(".section .note.GNU-stack,\"\",@progbits\n");
    assembly_code
}