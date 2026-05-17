use super::*;

use std::fmt;

pub struct TermDisplay<'a> {
    pub term: usize,
    pub arena: &'a Vec<Term>,
}

impl fmt::Display for TermDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_inner(
            arena: &'_ Vec<Term>,
            term: usize,
            paren: bool,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            match arena[term] {
                Term::Var(v) => write!(f, "{}", v),
                Term::Abs(t1) => {
                    write!(f, "(λ.")?;
                    fmt_inner(arena, t1, false, f)?;
                    write!(f, ")")
                }
                Term::App(t1, t2) if paren => {
                    write!(f, "(")?;
                    fmt_inner(arena, t1, false, f)?;
                    write!(f, " ")?;
                    fmt_inner(arena, t2, true, f)?;
                    write!(f, ")")
                }
                Term::App(t1, t2) => {
                    fmt_inner(arena, t1, false, f)?;
                    write!(f, " ")?;
                    fmt_inner(arena, t2, true, f)
                }
            }
        }

        fmt_inner(self.arena, self.term, false, f)
    }
}
