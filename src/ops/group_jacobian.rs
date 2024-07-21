use std::time::Instant;

use logging_timer::{stimer, time, timer, Level};
use rand::Rng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use tfhe::{
    core_crypto::prelude::Numeric,
    integer::{
        block_decomposition::{DecomposableInto, RecomposableFrom},
        BooleanBlock, RadixCiphertext, ServerKey,
    },
    prelude::FheEncrypt,
    FheBool,
};

use crate::{
    helper::{format, read_client_key},
    numeral::Numeral,
    ops::{
        native::{add_mod_native, double_mod_native, mul_mod_native, sub_mod_native},
        primitive::parallel_fn,
        selector_zero, selector_zero_constant,
    },
};

use super::{
    add_mod, double_mod, inverse_mod, mul_mod,
    native::{inverse_mod_native, square_mod_native},
    selector, square_mod, sub_mod,
};

/// native double group element using jacobian coordinates.
pub fn group_projective_double_native<P: Numeral>(x: P, y: P, z: P, p: P) -> (P, P, P) {
    // case curve a = 0
    // a = x^2
    let a = square_mod_native(x, p);
    // b = y^2
    let b = square_mod_native(y, p);
    // c = b^2
    let c = square_mod_native(b, p);
    // d = 2*((x + b)^2-(a + c))
    let xb2 = square_mod_native(add_mod_native(x, b, p), p);
    let ac = add_mod_native(a, c, p);
    let d = double_mod_native(sub_mod_native(xb2, ac, p), p);
    // e = 3*a
    let e = add_mod_native(double_mod_native(a, p), a, p);
    // f = e^2
    let f = square_mod_native(e, p);
    // z' = 2*y*z
    let z_prime = double_mod_native(mul_mod_native(y, z, p), p);
    // x' = f - 2*d
    let x_prime = sub_mod_native(f, double_mod_native(d, p), p);
    // y' = e*(d - x') - 8*c
    let edx = mul_mod_native(e, sub_mod_native(d, x_prime, p), p);
    let c2 = double_mod_native(c, p);
    let c4 = double_mod_native(c2, p);
    let c8 = double_mod_native(c4, p);
    let y_prime = sub_mod_native(edx, c8, p);

    (x_prime, y_prime, z_prime)
}

/// native add 2 group elements using between jacobian and affine coordinates
/// this algorithm faster than adding 2 group elements using jacobian coordinates
pub fn group_projective_add_affine_native<P: Numeral>(
    x: P,
    y: P,
    z: P,
    other_x: P,
    other_y: P,
    p: P,
) -> (P, P, P) {
    if z == P::ZERO {
        return (other_x, other_y, P::ONE);
    }
    // z1z1 = z1^2
    let z1z1 = square_mod_native(z, p);
    // u2 = x2*z1z1
    let u2 = mul_mod_native(other_x, z1z1, p);
    // s2 = y2*z1*z1*z1
    let s2 = mul_mod_native(other_y, mul_mod_native(z1z1, z, p), p);

    if x == u2 && y == s2 {
        return group_projective_double_native(x, y, z, p);
    }

    // h = u2 - x1
    let h = sub_mod_native(u2, x, p);
    // hh = h^2
    let hh = square_mod_native(h, p);
    // i = 4*hh
    let i = double_mod_native(hh, p);
    let i = double_mod_native(i, p);
    // j = h*i
    let j = mul_mod_native(h, i, p);
    // r = 2*(s2 - y1)
    let r = double_mod_native(sub_mod_native(s2, y, p), p);
    // v = x1*i
    let v = mul_mod_native(x, i, p);
    // x3 = r^2 - j - 2*v
    let x3 = sub_mod_native(
        square_mod_native(r, p),
        add_mod_native(j, double_mod_native(v, p), p),
        p,
    );
    // y3 = r*(v - x3) - 2*y1*j
    let y3 = sub_mod_native(
        mul_mod_native(r, sub_mod_native(v, x3, p), p),
        double_mod_native(mul_mod_native(y, j, p), p),
        p,
    );
    // z3 = 2*z1*h
    let z3 = double_mod_native(mul_mod_native(z, h, p), p);

    (x3, y3, z3)
}

