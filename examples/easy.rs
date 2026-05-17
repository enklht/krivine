use krivine::*;

fn main() {
    let mut arena = Vec::new();
    let zero = term!(arena, (abs (abs 1)));
    let succ = term!(arena, (abs (abs (abs 0 2))));

    let term = term!(arena, succ zero);

    let machine = Machine::new(term, arena);

    let (closure, arena) = machine.eval();
    println!("term: {}", arena.display(closure.term));
    println!("env:  {:?}", closure.env);
}
