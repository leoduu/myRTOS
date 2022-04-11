
#[macro_export]
macro_rules! align_down_16_bit {
    ($val:expr) => {
        ($val & & 0xFFFF_FFF0)
    };
} 

#[macro_export]
macro_rules! align_up_16_bit {
    ($val:expr) => {
        if ($val & 0x0F) == 0 {
            $val
        } else {
            (($val & 0xFFFF_FFF0) + 0x10)
        }
    };
} 

#[macro_export]
macro_rules! align_down_8_bit {
    ($val:expr) => {
        ($val & & 0xFFFF_FFF8)
    };
} 

#[macro_export]
macro_rules! align_up_8_bit {
    ($val:expr) => {
        if ($val & 0x07) == 0 {
            $val
        } else {
            (($val & 0xFFFF_FFF8) + 0x07)
        }
    };
} 
