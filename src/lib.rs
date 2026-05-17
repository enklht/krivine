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
    fn display(&self, term: usize) -> TermDisplay<'_>;
}

impl Arena for Vec<Term> {
    fn alloc(&mut self, term: Term) -> usize {
        self.push(term);
        self.len() - 1
    }
    fn display(&self, term: usize) -> display::TermDisplay<'_> {
        display::TermDisplay { arena: self, term }
    }
}

#[derive(Debug)]
pub struct Machine {
    closure: Closure,
    stack: Stack,
    arena: Vec<Term>,
}

impl Machine {
    pub fn new(term: usize, arena: Vec<Term>) -> Self {
        Machine {
            closure: Closure {
                term,
                env: Vec::new(),
            },
            stack: Stack::new(),
            arena,
        }
    }

    pub fn from_clause(closure: Closure, arena: Vec<Term>) -> Self {
        Machine {
            closure,
            stack: Stack::new(),
            arena,
        }
    }

    pub fn get(&self, term: usize) -> &Term {
        &self.arena[term]
    }

    pub fn eval1(&mut self) -> Option<()> {
        let Closure { term, env } = &mut self.closure;
        match self.arena[*term] {
            Term::Var(k) => {
                self.closure = env[env.len() - k - 1].clone();
            }
            Term::App(m, n) => {
                *term = m;
                let env = env.clone();
                self.stack.push(Closure { term: n, env });
            }
            Term::Abs(m) => {
                let c = self.stack.pop()?;
                *term = m;
                env.push(c);
            }
        }
        Some(())
    }

    pub fn eval(mut self) -> (Closure, Vec<Term>) {
        while self.eval1().is_some() {}
        (self.closure, self.arena)
    }
}
