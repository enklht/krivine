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

#[derive(Debug, Clone, Copy)]
enum Environment {
    Nil,
    Cons { parent: usize, value: usize },
}

impl Environment {
    fn get(&self, mut i: usize, env_arena: &[Environment]) -> usize {
        let mut current = *self;
        loop {
            let Environment::Cons { parent, value } = current else {
                panic!("invalid index");
            };

            if i == 0 {
                return value;
            }

            i -= 1;
            current = env_arena[parent];
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Closure {
    term: usize,
    env: usize,
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
    env_arena: Vec<Environment>,
}

fn get_term(
    term: usize,
    env: usize,
    machine: &Machine,
    arena: &mut Vec<Term>,
    depth: usize,
) -> usize {
    match *arena.get(term) {
        Term::Var(x) if x < depth => term,
        Term::Var(x) => {
            let env = machine.env_arena[env];
            let r = env.get(x - depth, &machine.env_arena);
            let l = machine.location_heap[r];
            get_term(
                machine.closure_heap[l].term,
                machine.closure_heap[l].env,
                machine,
                arena,
                0,
            )
        }
        Term::App(m, n) => {
            let m = get_term(m, env, machine, arena, depth);
            let n = get_term(n, env, machine, arena, depth);
            arena.alloc(Term::App(m, n))
        }
        Term::Abs(m) => {
            let m = get_term(m, env, machine, arena, depth + 1);
            arena.alloc(Term::Abs(m))
        }
    }
}

fn alloc_env(env: Environment, arena: &mut Vec<Environment>) -> usize {
    arena.push(env);
    arena.len() - 1
}

impl Machine {
    pub fn new(term: usize) -> Self {
        Machine {
            closure: Closure { term, env: 0 },
            stack: Stack::new(),
            closure_heap: Heap::new(),
            location_heap: Vec::new(),
            env_arena: vec![Environment::Nil],
        }
    }

    pub fn step(&mut self, arena: &Vec<Term>) -> bool {
        let Closure { term, env } = &mut self.closure;
        match *arena.get(*term) {
            Term::Var(x) => {
                let env = self.env_arena[*env];
                let r = env.get(x, &self.env_arena);
                let l = self.location_heap[r];
                let c = self.closure_heap[l];

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
                if let &Term::Var(x) = arena.get(n) {
                    let env = self.env_arena[*env];
                    let r = env.get(x, &self.env_arena);

                    *term = m;
                    self.stack.push(StackItem::Arg(r));
                } else {
                    let l = self.closure_heap.len();
                    let r = self.location_heap.len();

                    *term = m;
                    self.stack.push(StackItem::Arg(r));
                    self.closure_heap.push(Closure { term: n, env: *env });
                    self.location_heap.push(l);
                }
            }
            Term::Abs(m) => match self.stack.pop() {
                Some(StackItem::Arg(r)) => {
                    *term = m;
                    let new_env = alloc_env(
                        Environment::Cons {
                            parent: *env,
                            value: r,
                        },
                        &mut self.env_arena,
                    );
                    *env = new_env;
                }
                Some(StackItem::Mark(l)) if matches!(arena.get(m), Term::Abs(_)) => {
                    debug_assert!(l < self.closure_heap.len());
                    self.closure_heap[l] = self.closure;
                }
                _ => return false,
            },
        }
        true
    }

    pub fn run(&mut self, arena: &mut Vec<Term>) -> usize {
        while self.step(arena) {}
        get_term(self.closure.term, self.closure.env, self, arena, 0)
    }
}
