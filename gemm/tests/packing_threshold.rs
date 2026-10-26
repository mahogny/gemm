use gemm::{get_rhs_packing_threshold, set_rhs_packing_threshold};

#[test]
fn rhs_packing_threshold_accepts_values_above_default() {
    let previous = get_rhs_packing_threshold();
    set_rhs_packing_threshold(512);
    assert_eq!(get_rhs_packing_threshold(), 512);
    set_rhs_packing_threshold(4097);
    assert_eq!(get_rhs_packing_threshold(), 4096);
    set_rhs_packing_threshold(previous);
}
