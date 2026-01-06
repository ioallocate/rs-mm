pub mod ntapi;
pub mod module;
pub mod handle;
pub mod error;
pub mod memory;
pub mod mouse;

pub use error::{mm_error, Result};
pub use handle::{p_handle};
pub use memory::{mmg};
pub use module::ModuleInfo;
pub use mouse::Mouse;
