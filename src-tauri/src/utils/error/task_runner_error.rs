/// 日志配置错误
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskRunnerError {
    #[error("Task group \"{0}\" already exists")]
    GroupExists(String),
    #[error("Dependency group \"{0}\" not found for group \"{1}\"")]
    DependencyNotFound(String, String),
    #[error("Circular dependency detected when adding group \"{0}\"")]
    CircularDependency(String),
    #[error("Cannot remove the default group")]
    CannotRemoveDefault,
    #[error("Cannot remove group \"{0}\", it is depended upon by group \"{1}\"")]
    GroupDependedOn(String, String),
    #[error("Cannot add tasks while running")]
    AddingTaskWhileRunning,
    #[error("Task with id \"{0}\" already exists")]
    TaskExists(String),
    #[error("No task group \"{0}\" found")]
    GroupNotFound(String),
    #[error("Cannot remove tasks while running")]
    RemovingTaskWhileRunning,
    #[error("TaskRunner is already running")]
    AlreadyRunning,
    #[error("TaskRunner is not running")]
    NotRunning,
    #[error("Task \"{0}\" panicked")]
    TaskPanicError(String),
}
