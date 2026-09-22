# ! [allow (warnings)] use vest_lib::combinators::mapped::spec::* ;
use vest_lib::combinators::* ;
use vest_lib::combinators::recursive::* ;
use Sum::Inl as L ;
use Sum::Inr as R ;
use vest_lib::Never ;
use vest_lib::core::exec::input::{
    InputBuf,
    InputSlice
}
;
use vest_lib::core::exec::output::OutputBuf ;
use vest_lib::core::exec::parser::* ;
use vest_lib::core::exec::serializer::* ;
use vest_lib::core::exec::ParseError ;
use vest_lib::core::exec::bytes_eq ;
use vest_lib::core::{
    proof::*,
    spec::*
}
;
use vest_lib::primitives::btcvarint::VarInt ;
use vest_lib::primitives::leb128::ULeb128 ;
use vstd::prelude::* ;
verus! {
// ============================================================
// Data Types
// ============================================================
# [doc = "data type for `mavlink_msg`."]
# [derive (Debug, PartialEq, Eq, Clone, Copy)]
pub struct MavlinkMsg<'i> {
    pub magic: ProtocolMagic,
    pub msg: MavlinkMsgMsg<'i>,
}
# [verifier::ext_equal]
pub struct MavlinkMsgSpec < T0 = ProtocolMagicSpec, T1 = MavlinkMsgMsgSpec > {
    pub magic: T0,
    pub msg: T1,
}
pub type MavlinkMsgInner = (ProtocolMagicSpec, MavlinkMsgMsgSpec) ;
impl<'i> DeepView for MavlinkMsg<'i> {
    type V = MavlinkMsgSpec ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        MavlinkMsgSpec {
            magic: self.magic.deep_view(),
            msg: self.msg.deep_view(),
        }
    }
}
impl<'i> MavlinkMsg<'i> {
    pub proof fn lemma_deep_view_fields (& self) ensures self.deep_view().magic == self.magic.deep_view(),
    self.deep_view().msg == self.msg.deep_view(),
    {
        reveal(< MavlinkMsg as DeepView>::deep_view) ;
    }
}
impl < T0, T1 > MavlinkMsgSpec < T0, T1 > {
    # [verifier::opaque] pub open spec fn from_structural (input: (T0,
    T1)) -> Self {
        let (magic,
        msg) = input ;
        Self {
            magic,
            msg
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> (T0,
    T1) {
        let Self {
            magic,
            msg
        }
        = self ;
        (magic,
        msg)
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MavlinkMsgSpec::from_structural) ;
        reveal(MavlinkMsgSpec::into_structural) ;
    }
    pub broadcast proof fn lemma_into_from (input: (T0,
    T1)) ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MavlinkMsgSpec::from_structural) ;
        reveal(MavlinkMsgSpec::into_structural) ;
    }
    pub proof fn lemma_into_structural_fields (self) ensures Self::into_structural (self) == match self {
        Self {
            magic,
            msg
        }
        => (magic,
        msg),
    }
   ,
    {
        reveal(MavlinkMsgSpec::into_structural) ;
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkMsgForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkMsgReverse ;
impl SpecMap for MavlinkMsgForward {
    type Input = MavlinkMsgInner ;
    type Output = MavlinkMsgSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MavlinkMsgSpec::from_structural (input)
    }
}
impl SpecMap for MavlinkMsgReverse {
    type Input = MavlinkMsgSpec ;
    type Output = MavlinkMsgInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}

# [doc = "data type for `mavlink_v2_msg`."]
# [derive (Debug, PartialEq, Eq, Clone, Copy)]
pub struct MavlinkV2Msg<'i> {
    pub len: u8,
    pub incompat_flags: IncompatFlags,
    pub compat_flags: u8,
    pub seq: u8,
    pub sysid: u8,
    pub compid: u8,
    pub msgid: MessageIdsV2,
    pub payload: &'i [u8],
    pub checksum: u16,
    pub signature: MavlinkV2MsgSignature<'i>,
}
# [verifier::ext_equal]
pub struct MavlinkV2MsgSpec < T0 = u8, T1 = IncompatFlagsSpec, T2 = u8, T3 = u8, T4 = u8, T5 = u8, T6 = MessageIdsV2Spec, T7 = Seq < u8 >, T8 = u16, T9 = MavlinkV2MsgSignatureSpec > {
    pub len: T0,
    pub incompat_flags: T1,
    pub compat_flags: T2,
    pub seq: T3,
    pub sysid: T4,
    pub compid: T5,
    pub msgid: T6,
    pub payload: T7,
    pub checksum: T8,
    pub signature: T9,
}
pub type MavlinkV2MsgInner = (u8, (IncompatFlagsSpec, (u8, (u8, (u8, (u8, (MessageIdsV2Spec, (Seq < u8 >, (u16, MavlinkV2MsgSignatureSpec))))))))) ;
impl<'i> DeepView for MavlinkV2Msg<'i> {
    type V = MavlinkV2MsgSpec ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        MavlinkV2MsgSpec {
            len: self.len.deep_view(),
            incompat_flags: self.incompat_flags.deep_view(),
            compat_flags: self.compat_flags.deep_view(),
            seq: self.seq.deep_view(),
            sysid: self.sysid.deep_view(),
            compid: self.compid.deep_view(),
            msgid: self.msgid.deep_view(),
            payload: self.payload.deep_view(),
            checksum: self.checksum.deep_view(),
            signature: self.signature.deep_view(),
        }
    }
}
impl<'i> MavlinkV2Msg<'i> {
    pub proof fn lemma_deep_view_fields (& self) ensures self.deep_view().len == self.len.deep_view(),
    self.deep_view().incompat_flags == self.incompat_flags.deep_view(),
    self.deep_view().compat_flags == self.compat_flags.deep_view(),
    self.deep_view().seq == self.seq.deep_view(),
    self.deep_view().sysid == self.sysid.deep_view(),
    self.deep_view().compid == self.compid.deep_view(),
    self.deep_view().msgid == self.msgid.deep_view(),
    self.deep_view().payload == self.payload.deep_view(),
    self.deep_view().checksum == self.checksum.deep_view(),
    self.deep_view().signature == self.signature.deep_view(),
    {
        reveal(< MavlinkV2Msg as DeepView>::deep_view) ;
    }
}
impl < T0, T1, T2, T3, T4, T5, T6, T7, T8, T9 > MavlinkV2MsgSpec < T0, T1, T2, T3, T4, T5, T6, T7, T8, T9 > {
    # [verifier::opaque] pub open spec fn from_structural (input: (T0,
    (T1,
    (T2,
    (T3,
    (T4,
    (T5,
    (T6,
    (T7,
    (T8,
    T9)))))))))) -> Self {
        let (len,
        (incompat_flags,
        (compat_flags,
        (seq,
        (sysid,
        (compid,
        (msgid,
        (payload,
        (checksum,
        signature))))))))) = input ;
        Self {
            len,
            incompat_flags,
            compat_flags,
            seq,
            sysid,
            compid,
            msgid,
            payload,
            checksum,
            signature
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> (T0,
    (T1,
    (T2,
    (T3,
    (T4,
    (T5,
    (T6,
    (T7,
    (T8,
    T9))))))))) {
        let Self {
            len,
            incompat_flags,
            compat_flags,
            seq,
            sysid,
            compid,
            msgid,
            payload,
            checksum,
            signature
        }
        = self ;
        (len,
        (incompat_flags,
        (compat_flags,
        (seq,
        (sysid,
        (compid,
        (msgid,
        (payload,
        (checksum,
        signature)))))))))
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MavlinkV2MsgSpec::from_structural) ;
        reveal(MavlinkV2MsgSpec::into_structural) ;
    }
    pub broadcast proof fn lemma_into_from (input: (T0,
    (T1,
    (T2,
    (T3,
    (T4,
    (T5,
    (T6,
    (T7,
    (T8,
    T9)))))))))) ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MavlinkV2MsgSpec::from_structural) ;
        reveal(MavlinkV2MsgSpec::into_structural) ;
    }
    pub proof fn lemma_into_structural_fields (self) ensures Self::into_structural (self) == match self {
        Self {
            len,
            incompat_flags,
            compat_flags,
            seq,
            sysid,
            compid,
            msgid,
            payload,
            checksum,
            signature
        }
        => (len,
        (incompat_flags,
        (compat_flags,
        (seq,
        (sysid,
        (compid,
        (msgid,
        (payload,
        (checksum,
        signature))))))))),
    }
   ,
    {
        reveal(MavlinkV2MsgSpec::into_structural) ;
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkV2MsgForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkV2MsgReverse ;
impl SpecMap for MavlinkV2MsgForward {
    type Input = MavlinkV2MsgInner ;
    type Output = MavlinkV2MsgSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MavlinkV2MsgSpec::from_structural (input)
    }
}
impl SpecMap for MavlinkV2MsgReverse {
    type Input = MavlinkV2MsgSpec ;
    type Output = MavlinkV2MsgInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}

# [doc = "data type for `mavlink_v1_msg`."]
# [derive (Debug, PartialEq, Eq, Clone, Copy)]
pub struct MavlinkV1Msg<'i> {
    pub len: u8,
    pub seq: u8,
    pub sysid: u8,
    pub compid: u8,
    pub msgid: MessageIdsV1,
    pub payload: &'i [u8],
    pub checksum: u16,
}
# [verifier::ext_equal]
pub struct MavlinkV1MsgSpec < T0 = u8, T1 = u8, T2 = u8, T3 = u8, T4 = MessageIdsV1Spec, T5 = Seq < u8 >, T6 = u16 > {
    pub len: T0,
    pub seq: T1,
    pub sysid: T2,
    pub compid: T3,
    pub msgid: T4,
    pub payload: T5,
    pub checksum: T6,
}
pub type MavlinkV1MsgInner = (u8, (u8, (u8, (u8, (MessageIdsV1Spec, (Seq < u8 >, u16)))))) ;
impl<'i> DeepView for MavlinkV1Msg<'i> {
    type V = MavlinkV1MsgSpec ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        MavlinkV1MsgSpec {
            len: self.len.deep_view(),
            seq: self.seq.deep_view(),
            sysid: self.sysid.deep_view(),
            compid: self.compid.deep_view(),
            msgid: self.msgid.deep_view(),
            payload: self.payload.deep_view(),
            checksum: self.checksum.deep_view(),
        }
    }
}
impl<'i> MavlinkV1Msg<'i> {
    pub proof fn lemma_deep_view_fields (& self) ensures self.deep_view().len == self.len.deep_view(),
    self.deep_view().seq == self.seq.deep_view(),
    self.deep_view().sysid == self.sysid.deep_view(),
    self.deep_view().compid == self.compid.deep_view(),
    self.deep_view().msgid == self.msgid.deep_view(),
    self.deep_view().payload == self.payload.deep_view(),
    self.deep_view().checksum == self.checksum.deep_view(),
    {
        reveal(< MavlinkV1Msg as DeepView>::deep_view) ;
    }
}
impl < T0, T1, T2, T3, T4, T5, T6 > MavlinkV1MsgSpec < T0, T1, T2, T3, T4, T5, T6 > {
    # [verifier::opaque] pub open spec fn from_structural (input: (T0,
    (T1,
    (T2,
    (T3,
    (T4,
    (T5,
    T6))))))) -> Self {
        let (len,
        (seq,
        (sysid,
        (compid,
        (msgid,
        (payload,
        checksum)))))) = input ;
        Self {
            len,
            seq,
            sysid,
            compid,
            msgid,
            payload,
            checksum
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> (T0,
    (T1,
    (T2,
    (T3,
    (T4,
    (T5,
    T6)))))) {
        let Self {
            len,
            seq,
            sysid,
            compid,
            msgid,
            payload,
            checksum
        }
        = self ;
        (len,
        (seq,
        (sysid,
        (compid,
        (msgid,
        (payload,
        checksum))))))
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MavlinkV1MsgSpec::from_structural) ;
        reveal(MavlinkV1MsgSpec::into_structural) ;
    }
    pub broadcast proof fn lemma_into_from (input: (T0,
    (T1,
    (T2,
    (T3,
    (T4,
    (T5,
    T6))))))) ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MavlinkV1MsgSpec::from_structural) ;
        reveal(MavlinkV1MsgSpec::into_structural) ;
    }
    pub proof fn lemma_into_structural_fields (self) ensures Self::into_structural (self) == match self {
        Self {
            len,
            seq,
            sysid,
            compid,
            msgid,
            payload,
            checksum
        }
        => (len,
        (seq,
        (sysid,
        (compid,
        (msgid,
        (payload,
        checksum)))))),
    }
   ,
    {
        reveal(MavlinkV1MsgSpec::into_structural) ;
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkV1MsgForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkV1MsgReverse ;
impl SpecMap for MavlinkV1MsgForward {
    type Input = MavlinkV1MsgInner ;
    type Output = MavlinkV1MsgSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MavlinkV1MsgSpec::from_structural (input)
    }
}
impl SpecMap for MavlinkV1MsgReverse {
    type Input = MavlinkV1MsgSpec ;
    type Output = MavlinkV1MsgInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}

