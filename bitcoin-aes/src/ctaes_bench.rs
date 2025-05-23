// ---------------- [ File: bitcoin-aes/src/ctaes_bench.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/bench.c]

pub fn gettimedouble() -> f64 {
    
    let mut tv: libc::timeval = unsafe { std::mem::zeroed() };

    unsafe {
        libc::gettimeofday(&mut tv as *mut libc::timeval, null_mut());
    }

    (tv.tv_usec as f64) * 0.000001_f64 + (tv.tv_sec as f64)
}

pub fn print_number(x: f64)  {
    
    let mut y: f64 = x;

    let mut c: i32 = 0;

    if y < 0.0 {
        y = -y;
    }

    while y < 100.0 {
        y *= 10.0;

        c += 1;
    }

    unsafe {
        libc::printf(
            "%.*f".as_ptr() as *const i8, 
            c, 
            x
        );
    }
}

pub type BenchHook = fn(_0: *mut c_void);

pub const BENCH_HOOK_NOOP: BenchHook = null_hook;

pub fn null_hook(x: *mut c_void) {}

pub fn run_benchmark(
        name:      &str,
        benchmark: Option<BenchHook>,
        setup:     Option<BenchHook>,
        teardown:  Option<BenchHook>,
        data:      *mut c_void,
        count:     i32,
        iter:      i32)  {

    let benchmark = benchmark.unwrap_or(BENCH_HOOK_NOOP);

    let mut i = i32::default();

    let mut min: f64 = f64::MAX;
    let mut sum: f64 = 0.0;
    let mut max: f64 = 0.0;

    for i in 0..count {

        let mut begin: f64 = f64::default();
        let mut total: f64 = f64::default();

        if let Some(setup) = setup {
            setup(data);
        }

        begin = gettimedouble();

        benchmark(data);

        total = gettimedouble() - begin;

        if let Some(teardown) = teardown {
            teardown(data);
        }

        if total < min {
            min = total;
        }

        if total > max {
            max = total;
        }

        sum += total;
    }

    todo!();
        /*
        printf("%s: min ", name);
        print_number(min * 1000000000.0 / iter);
        printf("ns / avg ");
        print_number((sum / count) * 1000000000.0 / iter);
        printf("ns / max ");
        print_number(max * 1000000000.0 / iter);
        printf("ns\n");
        */
}

pub fn bench_aes128_init(data: *mut c_void)  {
    
    todo!();
        /*
            AES128_ctx* ctx = (AES128_ctx*)data;
        int i;
        for (i = 0; i < 50000; i++) {
            AES128_init(ctx, (unsigned char*)ctx);
        }
        */
}

pub fn bench_aes128_encrypt_setup(data: *mut c_void)  {
    
    todo!();
        /*
            AES128_ctx* ctx = (AES128_ctx*)data;
        static const unsigned char key[16] = {0};
        AES128_init(ctx, key);
        */
}

pub fn bench_aes128_encrypt(data: *mut c_void)  {
    
    todo!();
        /*
            const AES128_ctx* ctx = (const AES128_ctx*)data;
        unsigned char scratch[16] = {0};
        int i;
        for (i = 0; i < 4000000 / 16; i++) {
            AES128_encrypt(ctx, 1, scratch, scratch);
        }
        */
}

pub fn bench_aes128_decrypt(data: *mut c_void)  {
    
    todo!();
        /*
            const AES128_ctx* ctx = (const AES128_ctx*)data;
        unsigned char scratch[16] = {0};
        int i;
        for (i = 0; i < 4000000 / 16; i++) {
            AES128_decrypt(ctx, 1, scratch, scratch);
        }
        */
}

pub fn bench_aes192_init(data: *mut c_void)  {
    
    todo!();
        /*
            AES192_ctx* ctx = (AES192_ctx*)data;
        int i;
        for (i = 0; i < 50000; i++) {
            AES192_init(ctx, (unsigned char*)ctx);
        }
        */
}

