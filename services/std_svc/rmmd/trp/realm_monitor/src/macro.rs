#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let buffer = alloc::format!($($arg)*);
        let _ = $crate::io::stdout().write_all(buffer.as_bytes());
    };
}

// #[cfg(debug_assertions)]
#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($fmt:expr) => {$crate::print!(concat!($fmt, "\n"))};
    ($fmt:expr, $($arg:tt)*) => {$crate::print!(concat!($fmt, "\n"), $($arg)*)};
}

// #[cfg(not(debug_assertions))]
// #[macro_export]
// macro_rules! println {
//     () => {};
//     ($fmt:expr) => {};
//     ($fmt:expr, $($arg:tt)*) => {};
// }


#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dprintln {
    () => {$crate::print!("\n")};
    ($fmt:expr) => {$crate::print!(concat!($fmt, "\n"))};
    ($fmt:expr, $($arg:tt)*) => {$crate::print!(concat!($fmt, "\n"), $($arg)*)};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dprintln {
    () => {};
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}

#[macro_export]
macro_rules! eprint {
    ($fmt:expr) => {
        let buffer = concat!("\x1b[0;31m", $fmt, "\x1b[0m");
        let _ = $crate::io::stdout().write_all(buffer.as_bytes());
    };
    ($fmt:expr, $($arg:tt)*) => {{
        let buffer = alloc::format!(concat!("\x1b[0;31m", $fmt, "\x1b[0m"), $($arg)*);
        let _ = $crate::io::stdout().write_all(buffer.as_bytes());
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {$crate::eprint!("\n")};
    ($fmt:expr) => {$crate::eprint!(concat!($fmt, "\n"))};
    ($fmt:expr, $($arg:tt)*) => {$crate::eprint!(concat!($fmt, "\n"), $($arg)*)};
}

#[macro_export]
macro_rules! offset_of {
    ($Struct:path, $field:ident) => {{
        fn offset() -> usize {
            let u = core::mem::MaybeUninit::<$Struct>::uninit();
            let &$Struct { $field: ref f, .. } = unsafe { &*u.as_ptr() };
            let o = (f as *const _ as usize).wrapping_sub(&u as *const _ as usize);
            assert!((0..=core::mem::size_of_val(&u)).contains(&o));
            o
        }
        offset()
    }};
}

#[macro_export]
macro_rules! const_assert {
    ($cond:expr) => {
        // Causes overflow if condition is false
        let _ = [(); 0 - (!($cond) as usize)];
    };
}

#[macro_export]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr) => {
        const _: () = {
            $crate::const_assert!($left == $right);
            ()
        };
    };
}

#[macro_export]
macro_rules! const_assert_size {
    ($struct:ty, $size:expr) => {
        $crate::const_assert_eq!(core::mem::size_of::<$struct>(), ($size));
    };
}

#[macro_export]
macro_rules! rmm_print {
    ($($arg:tt)*) => ($crate::_print(core::format_args!($($arg)*)));
}

#[macro_export]
macro_rules! rmm_println {
    () => ($crate::rmm_print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*));
        $crate::rmm_print!("\r\n");
    }
}

#[cfg(test)]
mod test {
    use crate::io::test::MockDevice;
    use crate::io::{stdout, Write as IoWrite};
    use crate::{eprintln, println};
    use alloc::boxed::Box;

    extern crate alloc;

    #[test]
    fn println_without_arg() {
        let mock = Box::new(MockDevice::new());
        let mock_ptr = mock.as_ref() as *const MockDevice;
        stdout().attach(mock).ok().unwrap();

        println!();

        assert_eq!(unsafe { (*mock_ptr).output() }, "\n");
    }

    #[test]
    fn println_without_format() {
        let mock = Box::new(MockDevice::new());
        let mock_ptr = mock.as_ref() as *const MockDevice;
        stdout().attach(mock).ok().unwrap();

        println!("hello");
        assert_eq!(unsafe { (*mock_ptr).output() }, "hello\n");
    }

    #[test]
    fn println_with_format() {
        let mock = Box::new(MockDevice::new());
        let mock_ptr = mock.as_ref() as *const MockDevice;
        stdout().attach(mock).ok().unwrap();

        println!("number {}", 1234);
        assert_eq!(unsafe { (*mock_ptr).output() }, "number 1234\n");
    }

    #[test]
    fn eprintln_without_arg() {
        let mock = Box::new(MockDevice::new());
        let mock_ptr = mock.as_ref() as *const MockDevice;
        stdout().attach(mock).ok().unwrap();

        eprintln!();
        assert_eq!(unsafe { (*mock_ptr).output() }, "\x1b[0;31m\n\x1b[0m");
    }

    #[test]
    fn eprintln_without_format() {
        let mock = Box::new(MockDevice::new());
        let mock_ptr = mock.as_ref() as *const MockDevice;
        stdout().attach(mock).ok().unwrap();

        eprintln!("hello");
        assert_eq!(unsafe { (*mock_ptr).output() }, "\x1b[0;31mhello\n\x1b[0m");
    }

    #[test]
    fn eprintln_with_format() {
        let mock = Box::new(MockDevice::new());
        let mock_ptr = mock.as_ref() as *const MockDevice;
        stdout().attach(mock).ok().unwrap();

        eprintln!("number {}", 4321);
        assert_eq!(
            unsafe { (*mock_ptr).output() },
            "\x1b[0;31mnumber 4321\n\x1b[0m"
        );
    }

    #[test]
    fn offset_of() {
        struct Context {
            gp_regs: [u64; 31],
            elr: u64,
            spsr: u64,
            sys_regs: SystemRegister,
        }
        struct SystemRegister {
            pub ttbr0: u64,
            pub tpidr: u64,
        }

        assert_eq!(offset_of!(Context, gp_regs), 0);
        assert_eq!(offset_of!(Context, elr), 248);
        assert_eq!(offset_of!(Context, spsr), 256);
        assert_eq!(offset_of!(Context, sys_regs), 264);
    }

    #[test]
    fn set_of_const_assert() {
        const_assert!(1 != 2);
        const_assert!(true);

        const_assert_eq!(1, 1);
        const_assert_eq!(false, false);

        const_assert_size!(u32, 4);
        const_assert_size!(u64, 8);
    }
}
