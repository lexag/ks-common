#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    macro_rules! print_size_of {
        ($name:ty) => {
            std::println!("{} is {} bytes", stringify!($name), mem::size_of::<$name>());
        };
    }

    extern crate std;
    use std::mem;

    #[test]
    fn size_printout() {
        print_size_of!(crate::beat::Beat);
        print_size_of!(crate::cue::Cue);
        print_size_of!(crate::cue::CueMetadata);
        print_size_of!(crate::cue::CueSkeleton);
        print_size_of!(crate::cue::Show);
        print_size_of!(crate::cue::ShowMetadata);
        print_size_of!(crate::cue::ShowSkeleton);
        print_size_of!(crate::event::Event);
        print_size_of!(crate::event::EventDescription);
        print_size_of!(crate::event::EventTable);
        print_size_of!(crate::local::config::AudioConfiguration);
        print_size_of!(crate::local::config::ChannelConfiguration);
        print_size_of!(crate::local::config::JACKClientConfiguration);
        print_size_of!(crate::local::config::JACKServerConfiguration);
        print_size_of!(crate::local::config::LogContext);
        print_size_of!(crate::local::config::LoggerConfiguration);
        print_size_of!(crate::local::config::LogKind);
        print_size_of!(crate::local::config::SystemConfiguration);
        print_size_of!(crate::local::status::AudioDevice);
        print_size_of!(crate::local::status::AudioSourceState);
        print_size_of!(crate::local::status::BeatState);
        print_size_of!(crate::local::status::CombinedStatus);
        print_size_of!(crate::local::status::CueState);
        print_size_of!(crate::local::status::JACKStatus);
        print_size_of!(crate::local::status::NetworkStatus);
        print_size_of!(crate::local::status::PlaybackState);
        print_size_of!(crate::local::status::TransportState);
        print_size_of!(crate::mem::network::ConnectionInfo);
        print_size_of!(crate::mem::network::IpAddress);
        print_size_of!(crate::mem::network::SubscriberInfo);
        print_size_of!(crate::mem::smpte::TimecodeInstant);
        print_size_of!(crate::mem::str::StaticString<8>);
        print_size_of!(crate::mem::typeflags::MessageType);
        print_size_of!(crate::protocol::message::Heartbeat);
        print_size_of!(crate::protocol::message::LargeMessage);
        print_size_of!(crate::protocol::message::SmallMessage);
    }
}
