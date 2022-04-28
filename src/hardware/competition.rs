// Interfaces to get the current competition status


/// An Enum that specifies the competitions status
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CompetitionStatus {
    Disconnected,
    Disabled,
    DriverControl,
    Autonomous,
}

impl CompetitionStatus {
    /// Gets the current competition status
    pub fn get_competition_status()  -> CompetitionStatus{
        // Get the competition status
        let status = unsafe { vexv5rt::vexCompetitionStatus() };

        if status & (1 << 2) == 0 {
            // Checks if the robot is disconnected
            CompetitionStatus::Disconnected
        } else if status & (1 << 0) != 0 {
            // Checks if the robot is disabled
            CompetitionStatus::Disabled
        } else if status & (1 << 1) != 0 {
            // Checks if the robot is in autonomous
            CompetitionStatus::Autonomous
        } else {
            // If none of the above, then the robot is in driver control
            CompetitionStatus::DriverControl
        }
    }
}