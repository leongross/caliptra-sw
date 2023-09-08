// Licensed under the Apache-2.0 license

use crate::{Drivers};
use caliptra_common::mailbox_api::{MailboxResp, IncrementPcrResetCounter};
use caliptra_drivers::{CaliptraError, CaliptraResult};
use zerocopy::FromBytes;

pub struct IncrementPcrResetCounterCmd;
impl IncrementPcrResetCounterCmd {
    pub(crate) fn execute(drivers: &Drivers, cmd_args: &[u8]) -> CaliptraResult<MailboxResp> {
        if let Some(cmd) = IncrementPcrResetCounter::read_from(cmd_args) {
            // cmd.
            return Ok(MailboxResp::default());
        } else {
            return Err(CaliptraError::RUNTIME_INSUFFICIENT_MEMORY);
        }
        
        Ok(MailboxResp::default())
    }
}