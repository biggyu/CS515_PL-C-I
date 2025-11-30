use std::collections::HashMap;
use std::iter::Peekable;

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
    let mut rsp: usize = 0;
    let mut buffer = None;
    // let mut ifthen = String::new();
    // let mut ifelse = String::new();
    // let mut whilecond = String::new();
    // let mut whilebody = String::new();
    while let Some(line) = lines.next() {
        if line.starts_with("entry:") {
            // let mut rsp = 0;
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
            // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
            //     if !inner_line.starts_with("\tstore") {
            //         buffer = Some(inner_line);
            //         break;
            //     }
            //     gen_x86_stmts(&mut args, inner_line, &mut lines, &mut assembly_code);
            //     // let tmp = inner_line.replace(",", "");
            //     // let mut tokens: Vec<_> = tmp.split_whitespace().collect();
            //     // if tokens[2].contains("%") {
            //     //     assembly_code.push_str(&format!("\tmovq {}, {}\n", args[tokens[2]], args[tokens[4]]));
            //     // }
            //     // else {
            //     //     // let value = &tokens[2][..end];
            //     //     assembly_code.push_str(&format!("\tmovq ${}, {}\n", tokens[2], args[tokens[4]]));
            //     // }
            // }
            while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                if !inner_line.starts_with("\t") {
                    buffer = Some(inner_line);
                    break;
                }
                assembly_code.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
            }
        }
        else if line.starts_with("if") {
            if line.ends_with("then:") {
                let mut labels: Vec<_> = line.split(".").collect();
                let mut lookahead = lines.clone();
                // println!("labels: {:?}", labels);
                while let Some(line_ahead) = lookahead.next() {
                    if line_ahead.ends_with("else:") {
                        let mut tmp_labels: Vec<_> = line_ahead.split(".").collect();
                        // println!("{:?}", tmp_labels);
                        if labels[0] == tmp_labels[0] {
                            assembly_code.push_str("\n");
                            while let Some(ll_ahead) = lookahead.next() {
                                if !ll_ahead.starts_with("\t") {
                                    buffer = Some(ll_ahead);
                                    break;
                                }
                                assembly_code.push_str(&gen_x86_stmts(&mut args, ll_ahead, &mut lines, rsp));
                            }
                            break;
                        }
                    }
                }
                // ifthen = String::new();
                // let mut label = line.replace(".", "");
                // label = label.replace("%", "");
                // ifthen.push_str(&format!("{}\n", label));
                assembly_code.push_str(&format!("\n{}\n", labels.join("").replace("%", "")));
                while let Some(inner_line) = lines.next() {
                // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                    if !inner_line.starts_with("\t") {
                        buffer = Some(inner_line);
                        break;
                    }
                    assembly_code.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
                }
            }
            // if line.ends_with("else:") {
            //     // let mut label = line.replace(".", "");
            //     // label = label.replace("%", "");
            //     // assembly_code.push_str(&format!("{}\n", label));
            //     ifelse = String::new();
            //     while let Some(inner_line) = lines.next() {
            //     // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
            //         if !inner_line.starts_with("\t") {
            //             buffer = Some(inner_line);
            //             break;
            //         }
            //         ifelse.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
            //     }
            // }
            if line.ends_with("end:") {
                // assembly_code.push_str("\n");
                // assembly_code.push_str(&ifelse);
                // assembly_code.push_str("\n");
                // assembly_code.push_str(&ifthen);
                // assembly_code.push_str("\n");
                let mut label = line.replace(".", "");
                label = label.replace("%", "");
                assembly_code.push_str(&format!("\n{}\n", label));
                while let Some(inner_line) = lines.next() {
                // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                    if !inner_line.starts_with("\t") {
                        buffer = Some(inner_line);
                        break;
                    }
                    assembly_code.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
                }
            }
        }
        else if line.starts_with("while") {
            if line.ends_with("cond:") {
                // whilecond = String::new();
                // let mut label = line.replace(".", "");
                // label = label.replace("%", "");
                let mut labels: Vec<_> = line.split(".").collect();
                assembly_code.push_str(&format!("\n{}\n", labels.join("").replace("%", "")));
                // whilecond.push_str(&format!("{}\n", label));
                while let Some(inner_line) = lines.next() {
                    // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                    if !inner_line.starts_with("\t") {
                        buffer = Some(inner_line);
                        break;
                    }
                    assembly_code.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
                    // whilecond.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
                }
                let mut lookahead = lines.clone();
                while let Some(line_ahead) = lookahead.next() {
                    if line_ahead.ends_with("end:") {
                        let mut tmp_labels: Vec<_> = line_ahead.split(".").collect();
                        // println!("{:?}", tmp_labels);
                        if labels[0] == tmp_labels[0] {
                            assembly_code.push_str("\n");
                            while let Some(ll_ahead) = lookahead.next() {
                                if !ll_ahead.starts_with("\t") {
                                    buffer = Some(ll_ahead);
                                    break;
                                }
                                assembly_code.push_str(&gen_x86_stmts(&mut args, ll_ahead, &mut lines, rsp));
                            }
                            break;
                        }
                    }
                }
            }
            if line.ends_with("body:") {
                // whilebody = String::new();
                let mut label = line.replace(".", "");
                label = label.replace("%", "");
                assembly_code.push_str(&format!("{}\n", label));
                while let Some(inner_line) = lines.next() {
                // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
                    if !inner_line.starts_with("\t") {
                        buffer = Some(inner_line);
                        break;
                    }
                    assembly_code.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
                }
            }
            // if line.ends_with("end:") {
            //     // let mut label = line.replace(".", "");
            //     // label = label.replace("%", "");
            //     // assembly_code.push_str(&format!("{}:\n", label));
            //     // assembly_code.push_str("\n");
            //     // assembly_code.push_str(&whilecond);
            //     // assembly_code.push_str("\n");
            //     while let Some(inner_line) = lines.next() {
            //     // while let Some(inner_line) = buffer.take().or_else(|| lines.next()) {
            //         if !inner_line.starts_with("\t") {
            //             buffer = Some(inner_line);
            //             break;
            //         }
            //         assembly_code.push_str(&gen_x86_stmts(&mut args, inner_line, &mut lines, rsp));
            //     }
            //     assembly_code.push_str("\n");
            //     assembly_code.push_str(&whilebody);
            // }
        }
    }
    assembly_code.push_str(".section .note.GNU-stack,\"\",@progbits\n");
    assembly_code
}

