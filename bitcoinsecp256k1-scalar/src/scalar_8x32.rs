// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_8x32.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_8x32.h]

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
pub struct Scalar {
    pub d: [u32; 8],
}

impl Scalar {
    pub const fn new() -> Self {
        Self {
            d: [0; 8],
        }
    }
}

#[macro_export] macro_rules! scalar_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Scalar {
            d: [
                $d0, 
                $d1, 
                $d2, 
                $d3, 
                $d4, 
                $d5, 
                $d6, 
                $d7
            ]
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_8x32_impl.h]

/**
  | Limbs of the secp256k1 order.
  |
  */
pub const N_0: u32 = 0xD0364141;
pub const N_1: u32 = 0xBFD25E8C;
pub const N_2: u32 = 0xAF48A03B;
pub const N_3: u32 = 0xBAAEDCE6;
pub const N_4: u32 = 0xFFFFFFFE;
pub const N_5: u32 = 0xFFFFFFFF;
pub const N_6: u32 = 0xFFFFFFFF;
pub const N_7: u32 = 0xFFFFFFFF;

/**
  | Limbs of 2^256 minus the secp256k1 order.
  |
  */
pub const N_C_0: u32 = !N_0 + 1;
pub const N_C_1: u32 = !N_1;
pub const N_C_2: u32 = !N_2;
pub const N_C_3: u32 = !N_3;
pub const N_C_4: u32 = 1;

/**
  | Limbs of half the secp256k1 order.
  |
  */
pub const N_H_0: u32 = 0x681B20A0;
pub const N_H_1: u32 = 0xDFE92F46;
pub const N_H_2: u32 = 0x57A4501D;
pub const N_H_3: u32 = 0x5D576E73;
pub const N_H_4: u32 = 0xFFFFFFFF;
pub const N_H_5: u32 = 0xFFFFFFFF;
pub const N_H_6: u32 = 0xFFFFFFFF;
pub const N_H_7: u32 = 0x7FFFFFFF;

#[inline] pub fn scalar_clear(r: *mut Scalar)  {
    
    todo!();
        /*
            r->d[0] = 0;
        r->d[1] = 0;
        r->d[2] = 0;
        r->d[3] = 0;
        r->d[4] = 0;
        r->d[5] = 0;
        r->d[6] = 0;
        r->d[7] = 0;
        */
}

#[inline] pub fn scalar_set_int(
        r: *mut Scalar,
        v: u32)  {
    
    todo!();
        /*
            r->d[0] = v;
        r->d[1] = 0;
        r->d[2] = 0;
        r->d[3] = 0;
        r->d[4] = 0;
        r->d[5] = 0;
        r->d[6] = 0;
        r->d[7] = 0;
        */
}

#[inline] pub fn scalar_get_bits(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            VERIFY_CHECK((offset + count - 1) >> 5 == offset >> 5);
        return (a->d[offset >> 5] >> (offset & 0x1F)) & ((1 << count) - 1);
        */
}

#[inline] pub fn scalar_get_bits_var(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            VERIFY_CHECK(count < 32);
        VERIFY_CHECK(offset + count <= 256);
        if ((offset + count - 1) >> 5 == offset >> 5) {
            return scalar_get_bits(a, offset, count);
        } else {
            VERIFY_CHECK((offset >> 5) + 1 < 8);
            return ((a->d[offset >> 5] >> (offset & 0x1F)) | (a->d[(offset >> 5) + 1] << (32 - (offset & 0x1F)))) & ((((uint32_t)1) << count) - 1);
        }
        */
}

#[inline] pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            int yes = 0;
        int no = 0;
        no |= (a->d[7] < N_7); /* No need for a > check. */
        no |= (a->d[6] < N_6); /* No need for a > check. */
        no |= (a->d[5] < N_5); /* No need for a > check. */
        no |= (a->d[4] < N_4);
        yes |= (a->d[4] > N_4) & ~no;
        no |= (a->d[3] < N_3) & ~yes;
        yes |= (a->d[3] > N_3) & ~no;
        no |= (a->d[2] < N_2) & ~yes;
        yes |= (a->d[2] > N_2) & ~no;
        no |= (a->d[1] < N_1) & ~yes;
        yes |= (a->d[1] > N_1) & ~no;
        yes |= (a->d[0] >= N_0) & ~no;
        return yes;
        */
}

