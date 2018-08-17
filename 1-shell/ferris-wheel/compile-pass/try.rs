// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
struct ErrorA;
struct ErrorB;
enum Error {
    A(ErrorA),
    B(ErrorB),
    BOTH(ErrorA,ErrorB)
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    let f=(do_a(),do_b());
    match f {
        (Ok(ref a),Ok(ref b))=>Ok((*a,*b)),
        (Err(a),Ok(_))=>Err(Error::A(a)),
        (Ok(_),Err(b))=>Err(Error::B(b)),
        (Err(a),Err(b))=>Err(Error::BOTH(a,b))
    }
}

fn main() { }
