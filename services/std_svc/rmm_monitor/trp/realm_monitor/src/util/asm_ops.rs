#[macro_export]
macro_rules! read_sysreg {
    ($r: ident) => {
        {
            let mut val: usize = 0;
            unsafe{asm!("mrs {0},$r", out(reg) val);}
            val
        }
        
    };
}
