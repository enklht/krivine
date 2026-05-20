use krivine::*;

fn scott_to_int(mut term: usize, arena: &mut Vec<Term>) -> Option<u32> {
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

    let four = term!(arena, (plus two two));
    let four_value = scott_to_int(four, &mut arena).expect("not a scott numeral");

    println!("2 + 2");
    println!("> {}", four_value);
    assert_eq!(four_value, 4);

    println!();

    let eight = term!(arena, (plus two (plus (plus two two) two)));
    let eight_value = scott_to_int(eight, &mut arena).expect("not a scott numeral");

    println!("2 + ((2 + 2) + 2)");
    println!("> {}", eight_value);
    assert_eq!(eight_value, 8);
}
