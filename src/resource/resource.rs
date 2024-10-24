/// Represents a possible resource that could be assigned to a task.
///
/// This resource can be anything IO bound, such as a Pin, File, or
/// anything else.
pub enum TaskResource {
    /// A pin is a resource represented by its pin number.
    Pin(i32),
}