# [doc = "data type for `protocol_magic`."]
# [repr (u8)]
# [derive (Debug, PartialEq, Eq, Clone, Copy, StructuralEq)]
pub enum ProtocolMagic {
    MavLink1 = 254,
    MavLink2 = 253,
}
pub type ProtocolMagicSpec = ProtocolMagic ;
pub type ProtocolMagicInner = u8 ;
impl DeepView for ProtocolMagic {
    type V = Self ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        * self
    }
}
impl ProtocolMagic {
    pub proof fn lemma_deep_view (& self) ensures self.deep_view() == * self,
    {
        reveal(< ProtocolMagic as DeepView>::deep_view) ;
    }
    pub open spec fn structural_valid (input: ProtocolMagicInner) -> bool {
        {
            let x = input ;
            x == 254 || x == 253
        }
    }
    # [verifier::opaque] pub open spec fn from_structural (input: ProtocolMagicInner) -> Self {
        match input {
            254 => Self::MavLink1,
            253 => Self::MavLink2,
            _ => arbitrary(),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> ProtocolMagicInner {
        match self {
            Self::MavLink1 => 254,
            Self::MavLink2 => 253,
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(ProtocolMagic::from_structural) ;
        reveal(ProtocolMagic::into_structural) ;
        match self {
            Self::MavLink1 => {
            }
           ,
            Self::MavLink2 => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: ProtocolMagicInner) requires Self::structural_valid (input),
    ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(ProtocolMagic::from_structural) ;
        reveal(ProtocolMagic::into_structural) ;
        match input {
            254 => {
            }
           ,
            253 => {
            }
           ,
            _ => {
                assert (false) ;
            }
        }
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct ProtocolMagicForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct ProtocolMagicReverse ;
impl SpecMap for ProtocolMagicForward {
    type Input = ProtocolMagicInner ;
    type Output = ProtocolMagicSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        ProtocolMagic::from_structural (input)
    }
}
impl SpecMap for ProtocolMagicReverse {
    type Input = ProtocolMagicSpec ;
    type Output = ProtocolMagicInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}
# [cfg (not (verus_keep_ghost))] unsafe impl Structural for ProtocolMagic {
}

# [doc = "data type for `incompat_flags`."]
# [repr (u8)]
# [derive (Debug, PartialEq, Eq, Clone, Copy, StructuralEq)]
pub enum IncompatFlags {
    Signed = 1,
    Unknown (u8),
}
pub type IncompatFlagsSpec = IncompatFlags ;
pub type IncompatFlagsInner = Sum < u8, u8 > ;
impl DeepView for IncompatFlags {
    type V = Self ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        * self
    }
}
impl IncompatFlags {
    pub proof fn lemma_deep_view (& self) ensures self.deep_view() == * self,
    {
        reveal(< IncompatFlags as DeepView>::deep_view) ;
    }
    pub open spec fn structural_valid (input: IncompatFlagsInner) -> bool {
        match input {
            L (x) => x == 1,
            R (x) => true,
        }
    }
    # [verifier::opaque] pub open spec fn from_structural (input: IncompatFlagsInner) -> Self {
        match input {
            L (x) => match x {
                1 => Self::Signed,
                _ => arbitrary(),
            }
           ,
            R (x) => Self::Unknown (x),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> IncompatFlagsInner {
        match self {
            Self::Signed => L (1),
            Self::Unknown (x) => R (x),
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(IncompatFlags::from_structural) ;
        reveal(IncompatFlags::into_structural) ;
        match self {
            Self::Signed => {
            }
           ,
            Self::Unknown (_) => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: IncompatFlagsInner) requires Self::structural_valid (input),
    ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(IncompatFlags::from_structural) ;
        reveal(IncompatFlags::into_structural) ;
        match input {
            L (x) => match x {
                1 => {
                }
               ,
                _ => {
                    assert (false) ;
                }
            }
           ,
            R (_) => {
            }
           ,
        }
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct IncompatFlagsForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct IncompatFlagsReverse ;
impl SpecMap for IncompatFlagsForward {
    type Input = IncompatFlagsInner ;
    type Output = IncompatFlagsSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        IncompatFlags::from_structural (input)
    }
}
impl SpecMap for IncompatFlagsReverse {
    type Input = IncompatFlagsSpec ;
    type Output = IncompatFlagsInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}
# [cfg (not (verus_keep_ghost))] unsafe impl Structural for IncompatFlags {
}

# [doc = "data type for `message_ids_v2`."]
# [repr (u32)]
# [derive (Debug, PartialEq, Eq, Clone, Copy, StructuralEq)]
pub enum MessageIdsV2 {
    CommandInt = 75,
    CommandLong = 76,
    CommandAck = 77,
    Reserved = 8388608,
    Unknown (u32),
}
pub type MessageIdsV2Spec = MessageIdsV2 ;
pub type MessageIdsV2Inner = Sum < u32, u32 > ;
impl DeepView for MessageIdsV2 {
    type V = Self ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        * self
    }
}
impl MessageIdsV2 {
    pub proof fn lemma_deep_view (& self) ensures self.deep_view() == * self,
    {
        reveal(< MessageIdsV2 as DeepView>::deep_view) ;
    }
    pub open spec fn structural_valid (input: MessageIdsV2Inner) -> bool {
        match input {
            L (x) => x == 75 || x == 76 || x == 77 || x == 8388608,
            R (x) => true,
        }
    }
    # [verifier::opaque] pub open spec fn from_structural (input: MessageIdsV2Inner) -> Self {
        match input {
            L (x) => match x {
                75 => Self::CommandInt,
                76 => Self::CommandLong,
                77 => Self::CommandAck,
                8388608 => Self::Reserved,
                _ => arbitrary(),
            }
           ,
            R (x) => Self::Unknown (x),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> MessageIdsV2Inner {
        match self {
            Self::CommandInt => L (75),
            Self::CommandLong => L (76),
            Self::CommandAck => L (77),
            Self::Reserved => L (8388608),
            Self::Unknown (x) => R (x),
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MessageIdsV2::from_structural) ;
        reveal(MessageIdsV2::into_structural) ;
        match self {
            Self::CommandInt => {
            }
           ,
            Self::CommandLong => {
            }
           ,
            Self::CommandAck => {
            }
           ,
            Self::Reserved => {
            }
           ,
            Self::Unknown (_) => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: MessageIdsV2Inner) requires Self::structural_valid (input),
    ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MessageIdsV2::from_structural) ;
        reveal(MessageIdsV2::into_structural) ;
        match input {
            L (x) => match x {
                75 => {
                }
               ,
                76 => {
                }
               ,
                77 => {
                }
               ,
                8388608 => {
                }
               ,
                _ => {
                    assert (false) ;
                }
            }
           ,
            R (_) => {
            }
           ,
        }
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MessageIdsV2Forward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MessageIdsV2Reverse ;
impl SpecMap for MessageIdsV2Forward {
    type Input = MessageIdsV2Inner ;
    type Output = MessageIdsV2Spec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MessageIdsV2::from_structural (input)
    }
}
impl SpecMap for MessageIdsV2Reverse {
    type Input = MessageIdsV2Spec ;
    type Output = MessageIdsV2Inner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}
# [cfg (not (verus_keep_ghost))] unsafe impl Structural for MessageIdsV2 {
}

# [doc = "data type for `message_ids_v1`."]
# [repr (u8)]
# [derive (Debug, PartialEq, Eq, Clone, Copy, StructuralEq)]
pub enum MessageIdsV1 {
    CommandInt = 75,
    CommandLong = 76,
    CommandAck = 77,
    Unknown (u8),
}
pub type MessageIdsV1Spec = MessageIdsV1 ;
pub type MessageIdsV1Inner = Sum < u8, u8 > ;
impl DeepView for MessageIdsV1 {
    type V = Self ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        * self
    }
}
impl MessageIdsV1 {
    pub proof fn lemma_deep_view (& self) ensures self.deep_view() == * self,
    {
        reveal(< MessageIdsV1 as DeepView>::deep_view) ;
    }
    pub open spec fn structural_valid (input: MessageIdsV1Inner) -> bool {
        match input {
            L (x) => x == 75 || x == 76 || x == 77,
            R (x) => true,
        }
    }
    # [verifier::opaque] pub open spec fn from_structural (input: MessageIdsV1Inner) -> Self {
        match input {
            L (x) => match x {
                75 => Self::CommandInt,
                76 => Self::CommandLong,
                77 => Self::CommandAck,
                _ => arbitrary(),
            }
           ,
            R (x) => Self::Unknown (x),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> MessageIdsV1Inner {
        match self {
            Self::CommandInt => L (75),
            Self::CommandLong => L (76),
            Self::CommandAck => L (77),
            Self::Unknown (x) => R (x),
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MessageIdsV1::from_structural) ;
        reveal(MessageIdsV1::into_structural) ;
        match self {
            Self::CommandInt => {
            }
           ,
            Self::CommandLong => {
            }
           ,
            Self::CommandAck => {
            }
           ,
            Self::Unknown (_) => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: MessageIdsV1Inner) requires Self::structural_valid (input),
    ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MessageIdsV1::from_structural) ;
        reveal(MessageIdsV1::into_structural) ;
        match input {
            L (x) => match x {
                75 => {
                }
               ,
                76 => {
                }
               ,
                77 => {
                }
               ,
                _ => {
                    assert (false) ;
                }
            }
           ,
            R (_) => {
            }
           ,
        }
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MessageIdsV1Forward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MessageIdsV1Reverse ;
impl SpecMap for MessageIdsV1Forward {
    type Input = MessageIdsV1Inner ;
    type Output = MessageIdsV1Spec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MessageIdsV1::from_structural (input)
    }
}
impl SpecMap for MessageIdsV1Reverse {
    type Input = MessageIdsV1Spec ;
    type Output = MessageIdsV1Inner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}
# [cfg (not (verus_keep_ghost))] unsafe impl Structural for MessageIdsV1 {
}

# [doc = "data type for `mav_cmd`."]
# [repr (u16)]
# [derive (Debug, PartialEq, Eq, Clone, Copy, StructuralEq)]
pub enum MavCmd {
    FlashBootloader = 42650,
    Unknown (u16),
}
pub type MavCmdSpec = MavCmd ;
pub type MavCmdInner = Sum < u16, u16 > ;
impl DeepView for MavCmd {
    type V = Self ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        * self
    }
}
impl MavCmd {
    pub proof fn lemma_deep_view (& self) ensures self.deep_view() == * self,
    {
        reveal(< MavCmd as DeepView>::deep_view) ;
    }
    pub open spec fn structural_valid (input: MavCmdInner) -> bool {
        match input {
            L (x) => x == 42650,
            R (x) => true,
        }
    }
    # [verifier::opaque] pub open spec fn from_structural (input: MavCmdInner) -> Self {
        match input {
            L (x) => match x {
                42650 => Self::FlashBootloader,
                _ => arbitrary(),
            }
           ,
            R (x) => Self::Unknown (x),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> MavCmdInner {
        match self {
            Self::FlashBootloader => L (42650),
            Self::Unknown (x) => R (x),
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MavCmd::from_structural) ;
        reveal(MavCmd::into_structural) ;
        match self {
            Self::FlashBootloader => {
            }
           ,
            Self::Unknown (_) => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: MavCmdInner) requires Self::structural_valid (input),
    ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MavCmd::from_structural) ;
        reveal(MavCmd::into_structural) ;
        match input {
            L (x) => match x {
                42650 => {
                }
               ,
                _ => {
                    assert (false) ;
                }
            }
           ,
            R (_) => {
            }
           ,
        }
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavCmdForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavCmdReverse ;
impl SpecMap for MavCmdForward {
    type Input = MavCmdInner ;
    type Output = MavCmdSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MavCmd::from_structural (input)
    }
}
impl SpecMap for MavCmdReverse {
    type Input = MavCmdSpec ;
    type Output = MavCmdInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}
# [cfg (not (verus_keep_ghost))] unsafe impl Structural for MavCmd {
}

