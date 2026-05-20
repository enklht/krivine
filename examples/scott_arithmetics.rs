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

fn main() {
    let mut arena = Vec::new();

    let zero = term!(arena, (abs (abs 1)));
    let succ = term!(arena, (abs (abs (abs 0 2))));
    let one = term!(arena, succ zero);
    let two = term!(arena, succ one);

    println!("2");

    match scott_to_int(two, &mut arena) {
        Some(value) => {
            println!("= {}\n", value);
        }
        None => {
            println!("not a scott numeral");
        }
    }

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
    println!("{}", arena.display(four));

    match scott_to_int(four, &mut arena) {
        Some(value) => {
            println!("= {}\n", value);
        }
        None => {
            println!("not a scott numeral");
        }
    }

    println!("2 + ((2 + 2) + 2)");

    let eight = term!(arena, (plus two (plus (plus two two) two)));
    println!("{}", arena.display(eight));

    match scott_to_int(eight, &mut arena) {
        Some(value) => {
            println!("= {}\n", value);
        }
        None => {
            println!("not a scott numeral");
        }
    }
}
