use extendr_api::prelude::*;
use deunicode::deunicode_with_tofu;


#[extendr]
pub fn deunicode(x: Strings) -> Strings {
    x.into_iter()
        .map(|xi| match xi.is_na() {
            true => Rstr::na(),
            false => Rstr::from(deunicode_with_tofu(xi.as_str(), "[?]")),
        })
        .collect::<Strings>()
}

extendr_module! {
    mod deunicode;
    fn deunicode;
}