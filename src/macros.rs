/// Macro for inline clamping, if you don't know what is clamping, a
/// clamping is limit a value between a min value and a max value
#[macro_export]
macro_rules! clamp {
    ($x:expr, $min:expr, $max:expr) => {
        if $x < $min {
            $min
        } else if $x > $max {
            $max
        } else {
            $x
        }
    };
}

/// Macro for check if a point is on an area
/// # Arguments
/// * 'p' - A tuple (x: i32, y: i32)
/// * 'b' - A tuple (x: i32, y: i32, width: i32, height: i32)
#[macro_export]
macro_rules! point_on_area {
    ($p:expr, $b:expr) => {
        $p.0 >= $b.0 && $p.0 <= $b.0 + $b.2 &&
        $p.1 >= $b.1 && $p.1 <= $b.1 + $b.3
    };
}

/// Macro for convert a relative position (widget point) to absolute position (container point)
#[macro_export]
macro_rules! absolute_pos {
    ($p:expr, $b:expr) => {
        ($p.0 + $b.0, $p.1 + $b.1)
    };
}


/// Macro for convert an absolute position (container point) to relative position (widget point)
#[macro_export]
macro_rules! relative_pos {
    ($p:expr, $b:expr) => {
        ($p.0 - $b.0, $p.1 - $b.1)
    };
}

#[macro_export]
macro_rules! relative_pos_clamp {
    ($p:expr, $b:expr) => {
        (clamp!($p.0 - $b.0, 0, $b.2), clamp!($p.1 - $b.1, 0, $b.3))
    };
}