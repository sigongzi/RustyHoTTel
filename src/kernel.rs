pub mod term;
pub mod hottype;

pub use term::HoTTerm;
pub use hottype::HoTType;

pub use hottype::constructor::*;
pub use term::constructor::*;
pub use hottype::check_type;
pub use hottype::operator::*;