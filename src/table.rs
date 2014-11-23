use std::collections::Bitv;

pub type Table<T> = Vec<(T, Bitv)>;

pub fn write<T, W: Writer>(w: &mut W, table: &[(T, Bitv)]) {
}
