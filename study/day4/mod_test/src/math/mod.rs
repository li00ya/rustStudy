pub mod add;
pub mod sub;

//重导出函数，使其公开
//pub use add::add;
//pub use sub::sub;
pub use self::{add::add, sub::sub};

const DEBUG: bool = true;