pub fn bench_aes192_encrypt_setup(data: *mut c_void)  {
    
    todo!();
        /*
            AES192_ctx* ctx = (AES192_ctx*)data;
        static const unsigned char key[16] = {0};
        AES192_init(ctx, key);
        */
}

pub fn bench_aes192_encrypt(data: *mut c_void)  {
    
    todo!();
        /*
            const AES192_ctx* ctx = (const AES192_ctx*)data;
        unsigned char scratch[16] = {0};
        int i;
        for (i = 0; i < 4000000 / 16; i++) {
            AES192_encrypt(ctx, 1, scratch, scratch);
        }
        */
}

pub fn bench_aes192_decrypt(data: *mut c_void)  {
    
    todo!();
        /*
            const AES192_ctx* ctx = (const AES192_ctx*)data;
        unsigned char scratch[16] = {0};
        int i;
        for (i = 0; i < 4000000 / 16; i++) {
            AES192_decrypt(ctx, 1, scratch, scratch);
        }
        */
}

pub fn bench_aes256_init(data: *mut c_void)  {
    
    todo!();
        /*
            AES256_ctx* ctx = (AES256_ctx*)data;
        int i;
        for (i = 0; i < 50000; i++) {
            AES256_init(ctx, (unsigned char*)ctx);
        }
        */
}

pub fn bench_aes256_encrypt_setup(data: *mut c_void)  {
    
    todo!();
        /*
            AES256_ctx* ctx = (AES256_ctx*)data;
        static const unsigned char key[16] = {0};
        AES256_init(ctx, key);
        */
}

pub fn bench_aes256_encrypt(data: *mut c_void)  {
    
    todo!();
        /*
            const AES256_ctx* ctx = (const AES256_ctx*)data;
        unsigned char scratch[16] = {0};
        int i;
        for (i = 0; i < 4000000 / 16; i++) {
            AES256_encrypt(ctx, 1, scratch, scratch);
        }
        */
}

pub fn bench_aes256_decrypt(data: *mut c_void)  {
    
    todo!();
        /*
            const AES256_ctx* ctx = (const AES256_ctx*)data;
        unsigned char scratch[16] = {0};
        int i;
        for (i = 0; i < 4000000 / 16; i++) {
            AES256_decrypt(ctx, 1, scratch, scratch);
        }
        */
}

//#[bench] 
pub fn crypto_ctaes_bench(b: &mut Bencher) -> i32 {
    
    let mut ctx128 = AES128_ctx::default();
    let mut ctx192 = AES192_ctx::default();
    let mut ctx256 = AES256_ctx::default();

    run_benchmark("aes128_init",         Some(bench_aes128_init),     None,                             None, mut_cvoid![ctx128], 20, 50000);
    run_benchmark("aes128_encrypt_byte", Some(bench_aes128_encrypt),  Some(bench_aes128_encrypt_setup), None, mut_cvoid![ctx128], 20, 4000000);
    run_benchmark("aes128_decrypt_byte", Some(bench_aes128_decrypt),  Some(bench_aes128_encrypt_setup), None, mut_cvoid![ctx128], 20, 4000000);
    run_benchmark("aes192_init",         Some(bench_aes192_init),     None,                             None, mut_cvoid![ctx192], 20, 50000);
    run_benchmark("aes192_encrypt_byte", Some(bench_aes192_encrypt),  Some(bench_aes192_encrypt_setup), None, mut_cvoid![ctx192], 20, 4000000);
    run_benchmark("aes192_decrypt_byte", Some(bench_aes192_decrypt),  Some(bench_aes192_encrypt_setup), None, mut_cvoid![ctx192], 20, 4000000);
    run_benchmark("aes256_init",         Some(bench_aes256_init),     None,                             None, mut_cvoid![ctx256], 20, 50000);
    run_benchmark("aes256_encrypt_byte", Some(bench_aes256_encrypt),  Some(bench_aes256_encrypt_setup), None, mut_cvoid![ctx256], 20, 4000000);
    run_benchmark("aes256_decrypt_byte", Some(bench_aes256_decrypt),  Some(bench_aes256_encrypt_setup), None, mut_cvoid![ctx256], 20, 4000000);

    0
}
