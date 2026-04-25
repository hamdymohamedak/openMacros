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

#[macro_export]
macro_rules! retry {
    ($attempts:expr => $operation:expr) => {{
        let __attempts: usize = $attempts;
        if __attempts == 0 {
            Err($crate::AkError::Validation(
                "retry attempts must be greater than zero",
            ))
        } else {
            let mut __remaining = __attempts;
            loop {
                match $operation {
                    Ok(value) => break Ok(value),
                    Err(err) => {
                        __remaining -= 1;
                        if __remaining == 0 {
                            break Err(err);
                        }
                    }
                }
            }
        }
    }};
    ($attempts:expr => $operation:expr, delay_ms => $delay_ms:expr) => {{
        let __attempts: usize = $attempts;
        if __attempts == 0 {
            Err($crate::AkError::Validation(
                "retry attempts must be greater than zero",
            ))
        } else {
            let mut __remaining = __attempts;
            loop {
                match $operation {
                    Ok(value) => break Ok(value),
                    Err(err) => {
                        __remaining -= 1;
                        if __remaining == 0 {
                            break Err(err);
                        }
                        std::thread::sleep(std::time::Duration::from_millis($delay_ms));
                    }
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! measure_ms {
    ($operation:expr) => {{
        let __started_at = std::time::Instant::now();
        let __value = $operation;
        let __elapsed_ms = __started_at.elapsed().as_millis();
        (__value, __elapsed_ms)
    }};
}

#[macro_export]
macro_rules! ensure {
    ($condition:expr, $message:expr) => {{
        if !$condition {
            return Err($crate::AkError::Validation($message));
        }
    }};
}