# [doc = "data type for `mavlink_msg_msg`."]
# [derive (Debug, PartialEq, Eq, Clone, Copy)]
pub enum MavlinkMsgMsg<'i> {
    MavLink1 (MavlinkV1Msg<'i>),
    MavLink2 (MavlinkV2Msg<'i>),
}
# [verifier::ext_equal]
pub enum MavlinkMsgMsgSpec < T0 = MavlinkV1MsgSpec, T1 = MavlinkV2MsgSpec > {
    MavLink1 (T0),
    MavLink2 (T1),
}
pub type MavlinkMsgMsgInner = Sum < MavlinkV1MsgSpec, MavlinkV2MsgSpec > ;
impl<'i> DeepView for MavlinkMsgMsg<'i> {
    type V = MavlinkMsgMsgSpec ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        match self {
            MavlinkMsgMsg::MavLink1 (v) => MavlinkMsgMsgSpec::MavLink1 (v.deep_view()),
            MavlinkMsgMsg::MavLink2 (v) => MavlinkMsgMsgSpec::MavLink2 (v.deep_view()),
        }
    }
}
impl<'i> MavlinkMsgMsg<'i> {
    pub proof fn lemma_deep_view_fields (& self) ensures self.deep_view() == match self {
        MavlinkMsgMsg::MavLink1 (v) => MavlinkMsgMsgSpec::MavLink1 (v.deep_view()),
        MavlinkMsgMsg::MavLink2 (v) => MavlinkMsgMsgSpec::MavLink2 (v.deep_view()),
    }
   ,
    {
        reveal(< MavlinkMsgMsg as DeepView>::deep_view) ;
    }
}
impl < T0, T1 > MavlinkMsgMsgSpec < T0, T1 > {
    # [verifier::opaque] pub open spec fn from_structural (input: Sum < T0,
    T1 >) -> Self {
        match input {
            L (value) => Self::MavLink1 (value),
            R (value) => Self::MavLink2 (value),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> Sum < T0,
    T1 > {
        match self {
            Self::MavLink1 (value) => L (value),
            Self::MavLink2 (value) => R (value),
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MavlinkMsgMsgSpec::from_structural) ;
        reveal(MavlinkMsgMsgSpec::into_structural) ;
        match self {
            Self::MavLink1 (_) => {
            }
           ,
            Self::MavLink2 (_) => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: Sum < T0,
    T1 >) ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MavlinkMsgMsgSpec::from_structural) ;
        reveal(MavlinkMsgMsgSpec::into_structural) ;
        match input {
            L (_) => {
            }
           ,
            R (_) => {
            }
           ,
        }
    }
    pub proof fn lemma_into_structural_variant (self) ensures Self::into_structural (self) == match self {
        Self::MavLink1 (value) => L (value),
        Self::MavLink2 (value) => R (value),
    }
   ,
    {
        reveal(MavlinkMsgMsgSpec::into_structural) ;
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkMsgMsgForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkMsgMsgReverse ;
impl SpecMap for MavlinkMsgMsgForward {
    type Input = MavlinkMsgMsgInner ;
    type Output = MavlinkMsgMsgSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MavlinkMsgMsgSpec::from_structural (input)
    }
}
impl SpecMap for MavlinkMsgMsgReverse {
    type Input = MavlinkMsgMsgSpec ;
    type Output = MavlinkMsgMsgInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}

# [doc = "data type for `mavlink_v2_msg_signature`."]
# [derive (Debug, PartialEq, Eq, Clone, Copy)]
pub enum MavlinkV2MsgSignature<'i> {
    Signed (&'i [u8]),
    Default (&'i [u8]),
}
# [verifier::ext_equal]
pub enum MavlinkV2MsgSignatureSpec < T0 = Seq < u8 >, T1 = Seq < u8 > > {
    Signed (T0),
    Default (T1),
}
pub type MavlinkV2MsgSignatureInner = Sum < Seq < u8 >, Seq < u8 > > ;
impl<'i> DeepView for MavlinkV2MsgSignature<'i> {
    type V = MavlinkV2MsgSignatureSpec ;
    # [verifier::opaque] open spec fn deep_view (& self) -> Self::V {
        match self {
            MavlinkV2MsgSignature::Signed (v) => MavlinkV2MsgSignatureSpec::Signed (v.deep_view()),
            MavlinkV2MsgSignature::Default (v) => MavlinkV2MsgSignatureSpec::Default (v.deep_view()),
        }
    }
}
impl<'i> MavlinkV2MsgSignature<'i> {
    pub proof fn lemma_deep_view_fields (& self) ensures self.deep_view() == match self {
        MavlinkV2MsgSignature::Signed (v) => MavlinkV2MsgSignatureSpec::Signed (v.deep_view()),
        MavlinkV2MsgSignature::Default (v) => MavlinkV2MsgSignatureSpec::Default (v.deep_view()),
    }
   ,
    {
        reveal(< MavlinkV2MsgSignature as DeepView>::deep_view) ;
    }
}
impl < T0, T1 > MavlinkV2MsgSignatureSpec < T0, T1 > {
    # [verifier::opaque] pub open spec fn from_structural (input: Sum < T0,
    T1 >) -> Self {
        match input {
            L (value) => Self::Signed (value),
            R (value) => Self::Default (value),
        }
    }
    # [verifier::opaque] pub open spec fn into_structural (self) -> Sum < T0,
    T1 > {
        match self {
            Self::Signed (value) => L (value),
            Self::Default (value) => R (value),
        }
    }
    pub broadcast proof fn lemma_from_into (self) ensures # [trigger] Self::from_structural (Self::into_structural (self)) == self,
    {
        reveal(MavlinkV2MsgSignatureSpec::from_structural) ;
        reveal(MavlinkV2MsgSignatureSpec::into_structural) ;
        match self {
            Self::Signed (_) => {
            }
           ,
            Self::Default (_) => {
            }
           ,
        }
    }
    pub broadcast proof fn lemma_into_from (input: Sum < T0,
    T1 >) ensures # [trigger] Self::into_structural (Self::from_structural (input)) == input,
    {
        reveal(MavlinkV2MsgSignatureSpec::from_structural) ;
        reveal(MavlinkV2MsgSignatureSpec::into_structural) ;
        match input {
            L (_) => {
            }
           ,
            R (_) => {
            }
           ,
        }
    }
    pub proof fn lemma_into_structural_variant (self) ensures Self::into_structural (self) == match self {
        Self::Signed (value) => L (value),
        Self::Default (value) => R (value),
    }
   ,
    {
        reveal(MavlinkV2MsgSignatureSpec::into_structural) ;
    }
}
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkV2MsgSignatureForward ;
# [derive (Clone, Copy)]
# [doc (hidden)]
pub struct MavlinkV2MsgSignatureReverse ;
impl SpecMap for MavlinkV2MsgSignatureForward {
    type Input = MavlinkV2MsgSignatureInner ;
    type Output = MavlinkV2MsgSignatureSpec ;
    open spec fn spec_map (& self,
    input: Self::Input) -> Self::Output {
        MavlinkV2MsgSignatureSpec::from_structural (input)
    }
}
impl SpecMap for MavlinkV2MsgSignatureReverse {
    type Input = MavlinkV2MsgSignatureSpec ;
    type Output = MavlinkV2MsgSignatureInner ;
    open spec fn spec_map (& self,
    value: Self::Input) -> Self::Output {
        value.into_structural()
    }
}

// ============================================================
// Format Specifications
// ============================================================
# [doc = "named format combinator for `mavlink_msg`."]
# [derive (Clone, Copy)]
pub struct MavlinkMsgFmt ;

pub type MavlinkMsgFmtSpec = Named < Mapped < Bind < ProtocolMagicFmt, spec_fn (ProtocolMagicSpec) -> MavlinkMsgMsgFmt >, BiMap < MavlinkMsgForward, MavlinkMsgReverse >> > ;

impl MavlinkMsgFmt {
    # [doc = "specification constructor for `mavlink_msg`."] pub open spec fn spec_inner() -> MavlinkMsgFmtSpec {
        Named ("mavlink_msg",
        Mapped {
            inner: Bind (ProtocolMagicFmt,
            | magic: ProtocolMagicSpec | MavlinkMsgMsgFmt::spec (magic)),
            mapper: BiMap (MavlinkMsgForward,
            MavlinkMsgReverse),
        }
        )
    }
}


# [doc = "named format combinator for `mavlink_v2_msg`."]
# [derive (Clone, Copy)]
pub struct MavlinkV2MsgFmt ;

pub type MavlinkV2MsgFmtSpec = Named < Mapped < Bind < U8, spec_fn (u8) -> Bind < IncompatFlagsFmt, spec_fn (IncompatFlagsSpec) -> Pair < U8, Pair < U8, Pair < Refined < U8, PredFnSpec < u8 >>, Pair < Refined < U8, PredFnSpec < u8 >>, Pair < MessageIdsV2Fmt, Pair < Varied < u8 >, Pair < U16Le, MavlinkV2MsgSignatureFmt > > > > > > > > >, BiMap < MavlinkV2MsgForward, MavlinkV2MsgReverse >> > ;

impl MavlinkV2MsgFmt {
    # [doc = "specification constructor for `mavlink_v2_msg`."] pub open spec fn spec_inner() -> MavlinkV2MsgFmtSpec {
        Named ("mavlink_v2_msg",
        Mapped {
            inner: Bind (U8,
            | len: u8 | Bind (IncompatFlagsFmt,
            | incompat_flags: IncompatFlagsSpec | Pair (U8,
            Pair (U8,
            Pair (Refined (U8,
            | x: u8 | x >= 1),
            Pair (Refined (U8,
            | x: u8 | x >= 1),
            Pair (MessageIdsV2Fmt,
            Pair (Varied (len),
            Pair (U16Le,
            MavlinkV2MsgSignatureFmt::spec (incompat_flags)))))))))),
            mapper: BiMap (MavlinkV2MsgForward,
            MavlinkV2MsgReverse),
        }
        )
    }
}


# [doc = "named format combinator for `mavlink_v1_msg`."]
# [derive (Clone, Copy)]
pub struct MavlinkV1MsgFmt ;

pub type MavlinkV1MsgFmtSpec = Named < Mapped < Bind < U8, spec_fn (u8) -> Pair < U8, Pair < Refined < U8, PredFnSpec < u8 >>, Pair < Refined < U8, PredFnSpec < u8 >>, Pair < MessageIdsV1Fmt, Pair < Varied < u8 >, U16Le > > > > > >, BiMap < MavlinkV1MsgForward, MavlinkV1MsgReverse >> > ;