#[inline] pub fn scalar_reduce(
        r:        *mut Scalar,
        overflow: u32) -> i32 {
    
    todo!();
        /*
            uint64_t t;
        VERIFY_CHECK(overflow <= 1);
        t = (uint64_t)r->d[0] + overflow * N_C_0;
        r->d[0] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[1] + overflow * N_C_1;
        r->d[1] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[2] + overflow * N_C_2;
        r->d[2] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[3] + overflow * N_C_3;
        r->d[3] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[4] + overflow * N_C_4;
        r->d[4] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[5];
        r->d[5] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[6];
        r->d[6] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[7];
        r->d[7] = t & 0xFFFFFFFFUL;
        return overflow;
        */
}

pub fn scalar_add(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            int overflow;
        uint64_t t = (uint64_t)a->d[0] + b->d[0];
        r->d[0] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[1] + b->d[1];
        r->d[1] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[2] + b->d[2];
        r->d[2] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[3] + b->d[3];
        r->d[3] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[4] + b->d[4];
        r->d[4] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[5] + b->d[5];
        r->d[5] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[6] + b->d[6];
        r->d[6] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[7] + b->d[7];
        r->d[7] = t & 0xFFFFFFFFULL; t >>= 32;
        overflow = t + scalar_check_overflow(r);
        VERIFY_CHECK(overflow == 0 || overflow == 1);
        scalar_reduce(r, overflow);
        return overflow;
        */
}

pub fn scalar_cadd_bit(
        r:    *mut Scalar,
        bit:  u32,
        flag: i32)  {
    
    todo!();
        /*
            uint64_t t;
        VERIFY_CHECK(bit < 256);
        bit += ((uint32_t) flag - 1) & 0x100;  /* forcing (bit >> 5) > 7 makes this a noop */
        t = (uint64_t)r->d[0] + (((uint32_t)((bit >> 5) == 0)) << (bit & 0x1F));
        r->d[0] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[1] + (((uint32_t)((bit >> 5) == 1)) << (bit & 0x1F));
        r->d[1] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[2] + (((uint32_t)((bit >> 5) == 2)) << (bit & 0x1F));
        r->d[2] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[3] + (((uint32_t)((bit >> 5) == 3)) << (bit & 0x1F));
        r->d[3] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[4] + (((uint32_t)((bit >> 5) == 4)) << (bit & 0x1F));
        r->d[4] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[5] + (((uint32_t)((bit >> 5) == 5)) << (bit & 0x1F));
        r->d[5] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[6] + (((uint32_t)((bit >> 5) == 6)) << (bit & 0x1F));
        r->d[6] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[7] + (((uint32_t)((bit >> 5) == 7)) << (bit & 0x1F));
        r->d[7] = t & 0xFFFFFFFFULL;
    #ifdef VERIFY
        VERIFY_CHECK((t >> 32) == 0);
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}

pub fn scalar_set_b32(
        r:        *mut Scalar,
        b32:      *const u8,
        overflow: *mut i32)  {
    
    todo!();
        /*
            int over;
        r->d[0] = (uint32_t)b32[31] | (uint32_t)b32[30] << 8 | (uint32_t)b32[29] << 16 | (uint32_t)b32[28] << 24;
        r->d[1] = (uint32_t)b32[27] | (uint32_t)b32[26] << 8 | (uint32_t)b32[25] << 16 | (uint32_t)b32[24] << 24;
        r->d[2] = (uint32_t)b32[23] | (uint32_t)b32[22] << 8 | (uint32_t)b32[21] << 16 | (uint32_t)b32[20] << 24;
        r->d[3] = (uint32_t)b32[19] | (uint32_t)b32[18] << 8 | (uint32_t)b32[17] << 16 | (uint32_t)b32[16] << 24;
        r->d[4] = (uint32_t)b32[15] | (uint32_t)b32[14] << 8 | (uint32_t)b32[13] << 16 | (uint32_t)b32[12] << 24;
        r->d[5] = (uint32_t)b32[11] | (uint32_t)b32[10] << 8 | (uint32_t)b32[9] << 16 | (uint32_t)b32[8] << 24;
        r->d[6] = (uint32_t)b32[7] | (uint32_t)b32[6] << 8 | (uint32_t)b32[5] << 16 | (uint32_t)b32[4] << 24;
        r->d[7] = (uint32_t)b32[3] | (uint32_t)b32[2] << 8 | (uint32_t)b32[1] << 16 | (uint32_t)b32[0] << 24;
        over = scalar_reduce(r, scalar_check_overflow(r));
        if (overflow) {
            *overflow = over;
        }
        */
}