pub fn group_projective_add_projective_native<P: Numeral>(
    x: P,
    y: P,
    z: P,
    other_x: P,
    other_y: P,
    other_z: P,
    p: P,
) -> (P, P, P) {
    if z == P::ZERO {
        return (other_x, other_y, other_z);
    }

    if other_z == P::ZERO {
        return (x, y, z);
    }

    // z0z0 = z0^2
    let z0z0 = square_mod_native(z, p);
    // z1z1 = z1^2
    let z1z1 = square_mod_native(other_z, p);
    // u0 = x0*z1z1
    let u0 = mul_mod_native(x, z1z1, p);
    // u1 = x1*z0z0
    let u1 = mul_mod_native(other_x, z0z0, p);
    // s0 = y0*z1*z1z1
    let s0 = mul_mod_native(y, mul_mod_native(other_z, z1z1, p), p);
    // s1 = y1*z0*z0z0
    let s1 = mul_mod_native(other_y, mul_mod_native(z, z0z0, p), p);

    if u0 == u1 && s0 == s1 {
        return group_projective_double_native(x, y, z, p);
    }

    // h = u1 - u0
    let h = sub_mod_native(u1, u0, p);
    // r = 2*(s1 - s0)
    let r = double_mod_native(sub_mod_native(s1, s0, p), p);
    // i = (2*h)^2
    let i = square_mod_native(double_mod_native(h, p), p);
    // j = h*i
    let j = mul_mod_native(h, i, p);
    // v = u0*i
    let v = mul_mod_native(u0, i, p);
    // x_prime = r^2 - j - 2*v
    let x_prime = sub_mod_native(
        sub_mod_native(square_mod_native(r, p), j, p),
        double_mod_native(v, p),
        p,
    );
    // y_prime = r*(v - x_prime) - 2*s0*j
    let y_prime = sub_mod_native(
        mul_mod_native(r, sub_mod_native(v, x_prime, p), p),
        double_mod_native(mul_mod_native(s0, j, p), p),
        p,
    );
    // z_prime = ((z0 + z1)^2 - z0z0 - z1z1)*h
    let z_prime = mul_mod_native(
        sub_mod_native(
            sub_mod_native(square_mod_native(add_mod_native(z, other_z, p), p), z0z0, p),
            z1z1,
            p,
        ),
        h,
        p,
    );

    return (x_prime, y_prime, z_prime);
}

/// homomorphic add 2 group elements between jacobian coordinates and affine coordinates
/// this algorithm faster than adding 2 group elements using jacobian coordinates
#[allow(clippy::too_many_arguments)]
#[time("info", "Group Projective Add Mixed")]
pub fn group_projective_add_affine<const NB: usize, P: Numeral>(
    x: &RadixCiphertext,
    y: &RadixCiphertext,
    z: &RadixCiphertext,
    other_x: &RadixCiphertext,
    other_y: &RadixCiphertext,
    other_flag_bit: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext, RadixCiphertext) {
    // z1z1 = z1^2
    let z1z1 = square_mod::<NB, _>(z, p, server_key);
    // u2 = x2*z1z1
    // s2 = y2*z1*z1*z1
    let (u2, s2) = rayon::join(
        || mul_mod::<NB, _>(other_x, &z1z1, p, server_key),
        || {
            mul_mod::<NB, _>(
                other_y,
                &mul_mod::<NB, _>(&z1z1, z, p, server_key),
                p,
                server_key,
            )
        },
    );
    // h = u2 - x1
    let h = sub_mod::<NB, _>(&u2, x, p, server_key);
    // hh = h^2
    let hh = square_mod::<NB, _>(&h, p, server_key);
    // i = 4*hh
    let i = double_mod::<NB, _>(&double_mod::<NB, _>(&hh, p, server_key), p, server_key);
    // j = h*i
    // v = x1*i
    let (j, v) = rayon::join(
        || mul_mod::<NB, _>(&h, &i, p, server_key),
        || mul_mod::<NB, _>(x, &i, p, server_key),
    );
    // r = 2*(s2 - y1)
    let r = double_mod::<NB, _>(&sub_mod::<NB, _>(&s2, y, p, server_key), p, server_key);
    // x3 = r^2 - j - 2*v
    // y3 = r*(v - x3) - 2*y1*j
    // z3 = 2*z1*h
    let ((x3, z3), yj2) = rayon::join(
        || {
            rayon::join(
                || {
                    sub_mod::<NB, _>(
                        &sub_mod::<NB, _>(
                            &square_mod::<NB, _>(&r, p, server_key),
                            &j,
                            p,
                            server_key,
                        ),
                        &double_mod::<NB, _>(&v, p, server_key),
                        p,
                        server_key,
                    )
                },
                || double_mod::<NB, _>(&mul_mod::<NB, _>(z, &h, p, server_key), p, server_key),
            )
        },
        || mul_mod::<NB, _>(y, &double_mod::<NB, _>(&j, p, server_key), p, server_key),
    );
    let y3 = sub_mod::<NB, _>(
        &mul_mod::<NB, _>(&r, &sub_mod::<NB, _>(&v, &x3, p, server_key), p, server_key),
        &yj2,
        p,
        server_key,
    );

    // z1'/z0' 0  1
    //    0    x' x1
    //    1    x0 x0
    // x'' =  x' * is_z0_z1_non_zero + (x0 + x1) * not_is_z0_z1_non_zero
    // y'' =  y' * is_z0_z1_non_zero + (y0 + y1) * not_is_z0_z1_non_zero
    // z'' =  z' * is_z0_z1_non_zero + (z0 + z1) * not_is_z0_z1_non_zero
    let (is_z0_non_zero, is_z1_non_zero) = rayon::join(
        || server_key.scalar_ne_parallelized(z, 0),
        || server_key.scalar_ne_parallelized(other_flag_bit, 0),
    );
    let mut radix_is_z0_non_zero: RadixCiphertext = is_z0_non_zero.into_radix(NB - 1, server_key);
    let mut radix_is_z1_non_zero: RadixCiphertext = is_z1_non_zero.into_radix(NB - 1, server_key);

    server_key.trim_radix_blocks_msb_assign(&mut radix_is_z0_non_zero, NB - 1);
    server_key.trim_radix_blocks_msb_assign(&mut radix_is_z1_non_zero, NB - 1);

    server_key.trim_radix_blocks_msb_assign(&mut radix_is_z1_non_zero, NB - 1);
    let is_z0_z1_non_zero =
        server_key.bitand_parallelized(&radix_is_z0_non_zero, &radix_is_z1_non_zero);
    let not_is_z0_z1_non_zero =
        server_key.sub_parallelized(&server_key.create_trivial_radix(1, 1), &is_z0_z1_non_zero);

    let (((xp1, xp2), (yp1, yp2)), (zp1, zp2)) = rayon::join(
        || {
            rayon::join(
                || {
                    rayon::join(
                        || server_key.mul_parallelized(&x3, &is_z0_z1_non_zero),
                        || {
                            server_key.mul_parallelized(
                                &server_key.add_parallelized(x, other_x),
                                &not_is_z0_z1_non_zero,
                            )
                        },
                    )
                },
                || {
                    rayon::join(
                        || server_key.mul_parallelized(&y3, &is_z0_z1_non_zero),
                        || {
                            server_key.mul_parallelized(
                                &server_key.add_parallelized(y, other_y),
                                &not_is_z0_z1_non_zero,
                            )
                        },
                    )
                },
            )
        },
        || {
            rayon::join(
                || server_key.mul_parallelized(&z3, &is_z0_z1_non_zero),
                || {
                    server_key.mul_parallelized(
                        &server_key.add_parallelized(z, other_flag_bit),
                        &not_is_z0_z1_non_zero,
                    )
                },
            )
        },
    );

    let ((x_prime, y_prime), z_prime) = rayon::join(
        || {
            rayon::join(
                || server_key.add_parallelized(&xp1, &xp2),
                || server_key.add_parallelized(&yp1, &yp2),
            )
        },
        || server_key.add_parallelized(&zp1, &zp2),
    );

    (x_prime, y_prime, z_prime)
}

