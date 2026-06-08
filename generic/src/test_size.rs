#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    macro_rules! print_size_of {
        ($name:ty) => {
            std::println!("{} is {} bytes", stringify!($name), mem::size_of::<$name>());
        };
    }

    extern crate std;
    use core::mem;

    #[test]
    fn size_printout() {
        print_size_of!(crate::network::ConnectionInfo);
        print_size_of!(crate::network::IpAddress);
        print_size_of!(crate::network::SubscriberInfo);
        print_size_of!(crate::timecode::Timecode);
        print_size_of!(crate::str::StaticString<8>);
        print_size_of!(crate::typeflags::MessageType);
    }
}
