#[macro_export]
macro_rules! unwrap_error_type {
    ($self:expr,$op:expr) => {
        match $op {
            Ok(s) => s,
            Err(e) => return Err($self.context.error(e)),
        }
    }
}

#[macro_export]
macro_rules! xyz {
    ($x: expr, $y: expr, $z: expr) => {
        (($x & 3) << 6) | (($y & 7) << 3) | ($z & 7)
    }
}

#[macro_export]
macro_rules! xpqz {
    ($x: expr, $p: expr, $q: expr, $z: expr) => {
        (($x & 3) << 6) | (($p & 3) << 4) | (($q & 1) << 3) | ($z & 7)
    }
}

#[macro_export]
macro_rules! alu {
    ($op: expr, $r: expr) => {
        (2 << 6) | ((($op as u8 & 7) << 3) | $r & 7)
    }
}
#[macro_export]
macro_rules! rot_encode {
    ($op: expr, $r: expr) => {
        ($op as u8 & 7) << 3 | ($r & 7)
    }
}

#[macro_export]
macro_rules! alu_imm {
    ($op: expr) => {
        (3 << 6) | ((($op as u8) << 3) | 6)
    }
}