/// homomorphic group elements double for jacobian coordinates
#[time("info", "Group Projective Double")]
pub fn group_projective_double<const NB: usize, P: Numeral>(
    x: &RadixCiphertext,
    y: &RadixCiphertext,
    z: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext, RadixCiphertext) {
    // case curve a = 0
    // a = x^2
    // b = y^2

    let (a, b) = rayon::join(
        || square_mod::<NB, _>(x, p, server_key),
        || square_mod::<NB, _>(y, p, server_key),
    );
    // c = b^2
    let c = square_mod::<NB, _>(&b, p, server_key);
    // d = 2*((x + b)^2-(a + c))
    let (xb, ac) = rayon::join(
        || add_mod::<NB, _>(x, &b, p, server_key),
        || add_mod::<NB, _>(&a, &c, p, server_key),
    );
    let d = double_mod::<NB, _>(
        &sub_mod::<NB, _>(&square_mod::<NB, _>(&xb, p, server_key), &ac, p, server_key),
        p,
        server_key,
    );
    // e = 3*a
    let e = add_mod::<NB, _>(&double_mod::<NB, _>(&a, p, server_key), &a, p, server_key);
    // f = e^2
    // z' = 2*y*z
    let (f, z_prime) = rayon::join(
        || square_mod::<NB, _>(&e, p, server_key),
        || double_mod::<NB, _>(&mul_mod::<NB, _>(y, z, p, server_key), p, server_key),
    );
    // x' = f - 2*d
    let x_prime = sub_mod::<NB, _>(&f, &double_mod::<NB, _>(&d, p, server_key), p, server_key);
    // y' = e*(d - x') - 8*c
    let (edx, c8) = rayon::join(
        || {
            mul_mod::<NB, _>(
                &e,
                &sub_mod::<NB, _>(&d, &x_prime, p, server_key),
                p,
                server_key,
            )
        },
        || {
            let c2 = double_mod::<NB, _>(&c, p, server_key);
            let c4 = double_mod::<NB, _>(&c2, p, server_key);
            double_mod::<NB, _>(&c4, p, server_key)
        },
    );
    let y_prime = sub_mod::<NB, _>(&edx, &c8, p, server_key);
    (x_prime, y_prime, z_prime)
}

