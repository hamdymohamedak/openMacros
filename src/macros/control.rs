#[macro_export]
macro_rules! when {
    ($cond:expr => $then:block, else => $otherwise:block) => {{
        if $cond $then else $otherwise
    }};
    ($cond:expr => $then:block) => {{
        if $cond $then
    }};
}

#[macro_export]
macro_rules! repeat {
    ($var:ident in $start:expr => $end:expr, $body:block) => {{
        for $var in $start..$end $body
    }};
}

#[macro_export]
macro_rules! each {
    ($var:ident in $items:expr, $body:block) => {{
        for $var in $items $body
    }};
}
