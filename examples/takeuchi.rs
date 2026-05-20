use krivine::*;

pub fn scott_to_int(mut term: usize, arena: &mut Vec<Term>) -> Option<u32> {
    let mut n = 0;

    let zero = term!(arena, (abs (abs 1)));

    loop {
        term = Machine::new(term).run(arena);

        let Term::Abs(t1) = arena.get(term) else {
            return None;
        };
        let Term::Abs(t2) = arena.get(*t1) else {
            return None;
        };
        match arena.get(*t2) {
            Term::Var(1) => return Some(n),
            Term::App(s, _) if matches!(arena.get(*s), Term::Var(0)) => {
                n += 1;
                term = term!(arena, term zero (abs 0));
            }
            _ => return None,
        }
    }
}

pub fn bool_to_rust(term: usize, arena: &mut Vec<Term>) -> Option<bool> {
    let term = Machine::new(term).run(arena);

    let Term::Abs(t1) = arena.get(term) else {
        return None;
    };
    let Term::Abs(t2) = arena.get(*t1) else {
        return None;
    };

    match arena.get(*t2) {
        Term::Var(1) => Some(true),
        Term::Var(0) => Some(false),
        _ => None,
    }
}

pub fn scott_from_int(n: u32, zero: usize, succ: usize, arena: &mut Vec<Term>) -> usize {
    let mut term = zero;
    for _ in 0..n {
        term = term!(arena, succ term);
    }
    term
}

fn main() {
    let mut arena = Vec::new();

    let zero = term!(arena, (abs (abs 1)));
    let succ = term!(arena, (abs (abs (abs 0 2))));

    let t_true = zero;
    let t_false = term!(arena, (abs (abs 0)));

    let pred = term!(arena, (abs 0 zero (abs 0)));
    let iszero = term!(arena, (abs 0 t_true (abs t_false)));

    let y = term!(
        arena,
        (abs (abs 1 (0 0)) (abs 1 (0 0)))
    );

    let sub = term!(
        arena,
        y (abs (abs (abs 0 1 (abs 3 (pred 2) 0))))
    );

    let leq = term!(
        arena,
        y (abs (abs (abs 1 t_true (abs 1 t_false (abs 4 1 0)))))
    );

    let lt = term!(arena, (abs (abs leq (succ 1) 0)));

    let tak = term!(
        arena,
        y (abs (abs (abs (abs lt 1 2 (3 (3 (pred 2) 1 0) (3 (pred 1) 0 2) (3 (pred 0) 2 1)) 0))))
    );

    let twelve = scott_from_int(12, zero, succ, &mut arena);
    let ten = scott_from_int(10, zero, succ, &mut arena);
    let six = scott_from_int(6, zero, succ, &mut arena);
    let seven = scott_from_int(7, zero, succ, &mut arena);

    let tak_term = term!(arena, tak twelve ten six);
    let tak_nf = Machine::new(tak_term).run(&mut arena);

    let test = term!(arena, iszero (sub tak_nf seven));
    println!("(=0 (- (tak 12 10 6) 7))");
    let test_value = bool_to_rust(test, &mut arena).expect("not a boolean");
    println!("= {}", test_value);
    assert!(test_value);
}
