#[macro_export]
macro_rules! term {
    ($e:expr, (abs $($body:tt)+)) => {{
        let body = term!($e, $($body)*);
        $e.alloc(Term::Abs(body))
    }};
    ($e:expr, $func:tt $($args:tt)+) => {{
        let f = term!($e, $func);
        term!(@app $e, f, $($args)+)
    }};
    ($e:expr, ($($t:tt)+)) => {
        term!($e, $($t)+)
    };
    ($e:expr, $v:literal) => {
        $e.alloc(Term::Var($v))
    };
    ($e:expr, $v:tt) => {
        $v
    };
    (@app $e:expr, $acc:expr, $next:tt $($rest:tt)*) => {{
        let arg = term!($e, $next);
        let app = $e.alloc(Term::App($acc, arg));
        term!(@app $e, app, $($rest)*)
    }};
    (@app $e:expr, $acc:expr,) => { $acc };
}
