use krivine::*;

fn scott_to_bool(term: usize, arena: &mut Vec<Term>) -> Option<bool> {
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
    let one = term!(arena, succ zero);

    let t_true = zero;
    let t_false = term!(arena, (abs (abs 0)));

    let pred = term!(arena, (abs 0 zero (abs 0)));
    let iszero = term!(arena, (abs 0 t_true (abs t_false)));

    let y = term!(
        arena,
        (abs (abs 1 (0 0)) (abs 1 (0 0)))
    );

    let plus = term!(
        arena,
        y (abs (abs (abs 1 0 (abs succ (3 0 1)))))
    );

    let mul = term!(
        arena,
        y (abs (abs (abs 1 zero (abs plus 1 (3 0 1)))))
    );

    let sub = term!(
        arena,
        y (abs (abs (abs 0 1 (abs 3 (pred 2) 0))))
    );

    let two = scott_from_int(2, zero, succ, &mut arena);
    let three = scott_from_int(3, zero, succ, &mut arena);
    let four = scott_from_int(4, zero, succ, &mut arena);
    let five = scott_from_int(5, zero, succ, &mut arena);
    let one_twenty = scott_from_int(120, zero, succ, &mut arena);

    let factorial_five = term!(arena, mul five (mul four (mul three (mul two one))));

    let test = term!(arena, iszero (sub factorial_five one_twenty));
    println!("(=0 (- (factorial 5) 120))");
    let test_value = scott_to_bool(test, &mut arena).expect("not a boolean");
    println!("> {}", test_value);
    assert!(test_value);
}