impl MavlinkV1MsgFmt {
    # [doc = "specification constructor for `mavlink_v1_msg`."] pub open spec fn spec_inner() -> MavlinkV1MsgFmtSpec {
        Named ("mavlink_v1_msg",
        Mapped {
            inner: Bind (U8,
            | len: u8 | Pair (U8,
            Pair (Refined (U8,
            | x: u8 | x >= 1),
            Pair (Refined (U8,
            | x: u8 | x >= 1),
            Pair (MessageIdsV1Fmt,
            Pair (Varied (len),
            U16Le)))))),
            mapper: BiMap (MavlinkV1MsgForward,
            MavlinkV1MsgReverse),
        }
        )
    }
}


# [doc = "named format combinator for `protocol_magic`."]
# [derive (Clone, Copy)]
pub struct ProtocolMagicFmt ;

pub type ProtocolMagicFmtSpec = Named < Mapped < Refined < U8, PredFnSpec < u8 >>, BiMap < ProtocolMagicForward, ProtocolMagicReverse >> > ;

impl ProtocolMagicFmt {
    # [doc = "specification constructor for `protocol_magic`."] pub open spec fn spec_inner() -> ProtocolMagicFmtSpec {
        Named ("protocol_magic",
        Mapped {
            inner: Refined (U8,
            | x: u8 | (x == 254) || (x == 253)),
            mapper: BiMap (ProtocolMagicForward,
            ProtocolMagicReverse),
        }
        )
    }
}


# [doc = "named format combinator for `incompat_flags`."]
# [derive (Clone, Copy)]
pub struct IncompatFlagsFmt ;

pub type IncompatFlagsFmtSpec = Named < Mapped < Choice < Refined < U8, PredFnSpec < u8 >>, Refined < U8, PredFnSpec < u8 >> >, BiMap < IncompatFlagsForward, IncompatFlagsReverse >> > ;

impl IncompatFlagsFmt {
    # [doc = "specification constructor for `incompat_flags`."] pub open spec fn spec_inner() -> IncompatFlagsFmtSpec {
        Named ("incompat_flags",
        Mapped {
            inner: Choice (Refined (U8,
            | x: u8 | x == 1),
            Refined (U8,
            | x: u8 | x != 1)),
            mapper: BiMap (IncompatFlagsForward,
            IncompatFlagsReverse),
        }
        )
    }
}


# [doc = "named format combinator for `message_ids_v2`."]
# [derive (Clone, Copy)]
pub struct MessageIdsV2Fmt ;

pub type MessageIdsV2FmtSpec = Named < Mapped < Choice < Refined < U24Le, PredFnSpec < u32 >>, Refined < U24Le, PredFnSpec < u32 >> >, BiMap < MessageIdsV2Forward, MessageIdsV2Reverse >> > ;

impl MessageIdsV2Fmt {
    # [doc = "specification constructor for `message_ids_v2`."] pub open spec fn spec_inner() -> MessageIdsV2FmtSpec {
        Named ("message_ids_v2",
        Mapped {
            inner: Choice (Refined (U24Le,
            | x: u32 | (((x == 75) || (x == 76)) || (x == 77)) || (x == 8388608)),
            Refined (U24Le,
            | x: u32 | (((x != 75) && (x != 76)) && (x != 77)) && (x != 8388608))),
            mapper: BiMap (MessageIdsV2Forward,
            MessageIdsV2Reverse),
        }
        )
    }
}


# [doc = "named format combinator for `message_ids_v1`."]
# [derive (Clone, Copy)]
pub struct MessageIdsV1Fmt ;

pub type MessageIdsV1FmtSpec = Named < Mapped < Choice < Refined < U8, PredFnSpec < u8 >>, Refined < U8, PredFnSpec < u8 >> >, BiMap < MessageIdsV1Forward, MessageIdsV1Reverse >> > ;

impl MessageIdsV1Fmt {
    # [doc = "specification constructor for `message_ids_v1`."] pub open spec fn spec_inner() -> MessageIdsV1FmtSpec {
        Named ("message_ids_v1",
        Mapped {
            inner: Choice (Refined (U8,
            | x: u8 | ((x == 75) || (x == 76)) || (x == 77)),
            Refined (U8,
            | x: u8 | ((x != 75) && (x != 76)) && (x != 77))),
            mapper: BiMap (MessageIdsV1Forward,
            MessageIdsV1Reverse),
        }
        )
    }
}


# [doc = "named format combinator for `mav_cmd`."]
# [derive (Clone, Copy)]
pub struct MavCmdFmt ;

pub type MavCmdFmtSpec = Named < Mapped < Choice < Refined < U16Le, PredFnSpec < u16 >>, Refined < U16Le, PredFnSpec < u16 >> >, BiMap < MavCmdForward, MavCmdReverse >> > ;

impl MavCmdFmt {
    # [doc = "specification constructor for `mav_cmd`."] pub open spec fn spec_inner() -> MavCmdFmtSpec {
        Named ("mav_cmd",
        Mapped {
            inner: Choice (Refined (U16Le,
            | x: u16 | x == 42650),
            Refined (U16Le,
            | x: u16 | x != 42650)),
            mapper: BiMap (MavCmdForward,
            MavCmdReverse),
        }
        )
    }
}


# [doc = "named format combinator for `mavlink_msg_msg`."]
# [derive (Clone, Copy)]
pub struct MavlinkMsgMsgFmt {
    magic: ProtocolMagic,
}
impl MavlinkMsgMsgFmt {
    # [verifier::type_invariant] spec fn wf (& self) -> bool {
        ProtocolMagicFmt.consistent (self.magic.deep_view())
    }
    pub closed spec fn magic_spec (& self) -> ProtocolMagicSpec {
        self.magic.deep_view()
    }
    pub closed spec fn spec (magic: ProtocolMagic) -> Self {
        MavlinkMsgMsgFmt {
            magic
        }
    }
}

pub type MavlinkMsgMsgFmtSpec = Named < Mapped < Sum < MavlinkV1MsgFmt, MavlinkV2MsgFmt >, BiMap < MavlinkMsgMsgForward, MavlinkMsgMsgReverse >> > ;

impl MavlinkMsgMsgFmt {
    # [doc = "specification constructor for `mavlink_msg_msg`."] pub open spec fn spec_inner (magic: ProtocolMagicSpec) -> MavlinkMsgMsgFmtSpec {
        Named ("mavlink_msg_msg",
        Mapped {
            inner: match magic {
                ProtocolMagicSpec::MavLink1 => L (MavlinkV1MsgFmt),
                ProtocolMagicSpec::MavLink2 => R (MavlinkV2MsgFmt),
            }
           ,
            mapper: BiMap (MavlinkMsgMsgForward,
            MavlinkMsgMsgReverse),
        }
        )
    }
}


# [doc = "named format combinator for `mavlink_v2_msg_signature`."]
# [derive (Clone, Copy)]
pub struct MavlinkV2MsgSignatureFmt {
    incompat_flags: IncompatFlags,
}
impl MavlinkV2MsgSignatureFmt {
    # [verifier::type_invariant] spec fn wf (& self) -> bool {
        IncompatFlagsFmt.consistent (self.incompat_flags.deep_view())
    }
    pub closed spec fn incompat_flags_spec (& self) -> IncompatFlagsSpec {
        self.incompat_flags.deep_view()
    }
    pub closed spec fn spec (incompat_flags: IncompatFlags) -> Self {
        MavlinkV2MsgSignatureFmt {
            incompat_flags
        }
    }
}

pub type MavlinkV2MsgSignatureFmtSpec = Named < Mapped < Sum < Fixed < 13 >, Fixed < 0 > >, BiMap < MavlinkV2MsgSignatureForward, MavlinkV2MsgSignatureReverse >> > ;

impl MavlinkV2MsgSignatureFmt {
    # [doc = "specification constructor for `mavlink_v2_msg_signature`."] pub open spec fn spec_inner (incompat_flags: IncompatFlagsSpec) -> MavlinkV2MsgSignatureFmtSpec {
        Named ("mavlink_v2_msg_signature",
        Mapped {
            inner: match incompat_flags {
                IncompatFlagsSpec::Signed => L (Fixed::< 13 >),
                _ => R (Fixed::< 0 >),
            }
           ,
            mapper: BiMap (MavlinkV2MsgSignatureForward,
            MavlinkV2MsgSignatureReverse),
        }
        )
    }
}

// ============================================================
// Derived Parser, Serializer, Length, and Consistency Specifications
// ============================================================
mod derived_specs {
    use super::*;

