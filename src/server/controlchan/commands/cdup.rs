//! The RFC 959 Change To Parent Directory (`CDUP`) command
//
// This command is a special case of CWD, and is included to
// simplify the implementation of programs for transferring
// directory trees between operating systems having different
// syntaxes for naming the parent directory.  The reply codes
// shall be identical to the reply codes of CWD.

use crate::{
    auth::UserDetail,
    server::controlchan::{
        error::ControlChanError,
        handler::{CommandContext, CommandHandler},
        Reply, ReplyCode,
    },
    storage::{Metadata, StorageBackend},
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Cdup;

#[async_trait]
impl<Storage, User> CommandHandler<Storage, User> for Cdup
where
    User: UserDetail + 'static,
    Storage: StorageBackend<User> + 'static,
    Storage::Metadata: Metadata,
{
    async fn handle(&self, args: CommandContext<Storage, User>) -> Result<Reply, ControlChanError> {
        let mut session = args.session.lock().await;
        session.cwd.pop();
        Ok(Reply::new(ReplyCode::FileActionOkay, "OK"))
    }
}
