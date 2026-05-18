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

type Environment = Vec<usize>;

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

#[derive(Debug)]
enum StackItem {
    Mark(usize),
    Arg(Closure),
}

type Stack = Vec<StackItem>;
type Heap = Vec<Closure>;

#[derive(Debug)]
pub struct Machine {
    closure: Closure,
    stack: Stack,
    heap: Heap,
}

fn get_term(
    term: usize,
    env: &Environment,
    heap: &Heap,
    arena: &mut Vec<Term>,
    depth: usize,
) -> usize {
    match *arena.get(term) {
        Term::Var(x) if x < depth => term,
        Term::Var(x) => {
            let l = env[env.len() + depth - x - 1];
            get_term(heap[l].term, &heap[l].env, heap, arena, 0)
        }
        Term::App(m, n) => {
            let m = get_term(m, env, heap, arena, depth);
            let n = get_term(n, env, heap, arena, depth);
            arena.alloc(Term::App(m, n))
        }
        Term::Abs(m) => {
            let m = get_term(m, env, heap, arena, depth + 1);
            arena.alloc(Term::Abs(m))
        }
    }
}

impl Machine {
    pub fn new(term: usize) -> Self {
        Machine {
            closure: Closure::new(term),
            stack: Stack::new(),
            heap: Heap::new(),
        }
    }

    pub fn step(&mut self, arena: &Vec<Term>) -> bool {
        let Closure { term, env } = &mut self.closure;
        match *arena.get(*term) {
            Term::Var(x) => {
                let l = env[env.len() - x - 1];
                match arena.get(self.heap[l].term) {
                    Term::Abs(_) => {
                        self.closure = self.heap[l].clone();
                    }
                    _ => {
                        self.closure = self.heap[l].clone();
                        self.stack.push(StackItem::Mark(l));
                    }
                }
            }
            Term::App(m, n) => {
                self.stack.push(StackItem::Arg(Closure {
                    term: n,
                    env: env.clone(),
                }));
                *term = m;
            }
            Term::Abs(m) => match self.stack.pop() {
                Some(StackItem::Arg(c)) => {
                    let l = self.heap.len();
                    *term = m;
                    env.push(l);
                    self.heap.push(c);
                }
                Some(StackItem::Mark(l)) if matches!(arena.get(m), Term::Abs(_)) => {
                    debug_assert!(l < self.heap.len());
                    self.heap[l] = self.closure.clone();
                }
                _ => return false,
            },
        }
        true
    }

    pub fn run(mut self, arena: &mut Vec<Term>) -> usize {
        while self.step(arena) {}
        get_term(self.closure.term, &self.closure.env, &self.heap, arena, 0)
    }
}
