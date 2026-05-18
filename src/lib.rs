use crate::display::TermDisplay;

#[macro_use]
mod macros;
pub mod display;

#[derive(Debug, Clone, Copy)]
pub enum Term {
    Var(usize),
    Abs(usize),
    App(usize, usize),
}

type Environment = Vec<Closure>;

#[derive(Debug, Clone)]
pub struct Closure {
    pub term: usize,
    pub env: Environment,
}

impl Closure {
    pub fn new(term: usize) -> Self {
        Closure {
            term,
            env: Vec::new(),
        }
    }
}

type Stack = Vec<Closure>;

pub trait Arena {
    fn alloc(&mut self, term: Term) -> usize;
    fn get(&self, term: usize) -> &Term;
    fn display(&self, term: usize) -> TermDisplay<'_>;
}

impl Arena for Vec<Term> {
    fn alloc(&mut self, term: Term) -> usize {
        self.push(term);
        self.len() - 1
    }

    fn get(&self, term: usize) -> &Term {
        &self[term]
    }

    fn display(&self, term: usize) -> TermDisplay<'_> {
        TermDisplay { term, arena: self }
    }
}

#[derive(Debug)]
pub struct Machine {
    closure: Closure,
    stack: Stack,
}

impl Machine {
    pub fn new(closure: Closure) -> Self {
        Machine {
            closure,
            stack: Stack::new(),
        }
    }

    pub fn step(&mut self, arena: &Vec<Term>) -> bool {
        let Closure { term, env } = &mut self.closure;
        match *arena.get(*term) {
            Term::Var(k) => {
                self.closure = env[env.len() - k - 1].clone();
            }
            Term::App(m, n) => {
                self.stack.push(Closure {
                    term: n,
                    env: env.clone(),
                });
                *term = m;
            }
            Term::Abs(m) => {
                let Some(c) = self.stack.pop() else {
                    return false;
                };
                *term = m;
                env.push(c);
            }
        }
        true
    }

    pub fn run(mut self, arena: &Vec<Term>) -> Closure {
        while self.step(arena) {}
        self.closure
    }
}
