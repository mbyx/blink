/// Represents the priority in scheduling a task. This can be a fine grained
/// value or one of the default priorities (Low, Medium, High).
///
/// The numbers assigned to the default priorities are such that there will
/// always be a priority between each default as well as above and below it.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
#[repr(u8)]
pub enum TaskPriority {
    Low = 64,
    Normal = 128,
    High = 192,
    Custom(u8),
}

impl From<u8> for TaskPriority {
    fn from(value: u8) -> Self {
        match value {
            64 => Self::Low,
            128 => Self::Normal,
            192 => Self::High,
            other => Self::Custom(other),
        }
    }
}

impl From<TaskPriority> for u8 {
    fn from(value: TaskPriority) -> u8 {
        match value {
            TaskPriority::Low => 64,
            TaskPriority::Normal => 128,
            TaskPriority::High => 192,
            TaskPriority::Custom(other) => other,
        }
    }
}

impl Ord for TaskPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (p, q) = match (*self, *other) {
            (Self::Custom(p), Self::Custom(q)) => (p, q),
            (any, Self::Custom(priority)) => (any.into(), priority),
            (Self::Custom(priority), any) => (priority, any.into()),
            (any, other) => (any.into(), other.into()),
        };
        p.cmp(&q)
    }
}
