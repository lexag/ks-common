use crate::str::StaticString;

/// Format a unix timestamp (or similar) into hh:mm:ss
pub fn format_hms(time_t: u64) -> StaticString<8> {
    let day_time = time_t % 86400;
    let mut s = StaticString::empty();
    s.set_char(0, (day_time / 3600 / 10) as u8 + 0x30);
    s.set_char(1, (day_time / 3600 % 10) as u8 + 0x30);
    s.set_char(2, b':');
    s.set_char(3, (day_time / 60 % 60 / 10) as u8 + 0x30);
    s.set_char(4, (day_time / 60 % 60 % 10) as u8 + 0x30);
    s.set_char(5, b':');
    s.set_char(6, (day_time % 60 / 10) as u8 + 0x30);
    s.set_char(7, (day_time % 60 % 10) as u8 + 0x30);
    s
}

/// Format a unix timestamp (or similar) into hh:mm
pub fn format_hm(time_t: u64) -> StaticString<8> {
    let day_time = time_t % 86400;
    let mut s = StaticString::empty();
    s.set_char(0, (day_time / 3600 / 10) as u8 + 0x30);
    s.set_char(1, (day_time / 3600 % 10) as u8 + 0x30);
    s.set_char(2, b':');
    s.set_char(3, (day_time / 60 % 60 / 10) as u8 + 0x30);
    s.set_char(4, (day_time / 60 % 60 % 10) as u8 + 0x30);
    s
}
