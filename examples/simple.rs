extern crate progress_bar_rs;

use progress_bar_rs::ProgressBar;

use std::thread;
use std::time::Duration;


fn main() {
    // Amount of steps in the task
    let count = 50;

    // Create a progress bar struct to keep track of the progress
    let mut progress_bar = ProgressBar::new("example task".to_string(), count);

    // Set some optional settings
    progress_bar.length = 100; // Option: Set the length of the bar (default: 100)
    progress_bar.start_char = '['; // Option: Set the start char of the bar  (default: '[')
    progress_bar.end_char = ']'; // Option: Set the end char of the bar  (default: ']')
    progress_bar.progress_char = '='; // Option: Set the progress char that shows the amount completed(default: '=')
    progress_bar.tip_char = '>'; // Option: Set the char at end of the completed part  (default: '>')
    progress_bar.empty_char = '-'; // Option: Set the empty char that shows the amount left  (default: '-')

    // Start doing the steps of the task
    for _ in 0..count {
        // Increases the number of step completed by 1
        progress_bar.increase();

        // Do one of the steps in the task
        thread::sleep(Duration::from_millis(100));
    }

    // Print that the task is completed
    progress_bar.finished();
}