pub fn scalar_get_b32(
        bin: *mut u8,
        a:   *const Scalar)  {
    
    todo!();
        /*
            bin[0] = a->d[7] >> 24; bin[1] = a->d[7] >> 16; bin[2] = a->d[7] >> 8; bin[3] = a->d[7];
        bin[4] = a->d[6] >> 24; bin[5] = a->d[6] >> 16; bin[6] = a->d[6] >> 8; bin[7] = a->d[6];
        bin[8] = a->d[5] >> 24; bin[9] = a->d[5] >> 16; bin[10] = a->d[5] >> 8; bin[11] = a->d[5];
        bin[12] = a->d[4] >> 24; bin[13] = a->d[4] >> 16; bin[14] = a->d[4] >> 8; bin[15] = a->d[4];
        bin[16] = a->d[3] >> 24; bin[17] = a->d[3] >> 16; bin[18] = a->d[3] >> 8; bin[19] = a->d[3];
        bin[20] = a->d[2] >> 24; bin[21] = a->d[2] >> 16; bin[22] = a->d[2] >> 8; bin[23] = a->d[2];
        bin[24] = a->d[1] >> 24; bin[25] = a->d[1] >> 16; bin[26] = a->d[1] >> 8; bin[27] = a->d[1];
        bin[28] = a->d[0] >> 24; bin[29] = a->d[0] >> 16; bin[30] = a->d[0] >> 8; bin[31] = a->d[0];
        */
}

#[inline] pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return (a->d[0] | a->d[1] | a->d[2] | a->d[3] | a->d[4] | a->d[5] | a->d[6] | a->d[7]) == 0;
        */
}

pub fn scalar_negate(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
            uint32_t nonzero = 0xFFFFFFFFUL * (scalar_is_zero(a) == 0);
        uint64_t t = (uint64_t)(~a->d[0]) + N_0 + 1;
        r->d[0] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[1]) + N_1;
        r->d[1] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[2]) + N_2;
        r->d[2] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[3]) + N_3;
        r->d[3] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[4]) + N_4;
        r->d[4] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[5]) + N_5;
        r->d[5] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[6]) + N_6;
        r->d[6] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[7]) + N_7;
        r->d[7] = t & nonzero;
        */
}

#[inline] pub fn scalar_is_one(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return ((a->d[0] ^ 1) | a->d[1] | a->d[2] | a->d[3] | a->d[4] | a->d[5] | a->d[6] | a->d[7]) == 0;
        */
}

pub fn scalar_is_high(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            int yes = 0;
        int no = 0;
        no |= (a->d[7] < N_H_7);
        yes |= (a->d[7] > N_H_7) & ~no;
        no |= (a->d[6] < N_H_6) & ~yes; /* No need for a > check. */
        no |= (a->d[5] < N_H_5) & ~yes; /* No need for a > check. */
        no |= (a->d[4] < N_H_4) & ~yes; /* No need for a > check. */
        no |= (a->d[3] < N_H_3) & ~yes;
        yes |= (a->d[3] > N_H_3) & ~no;
        no |= (a->d[2] < N_H_2) & ~yes;
        yes |= (a->d[2] > N_H_2) & ~no;
        no |= (a->d[1] < N_H_1) & ~yes;
        yes |= (a->d[1] > N_H_1) & ~no;
        yes |= (a->d[0] > N_H_0) & ~no;
        return yes;
        */
}

pub fn scalar_cond_negate(
        r:    *mut Scalar,
        flag: i32) -> i32 {
    
    todo!();
        /*
            /* If we are flag = 0, mask = 00...00 and this is a no-op;
         * if we are flag = 1, mask = 11...11 and this is identical to scalar_negate */
        uint32_t mask = !flag - 1;
        uint32_t nonzero = 0xFFFFFFFFUL * (scalar_is_zero(r) == 0);
        uint64_t t = (uint64_t)(r->d[0] ^ mask) + ((N_0 + 1) & mask);
        r->d[0] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[1] ^ mask) + (N_1 & mask);
        r->d[1] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[2] ^ mask) + (N_2 & mask);
        r->d[2] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[3] ^ mask) + (N_3 & mask);
        r->d[3] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[4] ^ mask) + (N_4 & mask);
        r->d[4] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[5] ^ mask) + (N_5 & mask);
        r->d[5] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[6] ^ mask) + (N_6 & mask);
        r->d[6] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[7] ^ mask) + (N_7 & mask);
        r->d[7] = t & nonzero;
        return 2 * (mask == 0) - 1;
        */
}