/// homomorphic group elements add between jacobian coordinates
#[time("info", "Group Projective Add")]
#[allow(clippy::too_many_arguments)]
pub fn group_projective_add_projective<const NB: usize, P: Numeral>(
    x0: &RadixCiphertext,
    y0: &RadixCiphertext,
    z0: &RadixCiphertext,
    x1: &RadixCiphertext,
    y1: &RadixCiphertext,
    z1: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext, RadixCiphertext) {
    // z0z0 = z0^2
    // z1z1 = z1^2
    let (z0z0, z1z1) = rayon::join(
        || square_mod::<NB, _>(z0, p, server_key),
        || square_mod::<NB, _>(z1, p, server_key),
    );
    // u0 = x0*z1z1
    // u1 = x1*z0z0
    let (u0, u1) = rayon::join(
        || mul_mod::<NB, _>(x0, &z1z1, p, server_key),
        || mul_mod::<NB, _>(x1, &z0z0, p, server_key),
    );
    // s0 = y0*z1*z1z1
    // s1 = y1*z0*z0z0
    let (s0, s1) = rayon::join(
        || {
            mul_mod::<NB, _>(
                y0,
                &mul_mod::<NB, _>(z1, &z1z1, p, server_key),
                p,
                server_key,
            )
        },
        || {
            mul_mod::<NB, _>(
                y1,
                &mul_mod::<NB, _>(z0, &z0z0, p, server_key),
                p,
                server_key,
            )
        },
    );
    // h = u1 - u0
    // r = 2*(s1 - s0)
    let (h, r) = rayon::join(
        || sub_mod::<NB, _>(&u1, &u0, p, server_key),
        || double_mod::<NB, _>(&sub_mod::<NB, _>(&s1, &s0, p, server_key), p, server_key),
    );
    // i = (2*h)^2
    let i = square_mod::<NB, _>(&double_mod::<NB, _>(&h, p, server_key), p, server_key);
    // j = h*i
    // v = u0*i
    let (j, v) = rayon::join(
        || mul_mod::<NB, _>(&h, &i, p, server_key),
        || mul_mod::<NB, _>(&u0, &i, p, server_key),
    );
    // x_prime = r^2 - j - 2*v
    // y_prime = r*(v - x_prime) - 2*s0*j
    // z_prime = ((z0 + z1)^2 - z0z0 - z1z1)*h
    let ((r2, s0j2), z0z12) = rayon::join(
        || {
            rayon::join(
                || square_mod::<NB, _>(&r, p, server_key),
                || double_mod::<NB, _>(&mul_mod::<NB, _>(&s0, &j, p, server_key), p, server_key),
            )
        },
        || square_mod::<NB, _>(&add_mod::<NB, _>(z0, z1, p, server_key), p, server_key),
    );
    let mut x_prime = sub_mod::<NB, _>(
        &sub_mod::<NB, _>(&r2, &j, p, server_key),
        &double_mod::<NB, _>(&v, p, server_key),
        p,
        server_key,
    );
    let (mut y_prime, mut z_prime) = rayon::join(
        || {
            sub_mod::<NB, _>(
                &mul_mod::<NB, _>(
                    &r,
                    &sub_mod::<NB, _>(&v, &x_prime, p, server_key),
                    p,
                    server_key,
                ),
                &s0j2,
                p,
                server_key,
            )
        },
        || {
            mul_mod::<NB, _>(
                &sub_mod::<NB, _>(
                    &sub_mod::<NB, _>(&z0z12, &z0z0, p, server_key),
                    &z1z1,
                    p,
                    server_key,
                ),
                &h,
                p,
                server_key,
            )
        },
    );

    // z1'/z0' 0  1
    //    0    x' x1
    //    1    x0 x0
    // x'' =  x' * is_z0_z1_non_zero + (x0 + x1) * not_is_z0_z1_non_zero
    // y'' =  y' * is_z0_z1_non_zero + (y0 + y1) * not_is_z0_z1_non_zero
    // z'' =  z' * is_z0_z1_non_zero + (z0 + z1) * not_is_z0_z1_non_zero
    let (is_z0_non_zero, is_z1_non_zero) = rayon::join(
        || server_key.scalar_ne_parallelized(z0, 0),
        || server_key.scalar_ne_parallelized(z1, 0),
    );

    let mut radix_is_z0_non_zero: RadixCiphertext = is_z0_non_zero.into_radix(NB - 1, server_key);
    let mut radix_is_z1_non_zero: RadixCiphertext = is_z1_non_zero.into_radix(NB - 1, server_key);

    server_key.trim_radix_blocks_msb_assign(&mut radix_is_z0_non_zero, NB - 1);
    server_key.trim_radix_blocks_msb_assign(&mut radix_is_z1_non_zero, NB - 1);

    let is_z0_z1_non_zero =
        server_key.bitand_parallelized(&radix_is_z0_non_zero, &radix_is_z1_non_zero);
    let not_is_z0_z1_non_zero =
        server_key.sub_parallelized(&server_key.create_trivial_radix(1, 1), &is_z0_z1_non_zero);

    let (((xp1, xp2), (yp1, yp2)), (zp1, zp2)) = rayon::join(
        || {
            rayon::join(
                || {
                    rayon::join(
                        || server_key.mul_parallelized(&x_prime, &is_z0_z1_non_zero),
                        || {
                            server_key.mul_parallelized(
                                &server_key.add_parallelized(x0, x1),
                                &not_is_z0_z1_non_zero.clone(),
                            )
                        },
                    )
                },
                || {
                    rayon::join(
                        || server_key.mul_parallelized(&y_prime, &is_z0_z1_non_zero),
                        || {
                            server_key.mul_parallelized(
                                &server_key.add_parallelized(y0, y1),
                                &not_is_z0_z1_non_zero,
                            )
                        },
                    )
                },
            )
        },
        || {
            rayon::join(
                || server_key.mul_parallelized(&z_prime, &is_z0_z1_non_zero),
                || {
                    server_key.mul_parallelized(
                        &server_key.add_parallelized(z0, z1),
                        &not_is_z0_z1_non_zero,
                    )
                },
            )
        },
    );

    ((x_prime, y_prime), z_prime) = rayon::join(
        || {
            rayon::join(
                || server_key.add_parallelized(&xp1, &xp2),
                || server_key.add_parallelized(&yp1, &yp2),
            )
        },
        || server_key.add_parallelized(&zp1, &zp2),
    );

    (x_prime, y_prime, z_prime)
}

