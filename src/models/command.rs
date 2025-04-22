use super::task::TaskId;
use super::status::Status;

#[derive(Debug)]
pub enum Command {
    Add(String),
    Update(TaskId, String),
    Delete(TaskId),
    MarkInProgress(TaskId),
    MarkDone(TaskId),
    List(Option<Status>)
}