/*
  | Inspired by the macros in OpenSSL's
  | crypto/bn/asm/x86_64-gcc.c.
  |
  */

/**
  | Add a*b to the number defined by (c0,c1,c2).
  | c2 must never overflow.
  |
  */
#[macro_export] macro_rules! muladd {
    ($a:ident, $b:ident) => {
        /*
                { 
            uint32_t tl, th; 
            { 
                uint64_t t = (uint64_t)a * b; 
                th = t >> 32;         /* at most 0xFFFFFFFE */ 
                tl = t; 
            } 
            c0 += tl;                 /* overflow is handled on the next line */ 
            th += (c0 < tl);          /* at most 0xFFFFFFFF */ 
            c1 += th;                 /* overflow is handled on the next line */ 
            c2 += (c1 < th);          /* never overflows by contract (verified in the next line) */ 
            VERIFY_CHECK((c1 >= th) || (c2 != 0)); 
        }
        */
    }
}

/**
  | Add a*b to the number defined by (c0,c1).
  | c1 must never overflow.
  |
  */
#[macro_export] macro_rules! muladd_fast {
    ($a:ident, $b:ident) => {
        /*
                { 
            uint32_t tl, th; 
            { 
                uint64_t t = (uint64_t)a * b; 
                th = t >> 32;         /* at most 0xFFFFFFFE */ 
                tl = t; 
            } 
            c0 += tl;                 /* overflow is handled on the next line */ 
            th += (c0 < tl);          /* at most 0xFFFFFFFF */ 
            c1 += th;                 /* never overflows by contract (verified in the next line) */ 
            VERIFY_CHECK(c1 >= th); 
        }
        */
    }
}

/**
  | Add a to the number defined by (c0,c1,c2).
  | c2 must never overflow.
  |
  */
#[macro_export] macro_rules! sumadd {
    ($a:ident) => {
        /*
                { 
            unsigned int over; 
            c0 += (a);                  /* overflow is handled on the next line */ 
            over = (c0 < (a)); 
            c1 += over;                 /* overflow is handled on the next line */ 
            c2 += (c1 < over);          /* never overflows by contract */ 
        }
        */
    }
}

/**
  | Add a to the number defined by (c0,c1).
  | c1 must never overflow, c2 must be zero.
  |
  */
#[macro_export] macro_rules! sumadd_fast {
    ($a:ident) => {
        /*
                { 
            c0 += (a);                 /* overflow is handled on the next line */ 
            c1 += (c0 < (a));          /* never overflows by contract (verified the next line) */ 
            VERIFY_CHECK((c1 != 0) | (c0 >= (a))); 
            VERIFY_CHECK(c2 == 0); 
        }
        */
    }
}

/**
  | Extract the lowest 32 bits of (c0,c1,c2)
  | into n, and left shift the number 32 bits.
  |
  */
#[macro_export] macro_rules! extract {
    ($n:ident) => {
        /*
                { 
            (n) = c0; 
            c0 = c1; 
            c1 = c2; 
            c2 = 0; 
        }
        */
    }
}

/**
  | Extract the lowest 32 bits of (c0,c1,c2)
  | into n, and left shift the number 32 bits.
  | c2 is required to be zero.
  |
  */
#[macro_export] macro_rules! extract_fast {
    ($n:ident) => {
        /*
                { 
            (n) = c0; 
            c0 = c1; 
            c1 = 0; 
            VERIFY_CHECK(c2 == 0); 
        }
        */
    }
}

