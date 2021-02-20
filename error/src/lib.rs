mod error_functions;
mod error_text;

pub use error_functions::error_msg;
pub use error_functions::error_exit;
pub use error_functions::error_exit2;
pub use error_functions::error_exit_en;
pub use error_functions::fatal;
pub use error_functions::usage_error;
pub use error_functions::cmdline_error;

use error_text::error_text_rs as get_error_text;
