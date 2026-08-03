//! Provides the architecture dependnet abilities associated with paging.

cfg_select! {
    target_arch = "x86_64" => {
        mod x86_64;
        use x86_64 as imp;
    }
    _ => {
        compile_error!("Unsupported architecture");
    }
}

pub use imp::size::*;