pub fn scalar_reduce_512(
        r: *mut Scalar,
        l: *const u32)  {
    
    todo!();
        /*
            uint64_t c;
        uint32_t n0 = l[8], n1 = l[9], n2 = l[10], n3 = l[11], n4 = l[12], n5 = l[13], n6 = l[14], n7 = l[15];
        uint32_t m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12;
        uint32_t p0, p1, p2, p3, p4, p5, p6, p7, p8;

        /* 96 bit accumulator. */
        uint32_t c0, c1, c2;

        /* Reduce 512 bits into 385. */
        /* m[0..12] = l[0..7] + n[0..7] * N_C. */
        c0 = l[0]; c1 = 0; c2 = 0;
        muladd_fast(n0, N_C_0);
        extract_fast(m0);
        sumadd_fast(l[1]);
        muladd(n1, N_C_0);
        muladd(n0, N_C_1);
        extract(m1);
        sumadd(l[2]);
        muladd(n2, N_C_0);
        muladd(n1, N_C_1);
        muladd(n0, N_C_2);
        extract(m2);
        sumadd(l[3]);
        muladd(n3, N_C_0);
        muladd(n2, N_C_1);
        muladd(n1, N_C_2);
        muladd(n0, N_C_3);
        extract(m3);
        sumadd(l[4]);
        muladd(n4, N_C_0);
        muladd(n3, N_C_1);
        muladd(n2, N_C_2);
        muladd(n1, N_C_3);
        sumadd(n0);
        extract(m4);
        sumadd(l[5]);
        muladd(n5, N_C_0);
        muladd(n4, N_C_1);
        muladd(n3, N_C_2);
        muladd(n2, N_C_3);
        sumadd(n1);
        extract(m5);
        sumadd(l[6]);
        muladd(n6, N_C_0);
        muladd(n5, N_C_1);
        muladd(n4, N_C_2);
        muladd(n3, N_C_3);
        sumadd(n2);
        extract(m6);
        sumadd(l[7]);
        muladd(n7, N_C_0);
        muladd(n6, N_C_1);
        muladd(n5, N_C_2);
        muladd(n4, N_C_3);
        sumadd(n3);
        extract(m7);
        muladd(n7, N_C_1);
        muladd(n6, N_C_2);
        muladd(n5, N_C_3);
        sumadd(n4);
        extract(m8);
        muladd(n7, N_C_2);
        muladd(n6, N_C_3);
        sumadd(n5);
        extract(m9);
        muladd(n7, N_C_3);
        sumadd(n6);
        extract(m10);
        sumadd_fast(n7);
        extract_fast(m11);
        VERIFY_CHECK(c0 <= 1);
        m12 = c0;

        /* Reduce 385 bits into 258. */
        /* p[0..8] = m[0..7] + m[8..12] * N_C. */
        c0 = m0; c1 = 0; c2 = 0;
        muladd_fast(m8, N_C_0);
        extract_fast(p0);
        sumadd_fast(m1);
        muladd(m9, N_C_0);
        muladd(m8, N_C_1);
        extract(p1);
        sumadd(m2);
        muladd(m10, N_C_0);
        muladd(m9, N_C_1);
        muladd(m8, N_C_2);
        extract(p2);
        sumadd(m3);
        muladd(m11, N_C_0);
        muladd(m10, N_C_1);
        muladd(m9, N_C_2);
        muladd(m8, N_C_3);
        extract(p3);
        sumadd(m4);
        muladd(m12, N_C_0);
        muladd(m11, N_C_1);
        muladd(m10, N_C_2);
        muladd(m9, N_C_3);
        sumadd(m8);
        extract(p4);
        sumadd(m5);
        muladd(m12, N_C_1);
        muladd(m11, N_C_2);
        muladd(m10, N_C_3);
        sumadd(m9);
        extract(p5);
        sumadd(m6);
        muladd(m12, N_C_2);
        muladd(m11, N_C_3);
        sumadd(m10);
        extract(p6);
        sumadd_fast(m7);
        muladd_fast(m12, N_C_3);
        sumadd_fast(m11);
        extract_fast(p7);
        p8 = c0 + m12;
        VERIFY_CHECK(p8 <= 2);

        /* Reduce 258 bits into 256. */
        /* r[0..7] = p[0..7] + p[8] * N_C. */
        c = p0 + (uint64_t)N_C_0 * p8;
        r->d[0] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p1 + (uint64_t)N_C_1 * p8;
        r->d[1] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p2 + (uint64_t)N_C_2 * p8;
        r->d[2] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p3 + (uint64_t)N_C_3 * p8;
        r->d[3] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p4 + (uint64_t)p8;
        r->d[4] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p5;
        r->d[5] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p6;
        r->d[6] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p7;
        r->d[7] = c & 0xFFFFFFFFUL; c >>= 32;

        /* Final reduction of r. */
        scalar_reduce(r, c + scalar_check_overflow(r));
        */
}

