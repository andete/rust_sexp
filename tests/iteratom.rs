// (c) 2017 Productize SPRL <joost@productize.be>

extern crate symbolic_expressions;
#[macro_use]
extern crate error_chain;

mod error {
    use symbolic_expressions;

    error_chain! {
        
        links {
            SymbolicExpression(symbolic_expressions::Error, symbolic_expressions::ErrorKind) #[doc = "symbolic expression library error"];
        }
    }
}

use error::*;
use symbolic_expressions::iteratom::*;
use symbolic_expressions::Sexp;

struct Qq(i64);

impl FromSexp for Qq {
    fn from_sexp(s: &Sexp) -> SResult<Self> {
        let i = s.named_value_i("d")?;
        Ok(Qq(i))
    }
}


fn test_int() -> Result<()> {
    let s = "(a (b c) (d 42))";
    let s = symbolic_expressions::parser::parse_str(s)?;
    let mut i = IterAtom::new(&s, "a")?;
    let c = i.s_in_list("b")?;
    assert_eq!(&c, "c");
    let e: Qq = i.t("d")?;
    assert_eq!(e.0, 42);
    Ok(())
}

#[test]
fn test_iteratom() {
    test_int().unwrap();
}
