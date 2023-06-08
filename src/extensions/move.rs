//! IMAP - MOVE Extension

use abnf_core::streaming::sp;
/// Re-export everything from imap-types.
pub use imap_types::extensions::r#move::*;
use nom::{bytes::streaming::tag_no_case, sequence::tuple};

use crate::{codec::IMAPResult, command::CommandBody, mailbox::mailbox, sequence::sequence_set};

/// ```abnf
/// move = "MOVE" SP sequence-set SP mailbox
/// ```
pub fn r#move(input: &[u8]) -> IMAPResult<&[u8], CommandBody> {
    let mut parser = tuple((tag_no_case(b"MOVE"), sp, sequence_set, sp, mailbox));

    let (remaining, (_, _, sequence_set, _, mailbox)) = parser(input)?;

    Ok((
        remaining,
        CommandBody::Move {
            sequence_set,
            mailbox,
            uid: false,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        command::{Command, CommandBody},
        testing::kat_inverse_command,
    };

    #[test]
    fn test_kat_inverse_command_move() {
        kat_inverse_command(&[
            (
                b"A MOVE 1 INBOX\r\n".as_ref(),
                b"".as_ref(),
                Command::new("A", CommandBody::r#move("1", "inBox", false).unwrap()).unwrap(),
            ),
            (
                b"A UID MOVE 1 INBOX\r\n?",
                b"?",
                Command::new("A", CommandBody::r#move("1", "inBox", true).unwrap()).unwrap(),
            ),
            (
                b"A MOVE 1:* test\r\n??",
                b"??",
                Command::new("A", CommandBody::r#move("1:*", "test", false).unwrap()).unwrap(),
            ),
        ]);
    }
}
