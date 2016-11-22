// (c) 2016 Productize SPRL <joost@productize.be>

use ser;
use formatter;
use parser;
use decode;
use data::test::*;
    
#[allow(dead_code)]
fn check_parse_res(s: &str, o: &str) {
    let e = parser::parse_str(s).unwrap();
    let t = ser::to_string(&e).unwrap();
    assert_eq!(o, t)
}

#[allow(dead_code)]
fn check_parse(s: &str) {
    let e = parser::parse_str(s).unwrap();
    let t = ser::to_string(&e).unwrap();
    assert_eq!(s, t)
}

#[allow(dead_code)]
fn check_parse_kicad(s: &str) {
    let e = parser::parse_str(s).unwrap();
    let t = ser::to_string_with_rules(&e, kicad_test_rules()).unwrap();
    assert_eq!(s, t)
}

#[allow(dead_code)]
fn check_parse_rules(s: &str, rules: formatter::Rules) {
    let e = parser::parse_str(s).unwrap();
    let t = ser::to_string_with_rules(&e, rules).unwrap();
    assert_eq!(s, t)
}


#[allow(dead_code)]
fn parse_fail(s: &str) {
    parser::parse_str(s).unwrap();
}

#[allow(dead_code)]
pub fn kicad_test_rules() -> formatter::Rules {
    let mut rf = formatter::Rules::new();
    rf.insert("layer", 1);
    rf.insert("desc", 1);
    rf.insert("fp_text", 1);
    rf.insert("fp_poly", 1);
    rf.insert("fp_line", 1);
    rf.insert("pad", 1);
    rf.insert("general", 1);
    rf
}

#[test]
fn test_empty() {
    check_parse("")
}

#[test]
fn test_empty_qstring() {
    check_parse("(hello \"\")")
}

#[test]
fn test_minimal() {
    check_parse("()")
}

#[test]
fn test_string() {
    check_parse("hello")
}

#[test]
fn test_qstring_a() {
    check_parse_res("\"hello\"", "hello")
}

#[test]
fn test_qstring_a2() {
    check_parse("\"hello world\"")
}

#[test]
fn test_qstring_a3() {
    check_parse("\"hello(world)\"")
}

#[test]
fn test_number() {
    check_parse("1.3")
}

#[test]
fn test_float_vs_int() {
    check_parse("2.0")
}

#[test]
fn test_double() {
    check_parse("(())")
}

#[test]
fn test_br_string() {
    check_parse("(world)")
}

#[test]
fn test_br_qstring() {
    check_parse_res("(\"world\")", "(world)")
}

#[test]
fn test_br_int() {
    check_parse("(42)")
}

#[test]
fn test_br_float() {
    check_parse("(12.7)")
}

#[test]
fn test_br_qbrstring() {
    check_parse("(\"(()\")")
}

#[test]
fn test_number_string() {
    check_parse("567A_WZ")
}

#[test]
#[should_panic(expected="End of file reached")]
fn test_invalid1() {
    parse_fail("(")
}

#[test]
#[should_panic(expected="Unexpected )")]
fn test_invalid2() {
    parse_fail(")")
}

#[test]
#[should_panic(expected="End of file reached")]
fn test_invalid3() {
    parse_fail("\"hello")
}

