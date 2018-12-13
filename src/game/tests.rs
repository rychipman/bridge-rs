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

macro_rules! test_bidding_finished {
    ($($name:ident: $value:expr,)*) => {
        mod bidding_finished {
        $(
            #[test]
            fn $name() {
                use super::super::{BidSequence, Bid};
                let (bids, expected) = $value;
                let bid_seq = BidSequence::new(bids.into_iter().map(Bid::parse).collect());
                assert_eq!(bid_seq.is_finished(), expected);
            }
        )*
        }
    }
}

test_bidding_finished! {
    too_short_1: (vec![ "Pass" ], false),
    too_short_2: (vec![ "Pass", "Pass" ], false ),
    too_short_3: (vec![ "Pass", "Pass", "Pass" ], false ),
    passed_out: (vec![ "Pass", "Pass", "Pass", "Pass" ], true ),
    pass_after_bid_1: (vec![ "1S", "Pass" ], false ),
    pass_after_bid_2: (vec![ "1S", "Pass", "Pass" ], false ),
    pass_out_after_bid: (vec![ "1S", "Pass", "Pass", "Pass" ], true ),
    pass_after_dbl_2: (vec![ "1S", "Dbl", "Pass", "Pass" ], false ),
    pass_out_after_dbl: (vec![ "1S", "Dbl", "Pass", "Pass", "Pass" ], true ),
}

macro_rules! test_valid_continuation {
    ($($name:ident: $value:expr,)*) => {
        mod valid_continuation {
        $(
            #[test]
            fn $name() {
                use super::super::{BidSequence, Bid};
                let (bids, next, expected) = $value;
                let bid_seq = BidSequence::new(bids.into_iter().map(Bid::parse).collect());
                let next = Bid::parse(next);
                assert_eq!(bid_seq.valid_continuation(&next), expected);
            }
        )*
        }
    }
}

test_valid_continuation! {
    opening_pass: (vec![], "Pass", true),
    opening_double: (vec![], "Dbl", false),
    opening_redouble: (vec![], "Rdbl", false),
    opening_contract_0: (vec![], "1NT", true),
    opening_contract_1: (vec![], "7D", true),

    after_contract_pass: (vec!["1S"], "Pass", true),
    after_contract_double: (vec!["1S"], "Dbl", true),
    after_contract_redouble: (vec!["1S"], "Rdbl", false),
    after_contract_contract_0: (vec!["1S"], "1NT", true),
    after_contract_contract_1: (vec!["1S"], "4S", true),
    after_contract_too_low_0: (vec!["1S"], "1C", false),
    after_contract_too_low_1: (vec!["1NT"], "1S", false),

    double_partner: (vec!["1S", "Pass"], "Dbl", false),

    after_double_partner_contract_too_low: (vec!["1S", "Dbl", "Pass"], "1C", false),
    after_double_partner_contract_valid: (vec!["1S", "Dbl", "Pass"], "2H", true),
    after_double_partner_pass: (vec!["1S", "Dbl", "Pass"], "Pass", true),
    after_double_partner_double: (vec!["1S", "Dbl", "Pass"], "Dbl", false),
    after_double_partner_redouble: (vec!["1S", "Dbl", "Pass"], "Rdbl", false),
    after_double_redouble: (vec!["1S", "Dbl"], "Rdbl", true),
    after_double_left_opp_redouble: (vec!["1S", "Dbl", "Pass", "Pass"], "Rdbl", true),

    after_redouble_double: (vec!["1S", "Dbl", "Rdbl"], "Dbl", false),
    after_redouble_redouble: (vec!["1S", "Dbl", "Rdbl"], "Rdbl", false),
    after_redouble_contract_too_low_0: (vec!["1S", "Dbl", "Rdbl"], "1S", false),
    after_redouble_contract_too_low_1: (vec!["1S", "Dbl", "Rdbl"], "1D", false),
    after_redouble_pass: (vec!["1S", "Dbl", "Rdbl"], "Pass", true),
    after_redouble_contract_valid: (vec!["1S", "Dbl", "Rdbl"], "2C", true),

    pass_out: (vec!["Pass", "Pass", "Pass"], "Pass", true),
    already_passed_out_pass: (vec!["Pass", "Pass", "Pass", "Pass"], "Pass", false),
    already_passed_out_contract: (vec!["Pass", "Pass", "Pass", "Pass"], "1S", false),
    pass_out_after_contract: (vec!["1S", "Pass", "Pass", "Pass"], "Pass", false),
    already_passed_out_after_contract: (vec!["1C", "Pass", "Pass", "Pass"], "1S", false),
    not_passed_out: (vec!["1C", "Dbl", "Pass", "Pass"], "1S", true),
}

mod compare {

    macro_rules! test_bid_compare {
        ($($name:ident: $value:expr,)*) => {
            mod bid {
                use std::cmp::Ordering::*;
                use super::super::super::Bid;
            $(
                #[test]
                fn $name() {
                    let (left, right, expected) = $value;
                    let left = Bid::parse(left);
                    let right = Bid::parse(right);
                    let order = left.partial_cmp(&right).expect("should have an ordering");
                    assert_eq!(order, expected);
                }
            )*
            }
        }
    }

    test_bid_compare! {
        identity_pass: ("Pass", "Pass", Equal),
        identity_dbl: ("Dbl", "Dbl", Equal),
        identity_rdbl: ("Rdbl", "Rdbl", Equal),
        contract_0: ("1S", "1S", Equal),
        contract_1: ("1S", "1NT", Less),
        contract_2: ("2S", "1S", Greater),
        contract_3: ("4S", "1S", Greater),
        contract_4: ("1NT", "1S", Greater),
        contract_5: ("1S", "1H", Greater),
        contract_6: ("1H", "1D", Greater),
        contract_7: ("1D", "1C", Greater),
        contract_8: ("1D", "4C", Less),
    }

    #[test]
    fn level() {
        use super::super::Level::*;
        assert!(One == One);
        assert!(Two < Three);
        assert!(Five > Four);
    }
}