pub fn scalar_mul_512(
        l: *mut u32,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            /* 96 bit accumulator. */
        uint32_t c0 = 0, c1 = 0, c2 = 0;

        /* l[0..15] = a[0..7] * b[0..7]. */
        muladd_fast(a->d[0], b->d[0]);
        extract_fast(l[0]);
        muladd(a->d[0], b->d[1]);
        muladd(a->d[1], b->d[0]);
        extract(l[1]);
        muladd(a->d[0], b->d[2]);
        muladd(a->d[1], b->d[1]);
        muladd(a->d[2], b->d[0]);
        extract(l[2]);
        muladd(a->d[0], b->d[3]);
        muladd(a->d[1], b->d[2]);
        muladd(a->d[2], b->d[1]);
        muladd(a->d[3], b->d[0]);
        extract(l[3]);
        muladd(a->d[0], b->d[4]);
        muladd(a->d[1], b->d[3]);
        muladd(a->d[2], b->d[2]);
        muladd(a->d[3], b->d[1]);
        muladd(a->d[4], b->d[0]);
        extract(l[4]);
        muladd(a->d[0], b->d[5]);
        muladd(a->d[1], b->d[4]);
        muladd(a->d[2], b->d[3]);
        muladd(a->d[3], b->d[2]);
        muladd(a->d[4], b->d[1]);
        muladd(a->d[5], b->d[0]);
        extract(l[5]);
        muladd(a->d[0], b->d[6]);
        muladd(a->d[1], b->d[5]);
        muladd(a->d[2], b->d[4]);
        muladd(a->d[3], b->d[3]);
        muladd(a->d[4], b->d[2]);
        muladd(a->d[5], b->d[1]);
        muladd(a->d[6], b->d[0]);
        extract(l[6]);
        muladd(a->d[0], b->d[7]);
        muladd(a->d[1], b->d[6]);
        muladd(a->d[2], b->d[5]);
        muladd(a->d[3], b->d[4]);
        muladd(a->d[4], b->d[3]);
        muladd(a->d[5], b->d[2]);
        muladd(a->d[6], b->d[1]);
        muladd(a->d[7], b->d[0]);
        extract(l[7]);
        muladd(a->d[1], b->d[7]);
        muladd(a->d[2], b->d[6]);
        muladd(a->d[3], b->d[5]);
        muladd(a->d[4], b->d[4]);
        muladd(a->d[5], b->d[3]);
        muladd(a->d[6], b->d[2]);
        muladd(a->d[7], b->d[1]);
        extract(l[8]);
        muladd(a->d[2], b->d[7]);
        muladd(a->d[3], b->d[6]);
        muladd(a->d[4], b->d[5]);
        muladd(a->d[5], b->d[4]);
        muladd(a->d[6], b->d[3]);
        muladd(a->d[7], b->d[2]);
        extract(l[9]);
        muladd(a->d[3], b->d[7]);
        muladd(a->d[4], b->d[6]);
        muladd(a->d[5], b->d[5]);
        muladd(a->d[6], b->d[4]);
        muladd(a->d[7], b->d[3]);
        extract(l[10]);
        muladd(a->d[4], b->d[7]);
        muladd(a->d[5], b->d[6]);
        muladd(a->d[6], b->d[5]);
        muladd(a->d[7], b->d[4]);
        extract(l[11]);
        muladd(a->d[5], b->d[7]);
        muladd(a->d[6], b->d[6]);
        muladd(a->d[7], b->d[5]);
        extract(l[12]);
        muladd(a->d[6], b->d[7]);
        muladd(a->d[7], b->d[6]);
        extract(l[13]);
        muladd_fast(a->d[7], b->d[7]);
        extract_fast(l[14]);
        VERIFY_CHECK(c1 == 0);
        l[15] = c0;
        */
}

pub fn scalar_mul(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            uint32_t l[16];
        scalar_mul_512(l, a, b);
        scalar_reduce_512(r, l);
        */
}

