impl Display for FileState {
    fn fmt(&self, f:
        &mut fmt::Formatter,
    ) -> fmt::Result { // 1
        match &self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f:
        &mut fmt:: Formatter,
    ) -> fmt::Result { // 1
        write!(f, "<{} ({})",
            self.name, self.state) // 2
    }
}