fn gen_x86_stmts<'a>(args: &mut HashMap<String, String>, inner_line: &str, lines: &mut Peekable<impl Iterator<Item = &'a str>>, rsp: usize) -> String {
// fn gen_x86_stmts<'a>(args: &mut HashMap<String, String>, inner_line: &str, lines: &mut Peekable<impl Iterator<Item = &'a str>>, assembly_code: &mut String) {
// fn gen_x86_stmts<'a>(args: &mut HashMap<String, String>, inner_line: &str, lines: &mut Peekable<impl Iterator<Item = &'a str>>, assembly_code: &mut String) {
    let mut stmts_assm = String::new();
    let tmp = inner_line.replace(",", "");
    let mut tokens: Vec<_> = tmp.split_whitespace().collect();
    if inner_line.starts_with("\t%") {
        if inner_line.contains("= load") {
            stmts_assm.push_str(&format!("\tmovq {}, %rbx\n", args[tokens[5]]));
            args.insert(tokens[0].to_string(), "%rbx".to_string());
            // if let Some(operand) = lines.peek() {
            //     if operand.starts_with("\t%") && operand.contains("= load") {
            //         tokens = operand.split_whitespace().collect();
            //         stmts_assm.push_str(&format!("\tmovq {}, %rax\n", args[tokens[5]]));
            //         stmts_assm.push_str("\tpushq %rax\n");
            //         args.insert(tokens[0].to_string(), "%rax".to_string());
            //         lines.next();
            // }
            //     }
            stmts_assm.push_str("\tpushq %rbx\n");
        }
        else if inner_line.contains("= mul") {
            let mut dest = "%r10".to_string();
            if tokens[4].contains("%") && tokens[5].contains("%") {
                stmts_assm.push_str("\tpopq %rax\n");
                stmts_assm.push_str(&format!("\tpopq {}\n", dest));
                stmts_assm.push_str(&format!("\tmulq {}\n", dest));
                stmts_assm.push_str("\tpushq %rax\n");
                args.insert(tokens[0].to_string(), "%rax".to_string());
            }
            else if tokens[4].contains("%") {
                stmts_assm.push_str(&format!("\tpopq {}\n", dest));
                stmts_assm.push_str(&format!("\timulq ${}, {}, {}\n", tokens[5], dest, dest));
                stmts_assm.push_str(&format!("\tpushq {}\n", dest));
                args.insert(tokens[0].to_string(), dest.clone());
            }
            else if tokens[5].contains("%") {
                stmts_assm.push_str(&format!("\tpopq {}\n", dest));
                stmts_assm.push_str(&format!("\timulq ${}, {}, {}\n", tokens[4], dest, dest));
                stmts_assm.push_str(&format!("\tpushq {}\n", dest));
                args.insert(tokens[0].to_string(), dest.clone());
            }
            else {
                let lhs = tokens[4].parse::<i64>().unwrap_or(0);
                let rhs = tokens[5].parse::<i64>().unwrap_or(0);
                let result = lhs * rhs;
                stmts_assm.push_str(&format!("\tmovq ${}, {}\n", result, dest));
                stmts_assm.push_str(&format!("\tpushq {}\n", dest));
                args.insert(tokens[0].to_string(), dest.clone());
            }
            // // stmts_assm.push_str(&format!("\t{} {}, {}\n", inst, src, dest));
            // stmts_assm.push_str(&format!("\tpushq {}\n", dest));
            // args.insert(tokens[0].to_string(), dest.to_string());
        }
        else if inner_line.contains("= add") {
            let mut src = String::new();
            let mut dest = String::new();
            if tokens[4].contains("%") {
                src = "%r10".to_string();
                stmts_assm.push_str(&format!("\tpopq {}\n", src));
                if tokens[5].contains("%") {
                    dest = "%r11".to_string();
                    stmts_assm.push_str(&format!("\tpopq {}\n", dest));
                }
                else {
                    dest = src;
                    src = format!("${}", tokens[5]);
                }
            }
            else {
                src = format!("${}", tokens[4]);
                dest = "%r11".to_string();
                if tokens[5].contains("%") {
                    stmts_assm.push_str(&format!("\tpopq {}\n", dest));
                }
                else {
                    // dest = "%r11".to_string();
                    stmts_assm.push_str(&format!("\tmovq {}, {}\n", src, dest));
                    src = format!("${}", tokens[5]);
                }
            }
            stmts_assm.push_str(&format!("\taddq {}, {}\n", src, dest));
            stmts_assm.push_str(&format!("\tpushq {}\n", dest));
            args.insert(tokens[0].to_string(), dest.to_string());
        }
        else if inner_line.contains("= icmp") {
            let mut lhs = String::new();
            let mut rhs = String::new();
            if tokens[5].contains("%") {
                if args[tokens[5]] == "%rbx" || args[tokens[5]] == "%rax" || args[tokens[5]] == "%r10" {
                    stmts_assm.push_str("\tpopq %rax\n");
                    lhs = "%rax".to_string();
                    // stmts_assm.push_str(&format!("\tpopq {}\n", args[tokens[5]].clone()));
                }
                else {
                    lhs = args[tokens[5]].clone();
                }
                if tokens[6].contains("%") {
                    if args[tokens[6]] == "%rbx" || args[tokens[6]] == "%rax" || args[tokens[5]] == "%r10" {
                        stmts_assm.push_str("\tpopq %rbx\n");
                        lhs = "%rbx".to_string();
                        rhs = "%rax".to_string();
                        // stmts_assm.push_str(&format!("\tpopq {}\n", args[tokens[6]].clone()));
                    }
                    else {
                        rhs = args[tokens[6]].clone();
                    }
                }
                else {
                    stmts_assm.push_str(&format!("\tmovq ${}, %rbx\n", tokens[6].clone()));
                    rhs = "%rbx".to_string()
                    // lhs = format!("${}", tokens[6].clone());
                    // args.insert(tokens[6].to_string(), lhs.to_string());
                }
            }
            else {
                stmts_assm.push_str(&format!("\tmovq ${}, %rax\n", tokens[5].to_string()));
                lhs = "%rax".to_string();
                if tokens[6].contains("%") {
                    args.insert(tokens[5].to_string(), lhs.to_string());   
                    if args[tokens[6]] == "%rbx" || args[tokens[6]] == "%rax" {
                        stmts_assm.push_str("\tpopq %rbx\n");
                        // stmts_assm.push_str(&format!("\tpopq {}\n", args[tokens[6]].clone()));
                        // rhs = args[tokens[6]].clone();
                    }
                }
                else {
                    stmts_assm.push_str(&format!("\tmovq ${}, %rbx\n", tokens[6].to_string()));
                    // lhs = format!("${}", tokens[6].to_string());
                }
                rhs = "%rbx".to_string();
            }
            // if tokens[6].contains("%") {
            //     if args[tokens[5]] == "%rbx" || args[tokens[5]] == "%rax" {
            //         stmts_assm.push_str(&format!("\tpopq {}\n", args[tokens[6]].clone()));
            //     }
            //     rhs = args[tokens[6]].clone();
            // }
            // else {
            //     rhs = "%r10".to_string();
            //     stmts_assm.push_str(&format!("\tmovq ${}, {}\n", tokens[6], rhs));
            //     // rhs = format!("${}",tokens[6].to_string());
            // }
            stmts_assm.push_str(&format!("\tcmpq {}, {}\n", rhs, lhs));
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
        if tokens[2].contains("%") {
            if tokens[2].contains("t") {
                stmts_assm.push_str(&format!("\tpopq {}\n", args[tokens[2]].to_string()));
            }
        }
        else {
            args.insert(tokens[2].to_string(), format!("${}", tokens[2]));
        }
        stmts_assm.push_str(&format!("\tmovq {}, {}\n", args[tokens[2]].to_string(), args[tokens[4]]));
    }
    else if inner_line.starts_with("\tbr") {
        if inner_line.contains("i1") {
            if tokens[2].contains("%") {
                let mut label = tokens[4].replace(".", "");
                label = label.replace("%", "");
                stmts_assm.push_str(&format!("\t{} {}\n", args[tokens[2]], label));
            }
            else {
                if tokens[2].parse().unwrap() {
                    let mut label = tokens[4].replace(".", "");
                    label = label.replace("%", "");
                    stmts_assm.push_str(&format!("\tjmp {}\n", label));
                }
                // else {
                //     let mut label = tokens[6].replace(".", "");
                //     label = label.replace("%", "");
                //     stmts_assm.push_str(&format!("\tjmp {}\n", label));
                // }
            }
        }
        else {
            let mut label = tokens[2].replace(".", "");
            label = label.replace("%", "");
            stmts_assm.push_str(&format!("\tjmp {}\n", label));
        }
    }
    else if inner_line.starts_with("\tret") {
        if args[tokens[2]] == "%rbx" || args[tokens[2]] == "%rax" {
            stmts_assm.push_str(&format!("\tpopq {}\n", args[tokens[2]]));
        }
        stmts_assm.push_str(&format!("\tmovq {}, %rax\n", args[tokens[2]]));
        stmts_assm.push_str(&format!("\n\taddq ${}, %rsp\n\tpopq %rbx\n\tpopq %rbp\n\tret\n", rsp));
    }
    stmts_assm
}