pub fn scalar_shr_int(
        r: *mut Scalar,
        n: i32) -> i32 {
    
    todo!();
        /*
            int ret;
        VERIFY_CHECK(n > 0);
        VERIFY_CHECK(n < 16);
        ret = r->d[0] & ((1 << n) - 1);
        r->d[0] = (r->d[0] >> n) + (r->d[1] << (32 - n));
        r->d[1] = (r->d[1] >> n) + (r->d[2] << (32 - n));
        r->d[2] = (r->d[2] >> n) + (r->d[3] << (32 - n));
        r->d[3] = (r->d[3] >> n) + (r->d[4] << (32 - n));
        r->d[4] = (r->d[4] >> n) + (r->d[5] << (32 - n));
        r->d[5] = (r->d[5] >> n) + (r->d[6] << (32 - n));
        r->d[6] = (r->d[6] >> n) + (r->d[7] << (32 - n));
        r->d[7] = (r->d[7] >> n);
        return ret;
        */
}

pub fn scalar_split_128(
        r1: *mut Scalar,
        r2: *mut Scalar,
        k:  *const Scalar)  {
    
    todo!();
        /*
            r1->d[0] = k->d[0];
        r1->d[1] = k->d[1];
        r1->d[2] = k->d[2];
        r1->d[3] = k->d[3];
        r1->d[4] = 0;
        r1->d[5] = 0;
        r1->d[6] = 0;
        r1->d[7] = 0;
        r2->d[0] = k->d[4];
        r2->d[1] = k->d[5];
        r2->d[2] = k->d[6];
        r2->d[3] = k->d[7];
        r2->d[4] = 0;
        r2->d[5] = 0;
        r2->d[6] = 0;
        r2->d[7] = 0;
        */
}

#[inline] pub fn scalar_eq(
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            return ((a->d[0] ^ b->d[0]) | (a->d[1] ^ b->d[1]) | (a->d[2] ^ b->d[2]) | (a->d[3] ^ b->d[3]) | (a->d[4] ^ b->d[4]) | (a->d[5] ^ b->d[5]) | (a->d[6] ^ b->d[6]) | (a->d[7] ^ b->d[7])) == 0;
        */
}

#[inline] pub fn scalar_mul_shift_var(
        r:     *mut Scalar,
        a:     *const Scalar,
        b:     *const Scalar,
        shift: u32)  {
    
    todo!();
        /*
            uint32_t l[16];
        unsigned int shiftlimbs;
        unsigned int shiftlow;
        unsigned int shifthigh;
        VERIFY_CHECK(shift >= 256);
        scalar_mul_512(l, a, b);
        shiftlimbs = shift >> 5;
        shiftlow = shift & 0x1F;
        shifthigh = 32 - shiftlow;
        r->d[0] = shift < 512 ? (l[0 + shiftlimbs] >> shiftlow | (shift < 480 && shiftlow ? (l[1 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[1] = shift < 480 ? (l[1 + shiftlimbs] >> shiftlow | (shift < 448 && shiftlow ? (l[2 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[2] = shift < 448 ? (l[2 + shiftlimbs] >> shiftlow | (shift < 416 && shiftlow ? (l[3 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[3] = shift < 416 ? (l[3 + shiftlimbs] >> shiftlow | (shift < 384 && shiftlow ? (l[4 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[4] = shift < 384 ? (l[4 + shiftlimbs] >> shiftlow | (shift < 352 && shiftlow ? (l[5 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[5] = shift < 352 ? (l[5 + shiftlimbs] >> shiftlow | (shift < 320 && shiftlow ? (l[6 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[6] = shift < 320 ? (l[6 + shiftlimbs] >> shiftlow | (shift < 288 && shiftlow ? (l[7 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[7] = shift < 288 ? (l[7 + shiftlimbs] >> shiftlow)  : 0;
        scalar_cadd_bit(r, 0, (l[(shift - 1) >> 5] >> ((shift - 1) & 0x1f)) & 1);
        */
}

#[inline] pub fn scalar_cmov(
        r:    *mut Scalar,
        a:    *const Scalar,
        flag: i32)  {
    
    todo!();
        /*
            uint32_t mask0, mask1;
        VG_CHECK_VERIFY(r->d, sizeof(r->d));
        mask0 = flag + ~((uint32_t)0);
        mask1 = ~mask0;
        r->d[0] = (r->d[0] & mask0) | (a->d[0] & mask1);
        r->d[1] = (r->d[1] & mask0) | (a->d[1] & mask1);
        r->d[2] = (r->d[2] & mask0) | (a->d[2] & mask1);
        r->d[3] = (r->d[3] & mask0) | (a->d[3] & mask1);
        r->d[4] = (r->d[4] & mask0) | (a->d[4] & mask1);
        r->d[5] = (r->d[5] & mask0) | (a->d[5] & mask1);
        r->d[6] = (r->d[6] & mask0) | (a->d[6] & mask1);
        r->d[7] = (r->d[7] & mask0) | (a->d[7] & mask1);
        */
}

