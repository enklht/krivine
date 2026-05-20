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
    Arg(usize),
}

type Stack = Vec<StackItem>;
type Heap = Vec<Closure>;

#[derive(Debug)]
pub struct Machine {
    closure: Closure,
    stack: Stack,
    closure_heap: Heap,
    location_heap: Vec<usize>,
}

fn get_term(
    term: usize,
    env: &Environment,
    closure_heap: &Heap,
    location_heap: &Vec<usize>,
    arena: &mut Vec<Term>,
    depth: usize,
) -> usize {
    match *arena.get(term) {
        Term::Var(x) if x < depth => term,
        Term::Var(x) => {
            let r = env[env.len() + depth - x - 1];
            let l = location_heap[r];
            get_term(
                closure_heap[l].term,
                &closure_heap[l].env,
                closure_heap,
                location_heap,
                arena,
                0,
            )
        }
        Term::App(m, n) => {
            let m = get_term(m, env, closure_heap, location_heap, arena, depth);
            let n = get_term(n, env, closure_heap, location_heap, arena, depth);
            arena.alloc(Term::App(m, n))
        }
        Term::Abs(m) => {
            let m = get_term(m, env, closure_heap, location_heap, arena, depth + 1);
            arena.alloc(Term::Abs(m))
        }
    }
}

impl Machine {
    pub fn new(term: usize) -> Self {
        Machine {
            closure: Closure::new(term),
            stack: Stack::new(),
            closure_heap: Heap::new(),
            location_heap: Vec::new(),
        }
    }

    pub fn step(&mut self, arena: &Vec<Term>) -> bool {
        let Closure { term, env } = &mut self.closure;
        match *arena.get(*term) {
            Term::Var(x) => {
                let r = env[env.len() - x - 1];
                let l = self.location_heap[r];
                let c = self.closure_heap[l].clone();

                if matches!(arena.get(c.term), Term::Abs(_)) {
                    self.closure = c;
                    return true;
                }

                self.closure = c;
                if let Some(StackItem::Mark(l_prime)) = self.stack.last() {
                    self.location_heap[r] = *l_prime;
                } else {
                    self.stack.push(StackItem::Mark(l));
                }
            }
            Term::App(m, n) => {
                if let Term::Var(x) = arena.get(n) {
                    let r = env[env.len() - x - 1];

                    *term = m;
                    self.stack.push(StackItem::Arg(r));
                } else {
                    let l = self.closure_heap.len();
                    let r = self.location_heap.len();

                    *term = m;
                    self.stack.push(StackItem::Arg(r));
                    self.closure_heap.push(Closure {
                        term: n,
                        env: env.clone(),
                    });
                    self.location_heap.push(l);
                }
            }
            Term::Abs(m) => match self.stack.pop() {
                Some(StackItem::Arg(r)) => {
                    *term = m;
                    env.push(r);
                }
                Some(StackItem::Mark(l)) if matches!(arena.get(m), Term::Abs(_)) => {
                    debug_assert!(l < self.closure_heap.len());
                    self.closure_heap[l] = self.closure.clone();
                }
                _ => return false,
            },
        }
        true
    }

    pub fn run(mut self, arena: &mut Vec<Term>) -> usize {
        while self.step(arena) {}
        get_term(
            self.closure.term,
            &self.closure.env,
            &self.closure_heap,
            &self.location_heap,
            arena,
            0,
        )
    }
}
