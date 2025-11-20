use std::collections::HashMap;

pub fn gen_x86(llvm_ir: &String) -> String {
    let mut assembly_code = String::new();
    let mut lines = llvm_ir.split("\n").collect::<Vec<_>>().into_iter().peekable();
    let mut first_line = lines.next().unwrap();
    let mut args: HashMap<String, String> = HashMap::new();
    let arg_registers = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    if let Some(start) = first_line.find("@") {
        if let Some(end) = first_line.find("(") {
            let func_name = &first_line[start + 1..end];
            assembly_code.push_str(&format!(".text\n\t.global {}\n\n{}:\n\tpushq %rbp\n\tmovq %rsp, %rbp\n\tpushq %rbx\n\n", func_name, func_name));
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
    while let Some(line) = lines.next() {
        if line.starts_with("entry:") {
            let mut rsp = 0;
            let mut buffer = None;
            while let Some(inner_line) = lines.next() {
                if !inner_line.ends_with("alloca i64") {
                    buffer = Some(inner_line);
                    break;
                }
                let tokens: Vec<_> = inner_line.split_whitespace().collect();
                rsp += 8;
                args.insert(tokens[0].to_string(), format!("-{}(%rbp)", rsp));
            }
            assembly_code.push_str(&format!("\tsubq ${}, %rsp\n", rsp));
            while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                if !inner_line.starts_with("\tstore") {
                    buffer = Some(inner_line);
                    break;
                }
                let tmp = inner_line.replace(",", "");
                let mut tokens: Vec<_> = tmp.split_whitespace().collect();
                if tokens[2].contains("%") {
                    assembly_code.push_str(&format!("\tmovq {}, {}\n", args[tokens[2]], args[tokens[4]]));
                }
                else {
                    // let value = &tokens[2][..end];
                    assembly_code.push_str(&format!("\tmovq ${}, {}\n", tokens[2], args[tokens[4]]));
                }
            }
            while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                if !inner_line.starts_with("\t") {
                    buffer = Some(inner_line);
                    break;
                }
                let tmp = inner_line.replace(",", "");
                let mut tokens: Vec<_> = tmp.split_whitespace().collect();
                if inner_line.starts_with("\t%") {
                    if inner_line.contains("= load") {
                        assembly_code.push_str(&format!("\tmovq {}, %rbx\n", args[tokens[5]]));
                        args.insert(tokens[0].to_string(), "%rbx".to_string());
                        if let Some(operand) = lines.peek() {
                            if operand.starts_with("\t%") && operand.contains("= load") {
                                tokens = operand.split_whitespace().collect();
                                assembly_code.push_str(&format!("\tmovq {}, %rax\n", args[tokens[5]]));
                                assembly_code.push_str("\tpushq %rax\n");
                                args.insert(tokens[0].to_string(), "%rax".to_string());
                                lines.next();
                            }
                        }
                        assembly_code.push_str("\tpushq %rbx\n");
                    }
                    else if inner_line.contains("= mul") || inner_line.contains("= add") {
                        let mut src = String::new();
                        let mut dest = String::new();
                        let inst = format!("{}q", tokens[2]);
                        if tokens[4].contains("%") {
                            src = "%r10".to_string();
                            assembly_code.push_str(&format!("\tpopq {}\n", src));
                            if tokens[5].contains("%") {
                                dest = "%r11".to_string();
                                assembly_code.push_str(&format!("\tpopq {}\n", dest));
                            }
                            else {
                                let tmp = src;
                                src = format!("${}", tokens[5]);
                                dest = tmp;
                            }
                        }
                        else {
                            src = format!("${}", tokens[4]);
                            if tokens[5].contains("%") {
                                dest = "%r11".to_string();
                                assembly_code.push_str(&format!("\tpopq {}\n", dest));
                            }
                            else {
                                dest = "%r11".to_string();
                                assembly_code.push_str(&format!("\tmovq {}, {}\n", src, dest));
                                src = format!("${}", tokens[5]);
                                // assembly_code.push_str(&format!("\t{} ${}, {}\n", inst, tokens[5], dest));
                                // assembly_code.push_str(&format!("\tpushq {}\n", dest))
                                // dest = format!("${}", tokens[5]);
                            }
                        }
                        assembly_code.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
                        assembly_code.push_str(&format!("\tpushq {}\n", dest));
                        args.insert(tokens[0].to_string(), dest.to_string());
                    }
                    // else if inner_line.contains("= add") {
                    //     assembly_code.push_str(&format!("\taddq {}, {}\n", ,args[tokens[5]]));
                    // }
                    else if inner_line.contains("= icmp") {
                        let mut lhs = String::new();
                        let mut rhs = String::new();
                        if tokens[5].contains("%") {
                            if args[tokens[5]] == "%rbx" || args[tokens[5]] == "%rax" {
                                assembly_code.push_str(&format!("\tpopq {}\n", args[tokens[5]].clone()));
                            }
                            lhs = args[tokens[5]].clone();
                        }
                        else {
                            lhs = format!("${}",tokens[5].to_string());
                        }
                        if tokens[6].contains("%") {
                            if args[tokens[5]] == "%rbx" || args[tokens[5]] == "%rax" {
                                assembly_code.push_str(&format!("\tpopq {}\n", args[tokens[6]].clone()));
                            }
                            rhs = args[tokens[6]].clone();
                        }
                        else {
                            rhs = format!("${}",tokens[6].to_string());
                        }
                        assembly_code.push_str(&format!("\tcmpq {}, {}\n", lhs, rhs));
                        match tokens[3] {
                            "ult" => args.insert(tokens[0].to_string(), "jb".to_string()),
                            "ule" => args.insert(tokens[0].to_string(), "jbe".to_string()),
                            "ugt" => args.insert(tokens[0].to_string(), "ja".to_string()),
                            "uge" => args.insert(tokens[0].to_string(), "jae".to_string()),
                            "eq" => args.insert(tokens[0].to_string(), "je".to_string()),
                            "ne" => args.insert(tokens[0].to_string(), "jne".to_string()),
                            _ => args.insert(tokens[0].to_string(), "je".to_string()),
                        };
                    }
                }
                else if inner_line.starts_with("\tstore") {
                    assembly_code.push_str(&format!("\tpopq {}\n", args[tokens[2]].to_string()));
                    assembly_code.push_str(&format!("\tmovq {}, {}\n", args[tokens[2]].to_string(), args[tokens[4]]));
                }
                else if inner_line.starts_with("\tbr i1") {
                    let mut label = tokens[4].replace(".", "");
                    label = label.replace("%", "");
                    assembly_code.push_str(&format!("\t{} {}\n", args[tokens[2]], label));
                }
            }
        }
        else if line.starts_with("if") {
            if line.ends_with("else:") {
                
            }
            if line.ends_with("then:") {

            }
            if line.ends_with("end:") {

            }
        }
        else if line.starts_with("while") {
            if line.ends_with("cond:") {

            }
            if line.ends_with("end:") {

            }
            if line.ends_with("body:") {
                
            }
        }
    }
    // println!("{:?}", args);
    // for line in lines.collect::<Vec<_>>() {
    //     if line == "entry:" {
    //         lines.next();
    //         println!("{:?}", lines);
    //     }
    // }

    // assembly_code.push_str(&format!("\n.section .note.GNU-stack,{},@progbits", ""));
    assembly_code.push_str(".section .note.GNU-stack,\"\",@progbits\n");
    assembly_code
}

// fn gen_x86_stmts() {
//     d
// }