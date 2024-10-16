
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($x:tt)*) => { 
    {
        extern crate std;    
        std::println!($($x)*) 
    }
    };
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($x:tt)*) => {
    };
}