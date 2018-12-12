macro_rules! test_bid_should_parse {
    ($($name:ident: $value:expr,)*) => {
        mod bid_should_parse {
        $(
            #[allow(non_snake_case)]
            #[test]
            fn $name() {
                use super::super::Bid::{self, *};
                let (input, expected) = $value;
                let bid = Bid::parse(input);
                assert_eq!(bid, expected);
            }
        )*
        }
    }
}

test_bid_should_parse! {
    //pass_p: ("p", Pass),
    //pass_P: ("P", Pass),
    //pass_pass: ("pass", Pass),
    pass_Pass: ("Pass", Pass),

    //dbl_d: ("d", Double),
    //dbl_D: ("D", Double),
    //dbl_dbl: ("dbl", Double),
    dbl_Dbl: ("Dbl", Double),
    //dbl_double: ("double", Double),
    //dbl_Double: ("Double", Double),
    //dbl_x: ("x", Double),
    //dbl_X: ("X", Double),

    //rdbl_r: ("r", Redouble),
    //rdbl_R: ("R", Redouble),
    //rdbl_rdbl: ("rdbl", Redouble),
    rdbl_Rdbl: ("Rdbl", Redouble),
    //rdbl_redouble: ("redouble", Redouble),
    //rdbl_Redouble: ("Redouble", Redouble),
    //rdbl_xx: ("xx", Redouble),
    //rdbl_XX: ("XX", Redouble),
}
