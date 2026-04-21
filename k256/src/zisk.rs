
extern "C" {
    // Field operations
    pub fn reduce_fp_secp256k1_c(x_ptr: *const u64, result_ptr: *mut u64);
    pub fn add_fp_secp256k1_c(x_ptr: *const u64, y_ptr: *const u64, result_ptr: *mut u64);
    pub fn neg_fp_secp256k1_c(x_ptr: *const u64, result_ptr: *mut u64);
    pub fn mul_fp_secp256k1_c(x_ptr: *const u64, y_ptr: *const u64, result_ptr: *mut u64);

    // Scalar operations
    pub fn reduce_fn_secp256k1_c(x_ptr: *const u64, result_ptr: *mut u64);
    pub fn add_fn_secp256k1_c(x_ptr: *const u64, y_ptr: *const u64, result_ptr: *mut u64);
    pub fn neg_fn_secp256k1_c(x_ptr: *const u64, result_ptr: *mut u64);
    pub fn sub_fn_secp256k1_c(x_ptr: *const u64, y_ptr: *const u64, result_ptr: *mut u64);
    pub fn mul_fn_secp256k1_c(x_ptr: *const u64, y_ptr: *const u64, result_ptr: *mut u64);
    pub fn inv_fn_secp256k1_c(x_ptr: *const u64, result_ptr: *mut u64);

    // Curve operations
    pub fn jacobian_to_affine_secp256k1_c(p_ptr: *const u64, result_ptr: *mut u64);
    pub fn lift_x_secp256k1_c(x_ptr: *const u8, y_is_odd: u8, result_ptr: *mut u64) -> u8;
    pub fn double_scalar_mul_with_g_secp256k1_c(k1_ptr: *const u64, k2_ptr: *const u64, p_ptr: *const u64, result_ptr: *mut u64) -> u8;

    // ECDSA verification
    pub fn ecdsa_verify_secp256k1_c(pk_ptr: *const u64, z_ptr: *const u64, r_ptr: *const u64, s_ptr: *const u64) -> u8;
}