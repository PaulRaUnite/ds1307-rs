extern crate ds1307;
extern crate embedded_hal_mock as hal;
use self::ds1307::Error;
use self::hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, Register, ADDR};

get_test!(get, get_month, 12, trans_read!(MONTH, [0b0001_0010]));

set_invalid_test!(too_small, set_month, 0);
set_invalid_test!(too_big, set_month, 13);

set_test!(set, set_month, 12, trans_write!(MONTH, [0b0001_0010]));
