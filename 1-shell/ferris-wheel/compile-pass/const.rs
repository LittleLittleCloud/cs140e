// FIXME: Make me compile! Diff budget: 3 lines.
#![feature(const_fn)]
const fn add(a: i32, b: i32) -> i32 {
    a + b
}
const VAR: i32 = add(34, 12);
fn main() { }    
