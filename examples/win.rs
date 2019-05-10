extern crate console;

use console::Term;

fn main() {
    let term = Term::stdout();
    dbg!(term.read_key());
}
