use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_code(reader: &mut dyn BufRead) -> Vec<(String, isize)> {
    let mut code = Vec::new();
    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        let mut line = l.split_whitespace();
        code.push((
            line.next().unwrap().to_string(),
            line.next().unwrap().parse().unwrap(),
        ));
    });

    code
}

#[derive(Debug, Clone, Copy)]
struct State {
    pc: usize,
    acc: isize,
}

fn execute_instr(op: &str, arg: isize, state: &mut State, trace: &mut HashSet<usize>) {
    trace.insert(state.pc);
    match op {
        "acc" => state.acc += arg,
        "jmp" => {
            state.pc = if arg < 0 {
                state.pc - arg.abs() as usize
            } else {
                state.pc + arg as usize
            }
        }
        _ => (),
    }

    if op != "jmp" {
        state.pc += 1;
    }
}

fn patch_instr(code_ref: &RefCell<Vec<(String, isize)>>, &State{pc, ..}: &State, instr: &str) {
    code_ref.borrow_mut()[pc].0 = instr.to_string();
}

fn execute_fixup(
    code_ref: &RefCell<Vec<(String, isize)>>,
    patch: bool,
    state: State,
    trace: Option<&HashSet<usize>>,
) -> State {
    let mut state = state;
    let exit_addr = code_ref.borrow().len();
    let mut trace = match trace {
        Some(t) => t.clone(),
        None => HashSet::new(),
    };

    loop {
        if trace.contains(&state.pc) || state.pc >= exit_addr {
            return state;
        }

        let (op, arg) = code_ref.borrow()[state.pc].clone();
        if patch {
            match op.as_str() {
                instr @ "jmp" => {
                    patch_instr(code_ref, &state, "nop");
                    let result = execute_fixup(code_ref, false, state, Some(&trace));
                    if result.pc >= exit_addr {
                        return result;
                    } else {
                        patch_instr(code_ref, &state, instr);
                        execute_instr(instr, arg, &mut state, &mut trace);
                    }
                }
                instr @ "nop" => {
                    patch_instr(code_ref, &state, "jmp");
                    let result = execute_fixup(code_ref, false, state, Some(&trace));
                    if result.pc >= exit_addr {
                        return result;
                    } else {
                        patch_instr(code_ref, &state, instr);
                        execute_instr(instr, arg, &mut state, &mut trace);
                    }
                }
                instr @ "acc" => execute_instr(instr, arg, &mut state, &mut trace),
                _ => (),
            }
        } else {
            execute_instr(op.as_str(), arg, &mut state, &mut trace);
        }
    }
}

pub fn run() {
    let mut br = BufReader::new(File::open("src/aoc08_input.txt").unwrap());
    let code = load_code(&mut br);

    let code_ref = RefCell::new(code);
    let state = execute_fixup(&code_ref, true, State { pc: 0, acc: 0 }, None);
    println!("part2: {:?}", state.acc);
}