/// homomorphic scalar mul for group elements in jacobian coordinates
pub fn group_projective_scalar_mul<const NB: usize, P: Numeral>(
    x: &RadixCiphertext,
    y: &RadixCiphertext,
    z: &RadixCiphertext,
    scalar: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext, RadixCiphertext) {
    let mut tmp_x = x.clone();
    let mut tmp_y = y.clone();
    let mut tmp_z = z.clone();
    let mut scalar = scalar.clone();
    let mut res_x = server_key.create_trivial_radix(0, NB);
    let mut res_y = server_key.create_trivial_radix(0, NB);
    let mut res_z = server_key.create_trivial_radix(0, NB);

    for _i in 0..<P as Numeric>::BITS {
        let (mut bit, new_scalar) = rayon::join(
            || server_key.scalar_bitand_parallelized(&scalar, 1),
            || server_key.scalar_right_shift_parallelized(&scalar, 1),
        );
        server_key.trim_radix_blocks_msb_assign(&mut bit, NB - 1);
        scalar = new_scalar;
        ((res_x, res_y, res_z), (tmp_x, tmp_y, tmp_z)) = rayon::join(
            || {
                let ((x_to_add, y_to_add), z_to_add) = rayon::join(
                    || {
                        rayon::join(
                            || server_key.mul_parallelized(&tmp_x, &bit),
                            || server_key.mul_parallelized(&tmp_y, &bit),
                        )
                    },
                    || server_key.mul_parallelized(&tmp_z, &bit),
                );
                group_projective_add_projective::<NB, _>(
                    &res_x, &res_y, &res_z, &x_to_add, &y_to_add, &z_to_add, p, server_key,
                )
            },
            || group_projective_double::<NB, _>(&tmp_x, &tmp_y, &tmp_z, p, server_key),
        );
    }

    (res_x, res_y, res_z)
}

/// homomorphic scalar mul for group elements in jacobian coordinates for constant group e.g. G
pub fn group_projective_scalar_mul_constant<const NB: usize, P: Numeral>(
    x: P,
    y: P,
    scalar: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext, RadixCiphertext) {
    let mut tmp_x = x;
    let mut tmp_y = y;
    let mut scalar = scalar.clone();
    let mut res_x = server_key.create_trivial_radix(0, NB);
    let mut res_y = server_key.create_trivial_radix(0, NB);
    let mut res_z = server_key.create_trivial_radix(0, NB);

    for _i in 0..<P as Numeric>::BITS {
        let (mut bit, new_scalar) = rayon::join(
            || server_key.scalar_bitand_parallelized(&scalar, 1),
            || server_key.scalar_right_shift_parallelized(&scalar, 1),
        );
        server_key.trim_radix_blocks_msb_assign(&mut bit, NB - 1);
        scalar = new_scalar;

        let (x_to_add, y_to_add) = rayon::join(
            || server_key.mul_parallelized(&server_key.create_trivial_radix(tmp_x, NB), &bit),
            || server_key.mul_parallelized(&server_key.create_trivial_radix(tmp_y, NB), &bit),
        );

        (res_x, res_y, res_z) = group_projective_add_affine::<NB, _>(
            &res_x, &res_y, &res_z, &x_to_add, &y_to_add, &bit, p, server_key,
        );

        (tmp_x, tmp_y) = {
            let (tmp_x_new, temp_y_new, temp_z_new) =
                group_projective_double_native(tmp_x, tmp_y, P::ONE, p);
            group_projective_into_affine_native(tmp_x_new, temp_y_new, temp_z_new, p)
        };
    }

    (res_x, res_y, res_z)
}