pub fn scalar_from_signed30(
        r: *mut Scalar,
        a: *const ModInv32Signed30)  {
    
    todo!();
        /*
            const uint32_t a0 = a->v[0], a1 = a->v[1], a2 = a->v[2], a3 = a->v[3], a4 = a->v[4],
                       a5 = a->v[5], a6 = a->v[6], a7 = a->v[7], a8 = a->v[8];

        /* The output from modinv32{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^30). The modulus is < 2^256, so the top limb must be below 2^(256-30*8).
         */
        VERIFY_CHECK(a0 >> 30 == 0);
        VERIFY_CHECK(a1 >> 30 == 0);
        VERIFY_CHECK(a2 >> 30 == 0);
        VERIFY_CHECK(a3 >> 30 == 0);
        VERIFY_CHECK(a4 >> 30 == 0);
        VERIFY_CHECK(a5 >> 30 == 0);
        VERIFY_CHECK(a6 >> 30 == 0);
        VERIFY_CHECK(a7 >> 30 == 0);
        VERIFY_CHECK(a8 >> 16 == 0);

        r->d[0] = a0       | a1 << 30;
        r->d[1] = a1 >>  2 | a2 << 28;
        r->d[2] = a2 >>  4 | a3 << 26;
        r->d[3] = a3 >>  6 | a4 << 24;
        r->d[4] = a4 >>  8 | a5 << 22;
        r->d[5] = a5 >> 10 | a6 << 20;
        r->d[6] = a6 >> 12 | a7 << 18;
        r->d[7] = a7 >> 14 | a8 << 16;

    #ifdef VERIFY
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}

pub fn scalar_to_signed30(
        r: *mut ModInv32Signed30,
        a: *const Scalar)  {
    
    todo!();
        /*
            const uint32_t M30 = UINT32_MAX >> 2;
        const uint32_t a0 = a->d[0], a1 = a->d[1], a2 = a->d[2], a3 = a->d[3],
                       a4 = a->d[4], a5 = a->d[5], a6 = a->d[6], a7 = a->d[7];

    #ifdef VERIFY
        VERIFY_CHECK(scalar_check_overflow(a) == 0);
    #endif

        r->v[0] =  a0                   & M30;
        r->v[1] = (a0 >> 30 | a1 <<  2) & M30;
        r->v[2] = (a1 >> 28 | a2 <<  4) & M30;
        r->v[3] = (a2 >> 26 | a3 <<  6) & M30;
        r->v[4] = (a3 >> 24 | a4 <<  8) & M30;
        r->v[5] = (a4 >> 22 | a5 << 10) & M30;
        r->v[6] = (a5 >> 20 | a6 << 12) & M30;
        r->v[7] = (a6 >> 18 | a7 << 14) & M30;
        r->v[8] =  a7 >> 16;
        */
}

lazy_static!{
    /*
    static const modinv32_modinfo const_modinfo_scalar = {
        {{0x10364141L, 0x3F497A33L, 0x348A03BBL, 0x2BB739ABL, -0x146L, 0, 0, 0, 65536}},
        0x2A774EC1L
    };
    */
}

pub fn scalar_inverse(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            modinv32_signed30 s;
    #ifdef VERIFY
        int zero_in = scalar_is_zero(x);
    #endif
        scalar_to_signed30(&s, x);
        modinv32(&s, &const_modinfo_scalar);
        scalar_from_signed30(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(scalar_is_zero(r) == zero_in);
    #endif
        */
}

pub fn scalar_inverse_var(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            modinv32_signed30 s;
    #ifdef VERIFY
        int zero_in = scalar_is_zero(x);
    #endif
        scalar_to_signed30(&s, x);
        modinv32_var(&s, &const_modinfo_scalar);
        scalar_from_signed30(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(scalar_is_zero(r) == zero_in);
    #endif
        */
}

#[inline] pub fn scalar_is_even(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return !(a->d[0] & 1);
        */
}
