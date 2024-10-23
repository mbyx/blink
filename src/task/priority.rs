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
        match (self, other) {
            (&Self::Custom(p), &Self::Custom(q)) => p.cmp(&q),
            (&any, &Self::Custom(priority)) => <Self as Into<u8>>::into(any).cmp(&priority),
            (&Self::Custom(priority), &any) => priority.cmp(&any.into()),
            (&any, &other) => <Self as Into<u8>>::into(any).cmp(&other.into()),
        }
    }
}