    impl SpecParser for MavlinkMsgFmt {
        type PVal = MavlinkMsgSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for MavlinkMsgFmt {
        type Val = MavlinkMsgSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for MavlinkMsgFmt {
        type SValue = MavlinkMsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MavlinkMsgFmt {
        type SVal = MavlinkMsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for MavlinkMsgFmt {
        type T = MavlinkMsgSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for MavlinkV2MsgFmt {
        type PVal = MavlinkV2MsgSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for MavlinkV2MsgFmt {
        type Val = MavlinkV2MsgSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for MavlinkV2MsgFmt {
        type SValue = MavlinkV2MsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MavlinkV2MsgFmt {
        type SVal = MavlinkV2MsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for MavlinkV2MsgFmt {
        type T = MavlinkV2MsgSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for MavlinkV1MsgFmt {
        type PVal = MavlinkV1MsgSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for MavlinkV1MsgFmt {
        type Val = MavlinkV1MsgSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for MavlinkV1MsgFmt {
        type SValue = MavlinkV1MsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MavlinkV1MsgFmt {
        type SVal = MavlinkV1MsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for MavlinkV1MsgFmt {
        type T = MavlinkV1MsgSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for ProtocolMagicFmt {
        type PVal = ProtocolMagicSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for ProtocolMagicFmt {
        type Val = ProtocolMagicSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for ProtocolMagicFmt {
        type SValue = ProtocolMagicSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for ProtocolMagicFmt {
        type SVal = ProtocolMagicSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for ProtocolMagicFmt {
        type T = ProtocolMagicSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for IncompatFlagsFmt {
        type PVal = IncompatFlagsSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for IncompatFlagsFmt {
        type Val = IncompatFlagsSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for IncompatFlagsFmt {
        type SValue = IncompatFlagsSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for IncompatFlagsFmt {
        type SVal = IncompatFlagsSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for IncompatFlagsFmt {
        type T = IncompatFlagsSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for MessageIdsV2Fmt {
        type PVal = MessageIdsV2Spec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for MessageIdsV2Fmt {
        type Val = MessageIdsV2Spec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for MessageIdsV2Fmt {
        type SValue = MessageIdsV2Spec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MessageIdsV2Fmt {
        type SVal = MessageIdsV2Spec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for MessageIdsV2Fmt {
        type T = MessageIdsV2Spec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for MessageIdsV1Fmt {
        type PVal = MessageIdsV1Spec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for MessageIdsV1Fmt {
        type Val = MessageIdsV1Spec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for MessageIdsV1Fmt {
        type SValue = MessageIdsV1Spec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MessageIdsV1Fmt {
        type SVal = MessageIdsV1Spec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for MessageIdsV1Fmt {
        type T = MessageIdsV1Spec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for MavCmdFmt {
        type PVal = MavCmdSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner().spec_parse (ibuf)
        }
    }
    impl Consistency for MavCmdFmt {
        type Val = MavCmdSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner().consistent (v)
        }
    }
    impl SpecSerializerDps for MavCmdFmt {
        type SValue = MavCmdSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner().spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MavCmdFmt {
        type SVal = MavCmdSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner().spec_serialize (v)
        }
    }
    impl SpecByteLen for MavCmdFmt {
        type T = MavCmdSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner().byte_len (v)
        }
    }

    impl SpecParser for MavlinkMsgMsgFmt {
        type PVal = MavlinkMsgMsgSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner (self.magic_spec()).spec_parse (ibuf)
        }
    }
    impl Consistency for MavlinkMsgMsgFmt {
        type Val = MavlinkMsgMsgSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner (self.magic_spec()).consistent (v)
        }
    }
    impl SpecSerializerDps for MavlinkMsgMsgFmt {
        type SValue = MavlinkMsgMsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner (self.magic_spec()).spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MavlinkMsgMsgFmt {
        type SVal = MavlinkMsgMsgSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner (self.magic_spec()).spec_serialize (v)
        }
    }
    impl SpecByteLen for MavlinkMsgMsgFmt {
        type T = MavlinkMsgMsgSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner (self.magic_spec()).byte_len (v)
        }
    }

    impl SpecParser for MavlinkV2MsgSignatureFmt {
        type PVal = MavlinkV2MsgSignatureSpec ;
        # [verifier::opaque] open spec fn spec_parse (& self,
        ibuf: Seq < u8 >) -> Option < (int,
        Self::PVal) > {
            Self::spec_inner (self.incompat_flags_spec()).spec_parse (ibuf)
        }
    }
    impl Consistency for MavlinkV2MsgSignatureFmt {
        type Val = MavlinkV2MsgSignatureSpec ;
        open spec fn consistent (& self,
        v: Self::Val) -> bool {
            Self::spec_inner (self.incompat_flags_spec()).consistent (v)
        }
    }
    impl SpecSerializerDps for MavlinkV2MsgSignatureFmt {
        type SValue = MavlinkV2MsgSignatureSpec ;
        # [verifier::opaque] open spec fn spec_serialize_dps (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) -> Seq < u8 > {
            Self::spec_inner (self.incompat_flags_spec()).spec_serialize_dps (v,
            obuf)
        }
    }
    impl SpecSerializer for MavlinkV2MsgSignatureFmt {
        type SVal = MavlinkV2MsgSignatureSpec ;
        # [verifier::opaque] open spec fn spec_serialize (& self,
        v: Self::SVal) -> Seq < u8 > {
            Self::spec_inner (self.incompat_flags_spec()).spec_serialize (v)
        }
    }
    impl SpecByteLen for MavlinkV2MsgSignatureFmt {
        type T = MavlinkV2MsgSignatureSpec ;
        # [verifier::opaque] open spec fn byte_len (& self,
        v: Self::T) -> nat {
            Self::spec_inner (self.incompat_flags_spec()).byte_len (v)
        }
    }
}

// ============================================================
// Proven Format Properties
// ============================================================
mod derived_proofs {
    use super::*;
    broadcast use {
        vest_lib::combinators::disjoint::disjointness_lemmas,
        MavlinkMsgSpec::lemma_from_into,
        MavlinkMsgSpec::lemma_into_from,
        MavlinkV2MsgSpec::lemma_from_into,
        MavlinkV2MsgSpec::lemma_into_from,
        MavlinkV1MsgSpec::lemma_from_into,
        MavlinkV1MsgSpec::lemma_into_from,
        ProtocolMagic::lemma_from_into,
        ProtocolMagic::lemma_into_from,
        IncompatFlags::lemma_from_into,
        IncompatFlags::lemma_into_from,
        MessageIdsV2::lemma_from_into,
        MessageIdsV2::lemma_into_from,
        MessageIdsV1::lemma_from_into,
        MessageIdsV1::lemma_into_from,
        MavCmd::lemma_from_into,
        MavCmd::lemma_into_from,
        MavlinkMsgMsgSpec::lemma_from_into,
        MavlinkMsgMsgSpec::lemma_into_from,
        MavlinkV2MsgSignatureSpec::lemma_from_into,
        MavlinkV2MsgSignatureSpec::lemma_into_from,
    };

    impl SafeParser for MavlinkMsgFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MavlinkMsgFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MavlinkMsgFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkMsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkMsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkMsgFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkMsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkMsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MavlinkMsgFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MavlinkMsgFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MavlinkMsgFmt as SpecSerializer>::spec_serialize) ;
            reveal(< MavlinkMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MavlinkMsgFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgFmt as Consistency>::consistent) ;
            reveal(< MavlinkMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: MavlinkMsgSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MavlinkMsgSpec::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MavlinkMsgFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkMsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkMsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MavlinkMsgFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MavlinkMsgFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MavlinkMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MavlinkV2MsgFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MavlinkV2MsgFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MavlinkV2MsgFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV2MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkV2MsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV2MsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV2MsgFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkV2MsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV2MsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MavlinkV2MsgFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MavlinkV2MsgFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MavlinkV2MsgFmt as SpecSerializer>::spec_serialize) ;
            reveal(< MavlinkV2MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MavlinkV2MsgFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV2MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgFmt as Consistency>::consistent) ;
            reveal(< MavlinkV2MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: MavlinkV2MsgSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MavlinkV2MsgSpec::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MavlinkV2MsgFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkV2MsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV2MsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MavlinkV2MsgFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MavlinkV2MsgFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MavlinkV2MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MavlinkV1MsgFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MavlinkV1MsgFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MavlinkV1MsgFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV1MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkV1MsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV1MsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV1MsgFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkV1MsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV1MsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MavlinkV1MsgFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV1MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MavlinkV1MsgFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MavlinkV1MsgFmt as SpecSerializer>::spec_serialize) ;
            reveal(< MavlinkV1MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MavlinkV1MsgFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV1MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV1MsgFmt as Consistency>::consistent) ;
            reveal(< MavlinkV1MsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: MavlinkV1MsgSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MavlinkV1MsgSpec::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MavlinkV1MsgFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavlinkV1MsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV1MsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MavlinkV1MsgFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV1MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV1MsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MavlinkV1MsgFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MavlinkV1MsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV1MsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for ProtocolMagicFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for ProtocolMagicFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for ProtocolMagicFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecParser>::spec_parse) ;
            reveal(< ProtocolMagicFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: ProtocolMagicInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (ProtocolMagic::structural_valid (input)) ;
                ProtocolMagic::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecParser>::spec_parse) ;
            reveal(< ProtocolMagicFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: ProtocolMagicInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (ProtocolMagic::structural_valid (input)) ;
                ProtocolMagic::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for ProtocolMagicFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< ProtocolMagicFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for ProtocolMagicFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< ProtocolMagicFmt as SpecSerializer>::spec_serialize) ;
            reveal(< ProtocolMagicFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for ProtocolMagicFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecParser>::spec_parse) ;
            reveal(< ProtocolMagicFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< ProtocolMagicFmt as Consistency>::consistent) ;
            reveal(< ProtocolMagicFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: ProtocolMagicSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                ProtocolMagic::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for ProtocolMagicFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: ProtocolMagicInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (ProtocolMagic::structural_valid (input)) ;
                ProtocolMagic::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for ProtocolMagicFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< ProtocolMagicFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< ProtocolMagicFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for ProtocolMagicFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< ProtocolMagicFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< ProtocolMagicFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for IncompatFlagsFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for IncompatFlagsFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for IncompatFlagsFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecParser>::spec_parse) ;
            reveal(< IncompatFlagsFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: IncompatFlagsInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (IncompatFlags::structural_valid (input)) ;
                IncompatFlags::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecParser>::spec_parse) ;
            reveal(< IncompatFlagsFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: IncompatFlagsInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (IncompatFlags::structural_valid (input)) ;
                IncompatFlags::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for IncompatFlagsFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< IncompatFlagsFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for IncompatFlagsFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< IncompatFlagsFmt as SpecSerializer>::spec_serialize) ;
            reveal(< IncompatFlagsFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for IncompatFlagsFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecParser>::spec_parse) ;
            reveal(< IncompatFlagsFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< IncompatFlagsFmt as Consistency>::consistent) ;
            reveal(< IncompatFlagsFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: IncompatFlagsSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                IncompatFlags::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for IncompatFlagsFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: IncompatFlagsInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (IncompatFlags::structural_valid (input)) ;
                IncompatFlags::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for IncompatFlagsFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< IncompatFlagsFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< IncompatFlagsFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for IncompatFlagsFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< IncompatFlagsFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< IncompatFlagsFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MessageIdsV2Fmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MessageIdsV2Fmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MessageIdsV2Fmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecParser>::spec_parse) ;
            reveal(< MessageIdsV2Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MessageIdsV2Inner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MessageIdsV2::structural_valid (input)) ;
                MessageIdsV2::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecParser>::spec_parse) ;
            reveal(< MessageIdsV2Fmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MessageIdsV2Inner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MessageIdsV2::structural_valid (input)) ;
                MessageIdsV2::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MessageIdsV2Fmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV2Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MessageIdsV2Fmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MessageIdsV2Fmt as SpecSerializer>::spec_serialize) ;
            reveal(< MessageIdsV2Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MessageIdsV2Fmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecParser>::spec_parse) ;
            reveal(< MessageIdsV2Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV2Fmt as Consistency>::consistent) ;
            reveal(< MessageIdsV2Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: MessageIdsV2Spec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MessageIdsV2::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MessageIdsV2Fmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MessageIdsV2Inner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MessageIdsV2::structural_valid (input)) ;
                MessageIdsV2::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MessageIdsV2Fmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV2Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV2Fmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MessageIdsV2Fmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MessageIdsV2Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV2Fmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MessageIdsV1Fmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MessageIdsV1Fmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MessageIdsV1Fmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecParser>::spec_parse) ;
            reveal(< MessageIdsV1Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MessageIdsV1Inner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MessageIdsV1::structural_valid (input)) ;
                MessageIdsV1::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecParser>::spec_parse) ;
            reveal(< MessageIdsV1Fmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MessageIdsV1Inner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MessageIdsV1::structural_valid (input)) ;
                MessageIdsV1::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MessageIdsV1Fmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV1Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MessageIdsV1Fmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MessageIdsV1Fmt as SpecSerializer>::spec_serialize) ;
            reveal(< MessageIdsV1Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MessageIdsV1Fmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecParser>::spec_parse) ;
            reveal(< MessageIdsV1Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV1Fmt as Consistency>::consistent) ;
            reveal(< MessageIdsV1Fmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: MessageIdsV1Spec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MessageIdsV1::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MessageIdsV1Fmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MessageIdsV1Inner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MessageIdsV1::structural_valid (input)) ;
                MessageIdsV1::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MessageIdsV1Fmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MessageIdsV1Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV1Fmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MessageIdsV1Fmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MessageIdsV1Fmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MessageIdsV1Fmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MavCmdFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecParser>::spec_parse) ;
            Self::spec_inner().lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MavCmdFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner().productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MavCmdFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecParser>::spec_parse) ;
            reveal(< MavCmdFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavCmdInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MavCmd::structural_valid (input)) ;
                MavCmd::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecParser>::spec_parse) ;
            reveal(< MavCmdFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavCmdInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MavCmd::structural_valid (input)) ;
                MavCmd::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MavCmdFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavCmdFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MavCmdFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MavCmdFmt as SpecSerializer>::spec_serialize) ;
            reveal(< MavCmdFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MavCmdFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecParser>::spec_parse) ;
            reveal(< MavCmdFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavCmdFmt as Consistency>::consistent) ;
            reveal(< MavCmdFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner() ;
            assert forall | output: MavCmdSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MavCmd::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MavCmdFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner() ;
            assert forall | input: MavCmdInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                assert (MavCmd::structural_valid (input)) ;
                MavCmd::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MavCmdFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MavCmdFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavCmdFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MavCmdFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MavCmdFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavCmdFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner() ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MavlinkMsgMsgFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecParser>::spec_parse) ;
            Self::spec_inner (self.magic_spec()).lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MavlinkMsgMsgFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner (self.magic_spec()).productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MavlinkMsgMsgFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkMsgMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert forall | input: MavlinkMsgMsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkMsgMsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkMsgMsgFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert forall | input: MavlinkMsgMsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkMsgMsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MavlinkMsgMsgFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MavlinkMsgMsgFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MavlinkMsgMsgFmt as SpecSerializer>::spec_serialize) ;
            reveal(< MavlinkMsgMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MavlinkMsgMsgFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkMsgMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgMsgFmt as Consistency>::consistent) ;
            reveal(< MavlinkMsgMsgFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert forall | output: MavlinkMsgMsgSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MavlinkMsgMsgSpec::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MavlinkMsgMsgFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert forall | input: MavlinkMsgMsgInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkMsgMsgSpec::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MavlinkMsgMsgFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MavlinkMsgMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgMsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MavlinkMsgMsgFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MavlinkMsgMsgFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkMsgMsgFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner (self.magic_spec()) ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }

    impl SafeParser for MavlinkV2MsgSignatureFmt {
        proof fn lemma_parse_safe (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse) ;
            Self::spec_inner (self.incompat_flags_spec()).lemma_parse_safe (ibuf) ;
        }
    }
    impl Productive for MavlinkV2MsgSignatureFmt {
        open spec fn productive_inv (& self) -> bool {
            Self::spec_inner (self.incompat_flags_spec()).productive_inv()
        }
        proof fn lemma_productive (& self,
        s: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert (fmt.productive_inv()) ;
            fmt.lemma_productive (s) ;
        }
    }
    impl SoundParser for MavlinkV2MsgSignatureFmt {
        proof fn lemma_parse_sound_consumption (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert forall | input: MavlinkV2MsgSignatureInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV2MsgSignatureSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_consumption (ibuf) ;
        }
        proof fn lemma_parse_sound_value (& self,
        ibuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV2MsgSignatureFmt as Consistency>::consistent) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert forall | input: MavlinkV2MsgSignatureInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV2MsgSignatureSpec::lemma_into_from (input) ;
            }
            assert (fmt.sound_inv()) ;
            fmt.lemma_parse_sound_value (ibuf) ;
        }
    }
    impl NonTailFmt for MavlinkV2MsgSignatureFmt {
        proof fn lemma_serialize_dps_prepend (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializerDps>::spec_serialize_dps) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_prepend (v,
            obuf) ;
        }
        proof fn lemma_serialize_dps_len (& self,
        v: Self::SValue,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert (fmt.serialize_dps_inv()) ;
            fmt.lemma_serialize_dps_len (v,
            obuf) ;
        }
    }
    impl GoodSerializer for MavlinkV2MsgSignatureFmt {
        proof fn lemma_serialize_len (& self,
        v: Self::SVal) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializer>::spec_serialize) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert (fmt.serialize_inv()) ;
            fmt.lemma_serialize_len (v) ;
        }
    }
    impl SPRoundTripDps for MavlinkV2MsgSignatureFmt {
        proof fn theorem_serialize_dps_parse_roundtrip (& self,
        v: Self::T,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgSignatureFmt as Consistency>::consistent) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecByteLen>::byte_len) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert forall | output: MavlinkV2MsgSignatureSpec | # [trigger] fmt.1.consistent (output) implies fmt.1.mapper.sound (output) by {
                MavlinkV2MsgSignatureSpec::lemma_from_into (output) ;
            }
            assert (fmt.unambiguous()) ;
            fmt.theorem_serialize_dps_parse_roundtrip (v,
            obuf) ;
        }
    }
    impl NonMalleable for MavlinkV2MsgSignatureFmt {
        proof fn lemma_parse_non_malleable (& self,
        buf1: Seq < u8 >,
        buf2: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert forall | input: MavlinkV2MsgSignatureInner | # [trigger] fmt.1.inner.consistent (input) implies fmt.1.mapper.lossless (input) by {
                MavlinkV2MsgSignatureSpec::lemma_into_from (input) ;
            }
            assert (fmt.nonmal_inv()) ;
            fmt.lemma_parse_non_malleable (buf1,
            buf2) ;
        }
    }
    impl EquivSerializersGeneral for MavlinkV2MsgSignatureFmt {
        proof fn lemma_serialize_equiv (& self,
        v: Self::SVal,
        obuf: Seq < u8 >) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert (fmt.equiv_general_inv()) ;
            fmt.lemma_serialize_equiv (v,
            obuf) ;
        }
    }
    impl EquivSerializers for MavlinkV2MsgSignatureFmt {
        proof fn lemma_serialize_equiv_on_empty (& self,
        v: Self::SVal) {
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializerDps>::spec_serialize_dps) ;
            reveal(< MavlinkV2MsgSignatureFmt as SpecSerializer>::spec_serialize) ;
            let fmt = Self::spec_inner (self.incompat_flags_spec()) ;
            assert (fmt.equiv_inv()) ;
            fmt.lemma_serialize_equiv_on_empty (v) ;
        }
    }
}

// ============================================================
// Executable Implementations
// ============================================================
mod exec_impls {
    use super::*;