/// native scalar mul for group elements.
pub fn group_projective_scalar_mul_native<P: Numeral>(
    x: P,
    y: P,
    mut scalar: P,
    p: P,
) -> (P, P, P) {
    let mut tmp_x = x;
    let mut tmp_y = y;
    let mut res_x = P::ZERO;
    let mut res_y = P::ZERO;
    let mut res_z = P::ZERO;

    for _i in 0..P::BITS {
        let bit = scalar.bitand(P::ONE);
        scalar >>= 1;

        if bit == P::ONE {
            (res_x, res_y, res_z) =
                group_projective_add_affine_native(res_x, res_y, res_z, tmp_x, tmp_y, p);
        }

        let (tmp_x_new, temp_y_new, temp_z_new) =
            group_projective_double_native(tmp_x, tmp_y, P::ONE, p);
        (tmp_x, tmp_y) = group_projective_into_affine_native(tmp_x_new, temp_y_new, temp_z_new, p);
    }

    (res_x, res_y, res_z)
}

/// homomorphic scalar mul for group elements in jacobian coordinates for constant group e.g. G
/// W is the window size. 6 is the best window size for 256 bit on 64 cores machine.
#[time("info", "Group Projective Scalar Mul Windowed")]
pub fn group_projective_scalar_mul_constant_windowed<
    const W: usize,
    const NB: usize,
    P: Numeral,
>(
    x: P,
    y: P,
    scalar: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext, RadixCiphertext) {
    let mut tmp_x = x;
    let mut tmp_y = y;
    let mut scalar = scalar.clone();
    let mut res_x = server_key.create_trivial_radix(0, NB);
    let mut res_y = server_key.create_trivial_radix(0, NB);
    let mut res_z = server_key.create_trivial_radix(0, NB);

    // take W bits at a time
    // for each bit, we have a precomputed points of 2^W - 1 points
    // take the bit, and use it to select the point
    // add the point to the result
    // then double the temporary point W times
    let mut i = 0;
    while i < <P as Numeric>::BITS {
        let chunk_size = match i + W > <P as Numeric>::BITS {
            true => <P as Numeric>::BITS - i,
            false => W,
        };
        let _ic = i..i + chunk_size;
        i += chunk_size;

        let _tmr = stimer!(Level::Info; "Scalar Mul", "Bits {:?}", _ic);
        let cal_bits_tmr = timer!(Level::Debug; "Calculating bits");
        // get the next W bits
        let mut tmp_bits = vec![
            (
                server_key.create_trivial_radix(0, NB),
                server_key.create_trivial_radix(0, NB),
            );
            chunk_size
        ];
        (0..chunk_size)
            .into_par_iter()
            .map(|i| {
                let shifted = server_key.scalar_right_shift_parallelized(&scalar, i as u64);
                let mut bit = server_key.scalar_bitand_parallelized(&shifted, 1);
                server_key.trim_radix_blocks_msb_assign(&mut bit, NB - 1);
                (
                    server_key.sub_parallelized(&server_key.create_trivial_radix(P::ONE, 1), &bit),
                    bit,
                )
            })
            .collect_into_vec(&mut tmp_bits);
        let mut bits = vec![];
        let mut not_bits = vec![];
        for (not_bit, bit) in tmp_bits {
            not_bits.push(not_bit);
            bits.push(bit);
        }
        server_key.scalar_right_shift_assign_parallelized(&mut scalar, chunk_size as u64);
        drop(cal_bits_tmr);

        // get the precomputed values
        let mut points = vec![(P::ZERO, P::ZERO)];
        let tmp = (tmp_x, tmp_y);
        for _ in 1..2usize.pow(chunk_size as u32) {
            points.push((tmp_x, tmp_y));
            // points are stored in tmp
            (tmp_x, tmp_y) = {
                let (tmp_x_new, temp_y_new, temp_z_new) =
                    group_projective_add_affine_native(tmp_x, tmp_y, P::ONE, tmp.0, tmp.1, p);
                group_projective_into_affine_native(tmp_x_new, temp_y_new, temp_z_new, p)
            };
        }

        // select the points
        let sel_tmr = timer!(Level::Debug; "Selecting points", "Points {}", points.len() - 1);
        let mut points_to_add = vec![
            (
                server_key.create_trivial_radix(0, NB),
                server_key.create_trivial_radix(0, NB)
            );
            2usize.pow(chunk_size as u32) - 1
        ];
        points
            .into_par_iter()
            .enumerate()
            .take(2usize.pow(chunk_size as u32))
            .skip(1)
            .map(|(i, point)| {
                let bits = (0..chunk_size)
                    .map(|j| match i & 2usize.pow(j as u32) == 0 {
                        true => not_bits[j].clone(),
                        false => bits[j].clone(),
                    })
                    .collect::<Vec<_>>();
                let selected_bit =
                    parallel_fn(&bits, |b0, b1| server_key.bitand_parallelized(b0, b1));
                rayon::join(
                    || selector_zero_constant::<NB, _>(point.0, &selected_bit, server_key),
                    || selector_zero_constant::<NB, _>(point.1, &selected_bit, server_key),
                )
            })
            .collect_into_vec(&mut points_to_add);
        let selected_point = parallel_fn(&points_to_add, |p0, p1| {
            rayon::join(
                || server_key.add_parallelized(&p0.0, &p1.0),
                || server_key.add_parallelized(&p0.1, &p1.1),
            )
        });
        drop(sel_tmr);

        // check if all bits are not zero for flag bit
        let kary_or_tmr = timer!(Level::Debug; "Kary or");
        let all_not_zero = parallel_fn(&bits, |b0, b1| server_key.bitor_parallelized(b0, b1));
        drop(kary_or_tmr);

        // add the point
        (res_x, res_y, res_z) = group_projective_add_affine::<NB, _>(
            &res_x,
            &res_y,
            &res_z,
            &selected_point.0,
            &selected_point.1,
            &all_not_zero,
            p,
            server_key,
        );
    }

    (res_x, res_y, res_z)
}

