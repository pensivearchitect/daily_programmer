extern "platform-intrinsic" {
    fn x86_rdrand64_step() -> (u64, i32);
}

/// convenience struct for using the intel hardware TRNG
pub struct Rng;

impl Rng {
    /// gets an arguably truly random u64 from intels (D|T)RNG generator
    ///
    /// Pulling from rdrand too often can compromise the systems cryptographic integrity (ie
    /// compromising SSL's ability to provide perfect forward secrecy), but unless you use this in
    /// a script you shouldn't worry about it.
    ///
    /// How random this is truly is a matter of debate, as of the time of this writing rdrand has
    /// not been audited nor is there a good way of auditing hardware in general at this time.
    pub fn next(&self) -> u64 {
        loop {
            unsafe {
                let (n, cond) = x86_rdrand64_step();
                if cond != 0 { return n }
            }
        }
    }

    /// convenience function that makes it easier to work with ranges
    pub fn range<T: Into<u8>>(&self, limit: T) -> u64 {
        let seed = Rng.next() as f64 / ((::std::u64::MAX) as f64 + 1.0);
        (seed * limit.into() as f64) as u64 + 1
    }

    /// we declare this as unsafe because processor features cannot be safely checked from rust
    /// that is to say, the stdlib impl uses the asm! macro
    pub unsafe fn feature_check() -> bool {
        use std::arch::x86_64::{has_cpuid, __cpuid as cpuid};
        // assume that if cpuid isn't supported, rdrand isn't either
        if !has_cpuid() || cpuid(1).ecx & (1 << 30) == 0 {
            eprintln!("exiting because your processor does not support the rdrand instruction");
            ::std::process::exit(1);
        }
        true
    }
}