    impl<'i> Parser<&'i [u8]> for MavlinkMsgFmt {
        type PT = MavlinkMsg<'i>;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            broadcast use vest_lib::core::spec::SafeParser::lemma_parse_safe;
            broadcast use vest_lib::core::spec::SoundParser::lemma_parse_sound_value;

            reveal(<MavlinkMsgFmt as SpecParser>::spec_parse);
            reveal(<MavlinkMsg as DeepView>::deep_view);
            reveal(MavlinkMsgSpec::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n1, magic) = (Named ("protocol_magic", ProtocolMagicFmt)).parse (& rest) ?;
            proof {
                magic.lemma_deep_view();
            }
            let rest = rest.skip(n1);
            proof {
                magic.lemma_deep_view();
            }

            let (n2, msg) = (Named ("mavlink_msg_msg", MavlinkMsgMsgFmt {
                magic: magic
            }
            )).parse (& rest) ?;
            let rest = rest.skip(n2);
            let total_n = n1 + n2;
            let final_v = MavlinkMsg {
                magic,
                msg,
            };
            assert(self.spec_parse(ibuf@) == Some((total_n as int, final_v.deep_view())));
            Ok((total_n, final_v))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MavlinkMsg<'i>> for MavlinkMsgFmt {
        fn serialize_into(&self, v: &MavlinkMsg<'i>, obuf: &mut Output) {
            broadcast use vest_lib::core::exec::output::outbuf_lemmas;
            reveal(<MavlinkMsgFmt as SpecSerializer>::spec_serialize);
            reveal(<MavlinkMsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkMsg as DeepView>::deep_view);
            reveal(MavlinkMsgSpec::into_structural);
            let ghost old_obuf = obuf@;

            let MavlinkMsg {
                magic,
                msg,
            } = v;
            proof {
                magic.lemma_deep_view();
            }

            ProtocolMagicFmt.serialize_into(magic, obuf);
            MavlinkMsgMsgFmt {
                magic: * magic
            }
            .serialize_into(msg, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MavlinkMsg<'i>> for MavlinkMsgFmt {
        fn prepare(&self, v: &MavlinkMsg<'i>) -> Result<usize, PreSerializeError> {
            reveal(<MavlinkMsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkMsg as DeepView>::deep_view);
            reveal(MavlinkMsgSpec::into_structural);
            let MavlinkMsg {
                magic,
                msg,
            } = v;
            proof {
                magic.lemma_deep_view();
            }

            let l1 = (Named ("protocol_magic", ProtocolMagicFmt)).prepare (magic) ?;
            let l2 = (Named ("mavlink_msg_msg", MavlinkMsgMsgFmt {
                magic: * magic
            }
            )).prepare (msg) ?;
            let total_len = l1.checked_add (l2).ok_or (PreSerializeError::length_too_large()) ?;
            Ok(total_len)
        }
    }



    impl<'i> Parser<&'i [u8]> for MavlinkV2MsgFmt {
        type PT = MavlinkV2Msg<'i>;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            broadcast use vest_lib::core::spec::SafeParser::lemma_parse_safe;
            broadcast use vest_lib::core::spec::SoundParser::lemma_parse_sound_value;

            reveal(<MavlinkV2MsgFmt as SpecParser>::spec_parse);
            reveal(<MavlinkV2Msg as DeepView>::deep_view);
            reveal(MavlinkV2MsgSpec::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n1, len) = (U8).parse (& rest) ?;
            let rest = rest.skip(n1);
            let (n2, incompat_flags) = (Named ("incompat_flags", IncompatFlagsFmt)).parse (& rest) ?;
            proof {
                incompat_flags.lemma_deep_view();
            }
            let rest = rest.skip(n2);
            let (n3, compat_flags) = (U8).parse (& rest) ?;
            let rest = rest.skip(n3);
            let (n4, seq) = (U8).parse (& rest) ?;
            let rest = rest.skip(n4);
            let (n5, sysid) = (U8).parse (& rest) ?;
            if !(sysid >= 1) {
                return Err(ParseError::predicate_failed());
            }
            let rest = rest.skip(n5);
            let (n6, compid) = (U8).parse (& rest) ?;
            if !(compid >= 1) {
                return Err(ParseError::predicate_failed());
            }
            let rest = rest.skip(n6);
            let (n7, msgid) = (Named ("message_ids_v2", MessageIdsV2Fmt)).parse (& rest) ?;
            proof {
                msgid.lemma_deep_view();
            }
            let rest = rest.skip(n7);
            let (n8, payload) = (Varied (len)).parse (& rest) ?;
            let rest = rest.skip(n8);
            let (n9, checksum) = (U16Le).parse (& rest) ?;
            let rest = rest.skip(n9);
            proof {
                incompat_flags.lemma_deep_view();
            }

            let (n10, signature) = (Named ("mavlink_v2_msg_signature", MavlinkV2MsgSignatureFmt {
                incompat_flags: incompat_flags
            }
            )).parse (& rest) ?;
            let rest = rest.skip(n10);
            let total_n = n1 + n2 + n3 + n4 + n5 + n6 + n7 + n8 + n9 + n10;
            let final_v = MavlinkV2Msg {
                len,
                incompat_flags,
                compat_flags,
                seq,
                sysid,
                compid,
                msgid,
                payload,
                checksum,
                signature,
            };
            assert(self.spec_parse(ibuf@) == Some((total_n as int, final_v.deep_view())));
            Ok((total_n, final_v))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MavlinkV2Msg<'i>> for MavlinkV2MsgFmt {
        fn serialize_into(&self, v: &MavlinkV2Msg<'i>, obuf: &mut Output) {
            broadcast use vest_lib::core::exec::output::outbuf_lemmas;
            reveal(<MavlinkV2MsgFmt as SpecSerializer>::spec_serialize);
            reveal(<MavlinkV2MsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkV2Msg as DeepView>::deep_view);
            reveal(MavlinkV2MsgSpec::into_structural);
            let ghost old_obuf = obuf@;

            let MavlinkV2Msg {
                len,
                incompat_flags,
                compat_flags,
                seq,
                sysid,
                compid,
                msgid,
                payload,
                checksum,
                signature,
            } = v;
            proof {
                incompat_flags.lemma_deep_view();
                msgid.lemma_deep_view();
            }

            U8.serialize_into(len, obuf);
            IncompatFlagsFmt.serialize_into(incompat_flags, obuf);
            U8.serialize_into(compat_flags, obuf);
            U8.serialize_into(seq, obuf);
            U8.serialize_into(sysid, obuf);
            U8.serialize_into(compid, obuf);
            MessageIdsV2Fmt.serialize_into(msgid, obuf);
            Varied (*len).serialize_into(* payload, obuf);
            U16Le.serialize_into(checksum, obuf);
            MavlinkV2MsgSignatureFmt {
                incompat_flags: * incompat_flags
            }
            .serialize_into(signature, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MavlinkV2Msg<'i>> for MavlinkV2MsgFmt {
        fn prepare(&self, v: &MavlinkV2Msg<'i>) -> Result<usize, PreSerializeError> {
            reveal(<MavlinkV2MsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkV2Msg as DeepView>::deep_view);
            reveal(MavlinkV2MsgSpec::into_structural);
            let MavlinkV2Msg {
                len,
                incompat_flags,
                compat_flags,
                seq,
                sysid,
                compid,
                msgid,
                payload,
                checksum,
                signature,
            } = v;
            proof {
                incompat_flags.lemma_deep_view();
                msgid.lemma_deep_view();
            }

            let l1 = (U8).prepare (len) ?;
            let l2 = (Named ("incompat_flags", IncompatFlagsFmt)).prepare (incompat_flags) ?;
            let l3 = (U8).prepare (compat_flags) ?;
            let l4 = (U8).prepare (seq) ?;
            let l5 = {
                if ! (* sysid >= 1) {
                    Err (PreSerializeError::not_compliant (ComplianceErrorKind::PredicateFailed))
                }
                else {
                    (U8).prepare (sysid)
                }
            }
            ?;
            let l6 = {
                if ! (* compid >= 1) {
                    Err (PreSerializeError::not_compliant (ComplianceErrorKind::PredicateFailed))
                }
                else {
                    (U8).prepare (compid)
                }
            }
            ?;
            let l7 = (Named ("message_ids_v2", MessageIdsV2Fmt)).prepare (msgid) ?;
            let l8 = (Varied (*len)).prepare (payload) ?;
            let l9 = (U16Le).prepare (checksum) ?;
            let l10 = (Named ("mavlink_v2_msg_signature", MavlinkV2MsgSignatureFmt {
                incompat_flags: * incompat_flags
            }
            )).prepare (signature) ?;
            let total_len = l1.checked_add (l2).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l3).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l4).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l5).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l6).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l7).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l8).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l9).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l10).ok_or (PreSerializeError::length_too_large()) ?;
            Ok(total_len)
        }
    }



    impl<'i> Parser<&'i [u8]> for MavlinkV1MsgFmt {
        type PT = MavlinkV1Msg<'i>;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            broadcast use vest_lib::core::spec::SafeParser::lemma_parse_safe;
            broadcast use vest_lib::core::spec::SoundParser::lemma_parse_sound_value;

            reveal(<MavlinkV1MsgFmt as SpecParser>::spec_parse);
            reveal(<MavlinkV1Msg as DeepView>::deep_view);
            reveal(MavlinkV1MsgSpec::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n1, len) = (U8).parse (& rest) ?;
            let rest = rest.skip(n1);
            let (n2, seq) = (U8).parse (& rest) ?;
            let rest = rest.skip(n2);
            let (n3, sysid) = (U8).parse (& rest) ?;
            if !(sysid >= 1) {
                return Err(ParseError::predicate_failed());
            }
            let rest = rest.skip(n3);
            let (n4, compid) = (U8).parse (& rest) ?;
            if !(compid >= 1) {
                return Err(ParseError::predicate_failed());
            }
            let rest = rest.skip(n4);
            let (n5, msgid) = (Named ("message_ids_v1", MessageIdsV1Fmt)).parse (& rest) ?;
            proof {
                msgid.lemma_deep_view();
            }
            let rest = rest.skip(n5);
            let (n6, payload) = (Varied (len)).parse (& rest) ?;
            let rest = rest.skip(n6);
            let (n7, checksum) = (U16Le).parse (& rest) ?;
            let rest = rest.skip(n7);
            let total_n = n1 + n2 + n3 + n4 + n5 + n6 + n7;
            let final_v = MavlinkV1Msg {
                len,
                seq,
                sysid,
                compid,
                msgid,
                payload,
                checksum,
            };
            assert(self.spec_parse(ibuf@) == Some((total_n as int, final_v.deep_view())));
            Ok((total_n, final_v))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MavlinkV1Msg<'i>> for MavlinkV1MsgFmt {
        fn serialize_into(&self, v: &MavlinkV1Msg<'i>, obuf: &mut Output) {
            broadcast use vest_lib::core::exec::output::outbuf_lemmas;
            reveal(<MavlinkV1MsgFmt as SpecSerializer>::spec_serialize);
            reveal(<MavlinkV1MsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkV1Msg as DeepView>::deep_view);
            reveal(MavlinkV1MsgSpec::into_structural);
            let ghost old_obuf = obuf@;

            let MavlinkV1Msg {
                len,
                seq,
                sysid,
                compid,
                msgid,
                payload,
                checksum,
            } = v;
            proof {
                msgid.lemma_deep_view();
            }

            U8.serialize_into(len, obuf);
            U8.serialize_into(seq, obuf);
            U8.serialize_into(sysid, obuf);
            U8.serialize_into(compid, obuf);
            MessageIdsV1Fmt.serialize_into(msgid, obuf);
            Varied (*len).serialize_into(* payload, obuf);
            U16Le.serialize_into(checksum, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MavlinkV1Msg<'i>> for MavlinkV1MsgFmt {
        fn prepare(&self, v: &MavlinkV1Msg<'i>) -> Result<usize, PreSerializeError> {
            reveal(<MavlinkV1MsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkV1Msg as DeepView>::deep_view);
            reveal(MavlinkV1MsgSpec::into_structural);
            let MavlinkV1Msg {
                len,
                seq,
                sysid,
                compid,
                msgid,
                payload,
                checksum,
            } = v;
            proof {
                msgid.lemma_deep_view();
            }

            let l1 = (U8).prepare (len) ?;
            let l2 = (U8).prepare (seq) ?;
            let l3 = {
                if ! (* sysid >= 1) {
                    Err (PreSerializeError::not_compliant (ComplianceErrorKind::PredicateFailed))
                }
                else {
                    (U8).prepare (sysid)
                }
            }
            ?;
            let l4 = {
                if ! (* compid >= 1) {
                    Err (PreSerializeError::not_compliant (ComplianceErrorKind::PredicateFailed))
                }
                else {
                    (U8).prepare (compid)
                }
            }
            ?;
            let l5 = (Named ("message_ids_v1", MessageIdsV1Fmt)).prepare (msgid) ?;
            let l6 = (Varied (*len)).prepare (payload) ?;
            let l7 = (U16Le).prepare (checksum) ?;
            let total_len = l1.checked_add (l2).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l3).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l4).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l5).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l6).ok_or (PreSerializeError::length_too_large()) ?.checked_add (l7).ok_or (PreSerializeError::length_too_large()) ?;
            Ok(total_len)
        }
    }



    impl<'i> Parser<&'i [u8]> for ProtocolMagicFmt {
        type PT = ProtocolMagic;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<ProtocolMagicFmt as SpecParser>::spec_parse);
            reveal(<ProtocolMagic as DeepView>::deep_view);
            reveal(ProtocolMagic::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n, v) = U8.parse(&rest)?;
            let enum_val = match v {
                254 => ProtocolMagic::MavLink1,
                253 => ProtocolMagic::MavLink2,
                _ => return Err (ParseError::invalid_tag()),
            };
            assert (self.spec_parse (ibuf @) == Some ((n as int, enum_val.deep_view()))) ;
            Ok((n, enum_val))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, ProtocolMagic> for ProtocolMagicFmt {
        fn serialize_into(&self, v: &ProtocolMagic, obuf: &mut Output) {
            reveal(<ProtocolMagicFmt as SpecSerializer>::spec_serialize);
            reveal(<ProtocolMagicFmt as SpecByteLen>::byte_len);
            reveal(<ProtocolMagic as DeepView>::deep_view);
            reveal(ProtocolMagic::into_structural);
            let ghost old_obuf = obuf@;

            let tag = match *v {
                ProtocolMagic::MavLink1 => 254,
                ProtocolMagic::MavLink2 => 253,
            };
            U8.serialize_into(&tag, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<ProtocolMagic> for ProtocolMagicFmt {
        fn prepare(&self, v: &ProtocolMagic) -> Result<usize, PreSerializeError> {
            reveal(<ProtocolMagicFmt as SpecByteLen>::byte_len);
            reveal(<ProtocolMagic as DeepView>::deep_view);
            reveal(ProtocolMagic::into_structural);
            let tag = match *v {
                ProtocolMagic::MavLink1 => 254,
                ProtocolMagic::MavLink2 => 253,
                _ => return Err (PreSerializeError::not_compliant (ComplianceErrorKind::InvalidTag)),
            };
            U8.prepare(&tag)
        }
    }



    impl<'i> Parser<&'i [u8]> for IncompatFlagsFmt {
        type PT = IncompatFlags;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<IncompatFlagsFmt as SpecParser>::spec_parse);
            reveal(<IncompatFlags as DeepView>::deep_view);
            reveal(IncompatFlags::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n, v) = U8.parse(&rest)?;
            let enum_val = match v {
                1 => IncompatFlags::Signed,
                x => IncompatFlags::Unknown (x),
            };
            assert (self.spec_parse (ibuf @) == Some ((n as int, enum_val.deep_view()))) ;
            Ok((n, enum_val))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, IncompatFlags> for IncompatFlagsFmt {
        fn serialize_into(&self, v: &IncompatFlags, obuf: &mut Output) {
            reveal(<IncompatFlagsFmt as SpecSerializer>::spec_serialize);
            reveal(<IncompatFlagsFmt as SpecByteLen>::byte_len);
            reveal(<IncompatFlags as DeepView>::deep_view);
            reveal(IncompatFlags::into_structural);
            let ghost old_obuf = obuf@;

            let tag = match *v {
                IncompatFlags::Signed => 1,
                IncompatFlags::Unknown (x) => x,
            };
            U8.serialize_into(&tag, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<IncompatFlags> for IncompatFlagsFmt {
        fn prepare(&self, v: &IncompatFlags) -> Result<usize, PreSerializeError> {
            reveal(<IncompatFlagsFmt as SpecByteLen>::byte_len);
            reveal(<IncompatFlags as DeepView>::deep_view);
            reveal(IncompatFlags::into_structural);
            let tag = match *v {
                IncompatFlags::Signed => 1,
                IncompatFlags::Unknown (x) if x != 1 => x, _ => return Err (PreSerializeError::not_compliant (ComplianceErrorKind::InvalidTag)),
            };
            U8.prepare(&tag)
        }
    }



    impl<'i> Parser<&'i [u8]> for MessageIdsV2Fmt {
        type PT = MessageIdsV2;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<MessageIdsV2Fmt as SpecParser>::spec_parse);
            reveal(<MessageIdsV2 as DeepView>::deep_view);
            reveal(MessageIdsV2::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n, v) = U24Le.parse(&rest)?;
            let enum_val = match v {
                75 => MessageIdsV2::CommandInt,
                76 => MessageIdsV2::CommandLong,
                77 => MessageIdsV2::CommandAck,
                8388608 => MessageIdsV2::Reserved,
                x => MessageIdsV2::Unknown (x),
            };
            assert (self.spec_parse (ibuf @) == Some ((n as int, enum_val.deep_view()))) ;
            Ok((n, enum_val))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MessageIdsV2> for MessageIdsV2Fmt {
        fn serialize_into(&self, v: &MessageIdsV2, obuf: &mut Output) {
            reveal(<MessageIdsV2Fmt as SpecSerializer>::spec_serialize);
            reveal(<MessageIdsV2Fmt as SpecByteLen>::byte_len);
            reveal(<MessageIdsV2 as DeepView>::deep_view);
            reveal(MessageIdsV2::into_structural);
            let ghost old_obuf = obuf@;

            let tag = match *v {
                MessageIdsV2::CommandInt => 75,
                MessageIdsV2::CommandLong => 76,
                MessageIdsV2::CommandAck => 77,
                MessageIdsV2::Reserved => 8388608,
                MessageIdsV2::Unknown (x) => x,
            };
            U24Le.serialize_into(&tag, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MessageIdsV2> for MessageIdsV2Fmt {
        fn prepare(&self, v: &MessageIdsV2) -> Result<usize, PreSerializeError> {
            reveal(<MessageIdsV2Fmt as SpecByteLen>::byte_len);
            reveal(<MessageIdsV2 as DeepView>::deep_view);
            reveal(MessageIdsV2::into_structural);
            let tag = match *v {
                MessageIdsV2::CommandInt => 75,
                MessageIdsV2::CommandLong => 76,
                MessageIdsV2::CommandAck => 77,
                MessageIdsV2::Reserved => 8388608,
                MessageIdsV2::Unknown (x) if x != 75 && x != 76 && x != 77 && x != 8388608 => x, _ => return Err (PreSerializeError::not_compliant (ComplianceErrorKind::InvalidTag)),
            };
            U24Le.prepare(&tag)
        }
    }



    impl<'i> Parser<&'i [u8]> for MessageIdsV1Fmt {
        type PT = MessageIdsV1;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<MessageIdsV1Fmt as SpecParser>::spec_parse);
            reveal(<MessageIdsV1 as DeepView>::deep_view);
            reveal(MessageIdsV1::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n, v) = U8.parse(&rest)?;
            let enum_val = match v {
                75 => MessageIdsV1::CommandInt,
                76 => MessageIdsV1::CommandLong,
                77 => MessageIdsV1::CommandAck,
                x => MessageIdsV1::Unknown (x),
            };
            assert (self.spec_parse (ibuf @) == Some ((n as int, enum_val.deep_view()))) ;
            Ok((n, enum_val))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MessageIdsV1> for MessageIdsV1Fmt {
        fn serialize_into(&self, v: &MessageIdsV1, obuf: &mut Output) {
            reveal(<MessageIdsV1Fmt as SpecSerializer>::spec_serialize);
            reveal(<MessageIdsV1Fmt as SpecByteLen>::byte_len);
            reveal(<MessageIdsV1 as DeepView>::deep_view);
            reveal(MessageIdsV1::into_structural);
            let ghost old_obuf = obuf@;

            let tag = match *v {
                MessageIdsV1::CommandInt => 75,
                MessageIdsV1::CommandLong => 76,
                MessageIdsV1::CommandAck => 77,
                MessageIdsV1::Unknown (x) => x,
            };
            U8.serialize_into(&tag, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MessageIdsV1> for MessageIdsV1Fmt {
        fn prepare(&self, v: &MessageIdsV1) -> Result<usize, PreSerializeError> {
            reveal(<MessageIdsV1Fmt as SpecByteLen>::byte_len);
            reveal(<MessageIdsV1 as DeepView>::deep_view);
            reveal(MessageIdsV1::into_structural);
            let tag = match *v {
                MessageIdsV1::CommandInt => 75,
                MessageIdsV1::CommandLong => 76,
                MessageIdsV1::CommandAck => 77,
                MessageIdsV1::Unknown (x) if x != 75 && x != 76 && x != 77 => x, _ => return Err (PreSerializeError::not_compliant (ComplianceErrorKind::InvalidTag)),
            };
            U8.prepare(&tag)
        }
    }



    impl<'i> Parser<&'i [u8]> for MavCmdFmt {
        type PT = MavCmd;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<MavCmdFmt as SpecParser>::spec_parse);
            reveal(<MavCmd as DeepView>::deep_view);
            reveal(MavCmd::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            let (n, v) = U16Le.parse(&rest)?;
            let enum_val = match v {
                42650 => MavCmd::FlashBootloader,
                x => MavCmd::Unknown (x),
            };
            assert (self.spec_parse (ibuf @) == Some ((n as int, enum_val.deep_view()))) ;
            Ok((n, enum_val))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MavCmd> for MavCmdFmt {
        fn serialize_into(&self, v: &MavCmd, obuf: &mut Output) {
            reveal(<MavCmdFmt as SpecSerializer>::spec_serialize);
            reveal(<MavCmdFmt as SpecByteLen>::byte_len);
            reveal(<MavCmd as DeepView>::deep_view);
            reveal(MavCmd::into_structural);
            let ghost old_obuf = obuf@;

            let tag = match *v {
                MavCmd::FlashBootloader => 42650,
                MavCmd::Unknown (x) => x,
            };
            U16Le.serialize_into(&tag, obuf);

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MavCmd> for MavCmdFmt {
        fn prepare(&self, v: &MavCmd) -> Result<usize, PreSerializeError> {
            reveal(<MavCmdFmt as SpecByteLen>::byte_len);
            reveal(<MavCmd as DeepView>::deep_view);
            reveal(MavCmd::into_structural);
            let tag = match *v {
                MavCmd::FlashBootloader => 42650,
                MavCmd::Unknown (x) if x != 42650 => x, _ => return Err (PreSerializeError::not_compliant (ComplianceErrorKind::InvalidTag)),
            };
            U16Le.prepare(&tag)
        }
    }



    impl<'i> Parser<&'i [u8]> for MavlinkMsgMsgFmt {
        type PT = MavlinkMsgMsg<'i>;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<MavlinkMsgMsgFmt as SpecParser>::spec_parse);
            reveal(<MavlinkMsgMsg as DeepView>::deep_view);
            reveal(MavlinkMsgMsgSpec::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            proof {
                use_type_invariant(self);
                self.magic.lemma_deep_view();
            }

            proof {
                self.magic.lemma_deep_view();
            }

            let (n, v) = match self.magic {
                ProtocolMagic::MavLink1 => {
                    let (n,
                    v) = (Named ("mavlink_v1_msg",
                    MavlinkV1MsgFmt)).parse (& rest) ?;
                    (n,
                    MavlinkMsgMsg::MavLink1 (v))
                }
                ,
                ProtocolMagic::MavLink2 => {
                    let (n,
                    v) = (Named ("mavlink_v2_msg",
                    MavlinkV2MsgFmt)).parse (& rest) ?;
                    (n,
                    MavlinkMsgMsg::MavLink2 (v))
                }
                ,
            };
            assert(self.spec_parse(ibuf@) == Some((n as int, v.deep_view())));
            Ok((n, v))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MavlinkMsgMsg<'i>> for MavlinkMsgMsgFmt {
        fn serialize_into(&self, v: &MavlinkMsgMsg<'i>, obuf: &mut Output) {
            reveal(<MavlinkMsgMsgFmt as SpecSerializer>::spec_serialize);
            reveal(<MavlinkMsgMsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkMsgMsg as DeepView>::deep_view);
            reveal(MavlinkMsgMsgSpec::into_structural);
            proof {
                use_type_invariant(self);
                self.magic.lemma_deep_view();
            }

            let ghost old_obuf = obuf@;

            proof {
                self.magic.lemma_deep_view();
            }

            match (self.magic, v) {
                (ProtocolMagic::MavLink1, MavlinkMsgMsg::MavLink1 (v)) => {
                    (MavlinkV1MsgFmt).serialize_into (v,
                    obuf) ;
                }
                ,
                (ProtocolMagic::MavLink2, MavlinkMsgMsg::MavLink2 (v)) => {
                    (MavlinkV2MsgFmt).serialize_into (v,
                    obuf) ;
                }
                ,
                _ => {},
            }

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MavlinkMsgMsg<'i>> for MavlinkMsgMsgFmt {
        fn prepare(&self, v: &MavlinkMsgMsg<'i>) -> Result<usize, PreSerializeError> {
            reveal(<MavlinkMsgMsgFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkMsgMsg as DeepView>::deep_view);
            reveal(MavlinkMsgMsgSpec::into_structural);
            proof {
                use_type_invariant(self);
                self.magic.lemma_deep_view();
            }

            proof {
                self.magic.lemma_deep_view();
            }

            match (self.magic, v) {
                (ProtocolMagic::MavLink1, MavlinkMsgMsg::MavLink1 (v)) => (Named ("mavlink_v1_msg", MavlinkV1MsgFmt)).prepare (v),
                (ProtocolMagic::MavLink2, MavlinkMsgMsg::MavLink2 (v)) => (Named ("mavlink_v2_msg", MavlinkV2MsgFmt)).prepare (v),
                 _ => Err(PreSerializeError::not_compliant(ComplianceErrorKind::InvalidTag)),
            }
        }
    }



    impl<'i> Parser<&'i [u8]> for MavlinkV2MsgSignatureFmt {
        type PT = MavlinkV2MsgSignature<'i>;

        fn parse(&self, ibuf: &&'i [u8]) -> PResult<Self::PT> {
            reveal(<MavlinkV2MsgSignatureFmt as SpecParser>::spec_parse);
            reveal(<MavlinkV2MsgSignature as DeepView>::deep_view);
            reveal(MavlinkV2MsgSignatureSpec::from_structural);
            let _ = ibuf.len();
            let rest = *ibuf;

            proof {
                use_type_invariant(self);
                self.incompat_flags.lemma_deep_view();
            }

            proof {
                self.incompat_flags.lemma_deep_view();
            }

            let (n, v) = match self.incompat_flags {
                IncompatFlags::Signed => {
                    let (n,
                    v) = (Fixed::< 13 >).parse (& rest) ?;
                    (n,
                    MavlinkV2MsgSignature::Signed (v))
                }
                ,
                _ => {
                    let (n,
                    v) = (Fixed::< 0 >).parse (& rest) ?;
                    (n,
                    MavlinkV2MsgSignature::Default (v))
                }
                ,
            };
            assert(self.spec_parse(ibuf@) == Some((n as int, v.deep_view())));
            Ok((n, v))
        }
    }

    impl<Output: OutputBuf, 'i> Serializer<Output, MavlinkV2MsgSignature<'i>> for MavlinkV2MsgSignatureFmt {
        fn serialize_into(&self, v: &MavlinkV2MsgSignature<'i>, obuf: &mut Output) {
            reveal(<MavlinkV2MsgSignatureFmt as SpecSerializer>::spec_serialize);
            reveal(<MavlinkV2MsgSignatureFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkV2MsgSignature as DeepView>::deep_view);
            reveal(MavlinkV2MsgSignatureSpec::into_structural);
            proof {
                use_type_invariant(self);
                self.incompat_flags.lemma_deep_view();
            }

            let ghost old_obuf = obuf@;

            proof {
                self.incompat_flags.lemma_deep_view();
            }

            match (self.incompat_flags, v) {
                (IncompatFlags::Signed, MavlinkV2MsgSignature::Signed (v)) => {
                    (Fixed::< 13 >).serialize_into (*v,
                    obuf) ;
                }
                ,
                (_, MavlinkV2MsgSignature::Default (v)) => {
                    (Fixed::< 0 >).serialize_into (*v,
                    obuf) ;
                }
                ,
                _ => {},
            }

            assert(obuf@ == old_obuf + self.spec_serialize(v.deep_view()));
        }
    }

    impl<'i> Prepare<MavlinkV2MsgSignature<'i>> for MavlinkV2MsgSignatureFmt {
        fn prepare(&self, v: &MavlinkV2MsgSignature<'i>) -> Result<usize, PreSerializeError> {
            reveal(<MavlinkV2MsgSignatureFmt as SpecByteLen>::byte_len);
            reveal(<MavlinkV2MsgSignature as DeepView>::deep_view);
            reveal(MavlinkV2MsgSignatureSpec::into_structural);
            proof {
                use_type_invariant(self);
                self.incompat_flags.lemma_deep_view();
            }

            proof {
                self.incompat_flags.lemma_deep_view();
            }

            match (self.incompat_flags, v) {
                (IncompatFlags::Signed, MavlinkV2MsgSignature::Signed (v)) => (Fixed::< 13 >).prepare (v),
                (IncompatFlags::Unknown (x), MavlinkV2MsgSignature::Default (v)) if x != 1 => (Fixed::< 0 >).prepare (v),
                 _ => Err(PreSerializeError::not_compliant(ComplianceErrorKind::InvalidTag)),
            }
        }
    }

}
}