/// homomorphic conversion from jacobian coordinates to affine coordinates
#[time("info", "Group Projective Into Affine")]
pub fn group_projective_into_affine<const NB: usize, P: Numeral>(
    x: &RadixCiphertext,
    y: &RadixCiphertext,
    z: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext) {
    let z_inv = inverse_mod::<NB, _>(z, p, server_key);
    let z_inv2 = square_mod::<NB, _>(&z_inv, p, server_key);
    let z_inv3 = mul_mod::<NB, _>(&z_inv2, &z_inv, p, server_key);

    rayon::join(
        || mul_mod::<NB, _>(x, &z_inv2, p, server_key),
        || mul_mod::<NB, _>(y, &z_inv3, p, server_key),
    )
}
/// homomorphic conversion from jacobian coordinates to affine coordinates with inverted z
/// z inverse is can be computed outside of the function.
#[time("info", "Group Projective Into Affine Inversed")]
pub fn group_projective_into_affine_inv<const NB: usize, P: Numeral>(
    x: &RadixCiphertext,
    y: &RadixCiphertext,
    z_inv: &RadixCiphertext,
    p: P,
    server_key: &ServerKey,
) -> (RadixCiphertext, RadixCiphertext) {
    let z_inv2 = square_mod::<NB, _>(z_inv, p, server_key);
    let z_inv3 = mul_mod::<NB, _>(&z_inv2, z_inv, p, server_key);

    rayon::join(
        || mul_mod::<NB, _>(x, &z_inv2, p, server_key),
        || mul_mod::<NB, _>(y, &z_inv3, p, server_key),
    )
}
/// native conversion from jacobian coordinates to affine coordinates
pub fn group_projective_into_affine_native<P: Numeral>(x: P, y: P, z: P, p: P) -> (P, P) {
    let z_inv = inverse_mod_native(z, p);
    let z_inv2 = square_mod_native(z_inv, p);
    let z_inv3 = mul_mod_native(z_inv2, z_inv, p);

    (mul_mod_native(x, z_inv2, p), mul_mod_native(y, z_inv3, p))
}

#[cfg(test)]
mod tests {

    use tfhe::{
        integer::{keycache::IntegerKeyCache, IntegerKeyKind},
        shortint::prelude::PARAM_MESSAGE_2_CARRY_2,
    };

    use crate::{
        ops::group_jacobian::{
            group_projective_add_affine, group_projective_add_affine_native,
            group_projective_double, group_projective_double_native, group_projective_into_affine,
            group_projective_into_affine_native,
        },
        WINDOW,
    };

    use super::{
        group_projective_scalar_mul_constant_windowed, group_projective_scalar_mul_native,
    };

