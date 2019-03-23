use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Timer {
    /// Turns the colored output on
    ///
    /// Affected messages: Execution, Finish
    ///
    #[structopt(name = "color", short, long)]
    pub colored: bool,

    /// Sets the timer duration in seconds
    ///
    /// Only works on positive integers
    #[structopt(name = "duration", short, long, default_value = "0")]
    pub duration: u64,

    /// Sets the frequency in seconds
    ///
    /// Only works on positive integers
    #[structopt(name = "frequency", short, long, default_value = "1")]
    pub frequency: u64,

    /// Sets the indicator of the timer's progress (Numerical, Bar)
    ///
    /// Numerical indicator is outputted line-by-line
    ///
    /// Bar indicator is constantly filled on a single line
    #[structopt(name = "indicator", short, long, default_value = "numerical")]
    pub indicator: String,

    /// Turns the logger on
    ///
    /// This option enables logging, which can be helpful to track application's usage
    ///
    #[structopt(name = "logger", short, long)]
    pub logger: bool,

    /// Turns the sound on
    ///
    /// This option enables sound, which can be useful for tracking the timer
    ///
    #[structopt(name = "sound", short, long)]
    pub sound: bool,

    /// Sets the time zone (Local, UTC)
    ///
    /// Logs only save time using the Local time zone
    #[structopt(name = "timezone", short, long, default_value = "local")]
    pub timezone: String,
}

impl Timer {
    pub fn total_duration(&self) -> u64 {
        self.duration * self.frequency
    }
}
