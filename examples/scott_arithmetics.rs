use krivine::*;

pub fn scott_to_int(term: usize, mut arena: Vec<Term>) -> (Option<u32>, Vec<Term>) {
    let mut n = 0;
    let mut closure = Closure::new(term);

    let zero = term!(arena, (abs (abs 1)));

    loop {
        let machine = Machine::from_clause(closure, arena);
        (closure, arena) = machine.eval();

        match arena[closure.term] {
            Term::Abs(t1) => match arena[t1] {
                Term::Abs(t2) => match arena[t2] {
                    Term::Var(1) => return (Some(n), arena),
                    Term::App(s, _) if matches!(arena[s], Term::Var(0)) => {
                        n += 1;
                        let term = closure.term;
                        closure.term = term!(arena, term zero (abs 0));
                    }
                    _ => return (None, arena),
                },
                _ => return (None, arena),
            },
            _ => return (None, arena),
        }
    }
}

fn main() {
    let mut arena = Vec::new();

    let zero = term!(arena, (abs (abs 1)));
    let succ = term!(arena, (abs (abs (abs 0 2))));
    let one = term!(arena, succ zero);
    let two = term!(arena, succ one);

    let y = term!(
        arena,
        (abs (abs 1 (0 0))
             (abs 1 (0 0)))
    );

    let plus = term!(
        arena,
        y (abs (abs (abs 1 0 (abs succ (3 0 1)))))
    );

    println!("2 + 2");

    let four = term!(arena, (plus two two));

    let (result, mut arena) = scott_to_int(four, arena);

    match result {
        Some(value) => {
            println!("= {}\n", value);
        }
        None => {
            println!("not a scott numeral");
        }
    }

    println!("2 + ((2 + 2) + 2)");

    let eight = term!(arena, (plus two (plus (plus two two) two)));

    let (result, _) = scott_to_int(eight, arena);

    match result {
        Some(value) => {
            println!("= {}\n", value);
        }
        None => {
            println!("not a scott numeral");
        }
    }
}
