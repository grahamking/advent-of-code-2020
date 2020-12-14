use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::replace;

use anyhow::anyhow;

fn main() {
    let mut p = Program::load("input");
    p.fix();
}

#[derive(Debug)]
struct Program {
    code: Vec<Op>,
    pc: i32,
    acc: i32,

    steps: i32,
    fix_pos: usize,
    prev: Option<Op>,
}

impl Program {
    fn load(filename: &str) -> Program {
        let f = File::open(filename).unwrap();
        let mut code = Vec::new();
        for line in BufReader::new(f).lines().filter_map(|x| x.ok()) {
            code.push(line.into());
        }
        Program {
            code,
            pc: 0,
            acc: 0,
            steps: 0,
            fix_pos: 0,
            prev: None,
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.steps = 0;
        for cmd in self.code.iter_mut() {
            cmd.visited = false;
        }
        // undo our mutation
        if self.prev.is_some() {
            let _ = replace(&mut self.code[self.fix_pos - 1], self.prev.take().unwrap());
        }
    }

    fn fix(&mut self) {
        loop {
            match self.run() {
                Ok(_) => {
                    break; // success
                }
                Err(_) => {
                    self.reset();
                    self.mutate();
                }
            }
        }
        println!("steps: {}, pc: {}, acc: {}", self.steps, self.pc, self.acc);
    }

    fn run(&mut self) -> Result<(), anyhow::Error> {
        while self.pc < self.code.len() as i32 {
            if let Err(err) = self.step() {
                //println!("abort: {}", err);
                return Err(err);
            }
        }
        Ok(())
    }

    fn step(&mut self) -> Result<(), anyhow::Error> {
        self.steps += 1;
        let op = self.code.get_mut(self.pc as usize).unwrap();
        if op.visited {
            return Err(anyhow!("infinite loop detected"));
        }
        op.visited = true;
        match op.cmd {
            Cmd::Nop => {
                self.pc += 1;
            }
            Cmd::Acc => {
                self.acc += op.val;
                self.pc += 1;
            }
            Cmd::Jmp => {
                self.pc += op.val;
            }
        }
        Ok(())
    }

    fn mutate(&mut self) {
        let mut next = None;
        while self.fix_pos < self.code.len() {
            // loop is only to skip acc
            let op = self.code.get(self.fix_pos).unwrap();
            match op.cmd {
                Cmd::Acc => {
                    self.fix_pos += 1;
                    continue; // don't mutate acc
                }
                Cmd::Nop => {
                    next = Some(Op {
                        cmd: Cmd::Jmp,
                        visited: false,
                        val: op.val,
                    });
                    break;
                }
                Cmd::Jmp => {
                    next = Some(Op {
                        cmd: Cmd::Nop,
                        visited: false,
                        val: op.val,
                    });

                    break;
                }
            }
        }
        self.prev = Some(replace(&mut self.code[self.fix_pos], next.unwrap()));
        self.fix_pos += 1;
    }
}

#[derive(Debug)]
struct Op {
    cmd: Cmd,
    visited: bool,
    val: i32,
}

#[derive(Debug)]
enum Cmd {
    Nop,
    Acc,
    Jmp,
}

impl From<String> for Op {
    fn from(s: String) -> Self {
        let mut parts = s.split(" ");
        let cmd = parts.next().unwrap();
        let val = parts.next().unwrap().parse::<i32>().unwrap();
        let visited = false;
        match cmd {
            "nop" => Op {
                cmd: Cmd::Nop,
                visited,
                val,
            },
            "acc" => Op {
                cmd: Cmd::Acc,
                visited,
                val,
            },
            "jmp" => Op {
                cmd: Cmd::Jmp,
                visited,
                val,
            },
            _ => {
                panic!("unknown cmd {}", cmd);
            }
        }
    }
}
