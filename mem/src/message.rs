bitflags::bitflags! {
    /// Bitflag for one or multiple types of core -> client message
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
    pub struct MessageType: u16 {
        /// Transport changed
        const TransportData = 0x01;
        /// Transport changed
        const TimecodeData = 0x800;
        /// Playback data changed
        const PlaybackData = 0x2000;
        /// Beat changed
        const BeatData = 0x02;
        /// Cue changed (lightweight)
        const SmallCueData = 0x400;
        /// Cue changed
        const CueData = 0x04;
        /// Show changed
        const ShowData = 0x08;
        /// Network changed
        const NetworkChanged = 0x10;
        /// JACKState changed
        const JACKStateChanged = 0x20;
        /// Configuration changed
        const ConfigurationChanged = 0x40;
        /// Shutdown occured
        const ShutdownOccured = 0x80;
        /// Heartbeat
        const Heartbeat = 0x100;
        /// Event occured
        const EventOccured = 0x200;
        /// Log occured
        const Log = 0x1000;
    }
}

//impl bincode::Encode for MessageType {
//    fn encode<E: bincode::enc::Encoder>(
//        &self,
//        encoder: &mut E,
//    ) -> Result<(), bincode::error::EncodeError> {
//        self.bits().encode(encoder)
//    }
//}
//
//impl<'de, Context> bincode::BorrowDecode<'de, Context> for MessageType {
//    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
//        decoder: &mut D,
//    ) -> core::result::Result<Self, bincode::error::DecodeError> {
//        Ok(Self::from_bits_retain(
//            bincode::BorrowDecode::borrow_decode(decoder)?,
//        ))
//    }
//}
//impl<Context> bincode::Decode<Context> for MessageType {
//    fn decode<D: bincode::de::Decoder<Context = Context>>(
//        decoder: &mut D,
//    ) -> core::result::Result<Self, bincode::error::DecodeError> {
//        Ok(Self::from_bits_retain(bincode::Decode::decode(decoder)?))
//    }
//}