#[test]
#[should_panic(expected="line: 4, col: 5")]
fn test_invalid_check_position() {
    parse_fail("\"hello


    ")
}

#[test]
fn test_complex() {
    check_parse("(module SWITCH_3W_SIDE_MMP221-R (layer F.Cu) (descr \"\") (pad 1 thru_hole rect \
                 (size 1.2 1.2) (at -2.5 -1.6 0) (layers *.Cu *.Mask) (drill 0.8)) (pad 2 \
                 thru_hole rect (size 1.2 1.2) (at 0.0 -1.6 0) (layers *.Cu *.Mask) (drill 0.8)) \
                 (pad 3 thru_hole rect (size 1.2 1.2) (at 2.5 -1.6 0) (layers *.Cu *.Mask) (drill \
                 0.8)) (pad 5 thru_hole rect (size 1.2 1.2) (at 0.0 1.6 0) (layers *.Cu *.Mask) \
                 (drill 0.8)) (pad 6 thru_hole rect (size 1.2 1.2) (at -2.5 1.6 0) (layers *.Cu \
                 *.Mask) (drill 0.8)) (pad 4 thru_hole rect (size 1.2 1.2) (at 2.5 1.6 0) (layers \
                 *.Cu *.Mask) (drill 0.8)) (fp_line (start -4.5 -1.75) (end 4.5 -1.75) (layer \
                 F.SilkS) (width 0.127)) (fp_line (start 4.5 -1.75) (end 4.5 1.75) (layer \
                 F.SilkS) (width 0.127)) (fp_line (start 4.5 1.75) (end -4.5 1.75) (layer \
                 F.SilkS) (width 0.127)) (fp_line (start -4.5 1.75) (end -4.5 -1.75) (layer \
                 F.SilkS) (width 0.127)))")
}

#[test]
fn test_kicad_1() {
    check_parse_kicad("(module SILABS_EFM32_QFM24
  (layer F.Cu))")
}

#[test]
fn test_multiline() {
    let mut rules = formatter::Rules::new();
    rules.insert("foo", 1);
    rules.insert("mars", 1);
    check_parse_rules("\
(hello \"test it\"
  (foo bar)
  (mars venus))",
                      rules)
}

#[test]
fn test_multiline_two_empty() {
    check_parse_res("\
(hello

world)",
                    "(hello world)")
}

#[test]
fn test_fail_pcb() {
    check_parse_kicad("\
(kicad_pcb (version 4) (host pcbnew \"(2015-05-31 BZR 5692)-product\")
  \
                       (general))")
}

#[test]
fn test_decode_struct() {
    let s = "(decodestruct (world foo) (mars 42))";
    let e = parser::parse_str(s).unwrap();
    let h:DecodeStruct = decode::decode(e).unwrap();
    assert_eq!(h, DecodeStruct { world:"foo".into(), mars:42 });
}

#[test]
fn test_decode_tuple_struct() {
    let s = "(decodetuplestruct 42 foo)";
    let e = parser::parse_str(s).unwrap();
    let h:DecodeTupleStruct = decode::decode(e).unwrap();
    assert_eq!(h, DecodeTupleStruct(42,"foo".into()));
}

#[test]
fn test_decode_vec_int() {
    let s = "(4 5 42)";
    let e = parser::parse_str(s).unwrap();
    let h:Vec<i64> = decode::decode(e).unwrap();
    assert_eq!(h, vec![4,5,42]);
}

#[test]
fn test_decode_vec_string() {
    let s = "(hi there mars)";
    let e = parser::parse_str(s).unwrap();
    let h:Vec<String> = decode::decode(e).unwrap();
    let i:Vec<String> = vec!["hi","there", "mars"].iter().map(|&x| x.into()).collect();
    assert_eq!(h, i);
}

#[test]
fn test_decode_vec_string_int() {
    let s = "(4 5 42)";
    let e = parser::parse_str(s).unwrap();
    let h:Vec<String> = decode::decode(e).unwrap();
    let i:Vec<String> = vec!["4","5", "42"].iter().map(|&x| x.into()).collect();
    assert_eq!(h, i);
}

#[test]
fn test_decode_struct_nested() {
    let s = "(decodenested (world (1 2 3)) (mars (planet (size 7))))";
    let e = parser::parse_str(s).unwrap();
    let h:DecodeNested = decode::decode(e).unwrap();
    let i = DecodeNested {
        world:vec![1,2,3],
        mars:Planet { size: 7 },
    };
    assert_eq!(h, i);
}

#[test]
fn test_decode_empty() {
    let s = "";
    let e = parser::parse_str(s).unwrap();
    let () = decode::decode(e).unwrap();
}
