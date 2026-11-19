#![no_std]
extern crate alloc;

mod mavlink;
pub use mavlink::*;

use vest_lib::core::exec::parser::{Parser, PResult};
#[cfg(verus_keep_ghost)]
use vest_lib::core::exec::parser::parse_matches_spec;
#[cfg(verus_keep_ghost)]
use vest_lib::core::spec::SpecParser;
use vstd::prelude::*;

verus! {
    pub open spec fn spec_mavlink_msg() -> MavlinkMsgFmt { MavlinkMsgFmt }

    pub fn parse_mavlink_msg(input: &[u8]) -> (res: PResult<MavlinkMsg<'_>>)
        ensures parse_matches_spec(res, MavlinkMsgFmt.spec_parse(input@)),
    {
        MavlinkMsgFmt.parse(&input)
    }
}