    #[test]
    fn correct_jacobian_double() {
        let (client_key, server_key) =
            IntegerKeyCache.get_from_params(PARAM_MESSAGE_2_CARRY_2, IntegerKeyKind::Radix);

        const NUM_BLOCK: usize = 4;
        type Integer = u8;
        let p: Integer = 251;
        let x1: Integer = 8;
        let y1: Integer = 45;

        let ct_x1 = client_key.encrypt_radix(x1, NUM_BLOCK);
        let ct_y1 = client_key.encrypt_radix(y1, NUM_BLOCK);

        let (x_new, y_new, z_new) = group_projective_double::<NUM_BLOCK, _>(
            &ct_x1,
            &ct_y1,
            &server_key.create_trivial_radix(1, NUM_BLOCK),
            p,
            &server_key,
        );
        let x_dec = client_key.decrypt_radix::<Integer>(&x_new);
        let y_dec = client_key.decrypt_radix::<Integer>(&y_new);
        let z_dec = client_key.decrypt_radix::<Integer>(&z_new);

        let res = group_projective_double_native(x1, y1, 1, p);

        assert_eq!(x_dec, res.0);
        assert_eq!(y_dec, res.1);
        assert_eq!(z_dec, res.2);
    }

    #[test]
    fn correct_jacobian_add_affine() {
        let (client_key, server_key) =
            IntegerKeyCache.get_from_params(PARAM_MESSAGE_2_CARRY_2, IntegerKeyKind::Radix);

        const NUM_BLOCK: usize = 4;
        type Integer = u8;
        let p: Integer = 251;
        let x1: Integer = 48;
        let y1: Integer = 68;
        let z1: Integer = 153;
        let x2: Integer = 56;
        let y2: Integer = 225;

        let ct_x1 = client_key.encrypt_radix(x1, NUM_BLOCK);
        let ct_y1 = client_key.encrypt_radix(y1, NUM_BLOCK);
        let ct_z1 = client_key.encrypt_radix(z1, NUM_BLOCK);
        let ct_x2 = client_key.encrypt_radix(x2, NUM_BLOCK);
        let ct_y2 = client_key.encrypt_radix(y2, NUM_BLOCK);

        let (x_new, y_new, z_new) = group_projective_add_affine::<NUM_BLOCK, _>(
            &ct_x1,
            &ct_y1,
            &ct_z1,
            &ct_x2,
            &ct_y2,
            &client_key.encrypt_radix(1u8, NUM_BLOCK),
            p,
            &server_key,
        );
        let x_dec = client_key.decrypt_radix::<Integer>(&x_new);
        let y_dec = client_key.decrypt_radix::<Integer>(&y_new);
        let z_dec = client_key.decrypt_radix::<Integer>(&z_new);

        let res = group_projective_add_affine_native(x1, y1, z1, x2, y2, p);

        assert_eq!(x_dec, res.0);
        assert_eq!(y_dec, res.1);
        assert_eq!(z_dec, res.2);
    }

    #[test]
    fn correct_jacobian_scalar_mul() {
        let (client_key, server_key) =
            IntegerKeyCache.get_from_params(PARAM_MESSAGE_2_CARRY_2, IntegerKeyKind::Radix);

        const NUM_BLOCK: usize = 4;
        type Integer = u8;
        let p: u8 = 251;
        let x: u8 = 8;
        let y: u8 = 45;
        let scalar: u8 = 6;
        let ct_scalar = client_key.encrypt_radix(scalar, NUM_BLOCK);

        let (x_new, y_new, z_new) = group_projective_scalar_mul_constant_windowed::<
            WINDOW,
            NUM_BLOCK,
            _,
        >(x, y, &ct_scalar, p, &server_key);
        let (x_final, y_final) =
            group_projective_into_affine::<NUM_BLOCK, _>(&x_new, &y_new, &z_new, p, &server_key);

        let x_dec = client_key.decrypt_radix::<Integer>(&x_final);
        let y_dec = client_key.decrypt_radix::<Integer>(&y_final);

        let res = group_projective_scalar_mul_native(x, y, scalar, p);
        let res = group_projective_into_affine_native(res.0, res.1, res.2, p);

        assert_eq!(x_dec, res.0);
        assert_eq!(y_dec, res.1);
    }

    #[test]
    fn correct_native_group_ops_jacobian() {
        let p: u8 = 251;
        let x: u8 = 8;
        let y: u8 = 45;
        let z: u8 = 1;

        let (xp, yp, zp) = group_projective_double_native(x, y, z, p);
        let (xn, yn) = group_projective_into_affine_native(xp, yp, zp, p);

        assert_eq!(xn, 157);
        assert_eq!(yn, 22);
    }

    #[test]
    fn correct_native_scalar_mul() {
        let p: u8 = 251;
        let x: u8 = 8;
        let y: u8 = 45;

        let scalar: u8 = 6;
        let g = group_projective_scalar_mul_native(x, y, scalar, p);
        let affine = group_projective_into_affine_native(g.0, g.1, g.2, p);
        assert_eq!(affine, (176, 125));

        let scalar: u8 = 26;
        let g = group_projective_scalar_mul_native(x, y, scalar, p);
        let affine = group_projective_into_affine_native(g.0, g.1, g.2, p);
        assert_eq!(affine, (92, 120));
    }
}
