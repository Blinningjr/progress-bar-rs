use std::io::Write;

/**
 * Progress bar stuct that keeps track of the state and all the settings.
 */
pub struct ProgressBar {
    name: String,
    progress: u32,
    count: u32,
    pub length: u32,            // Length of the bar
    pub start_char: char,       // Start char of the bar
    pub end_char: char,         // End char of the bar
    pub progress_char: char,    // Char that shows completion
    pub tip_char: char,         // Char that is at the tip of the completion part
    pub empty_char: char,       // Char representing the empty parts of the bar
}


impl ProgressBar {
    /**
     * Creates a standard progress bar
     */
    pub fn new(name: String, count: u32) -> ProgressBar {
        ProgressBar {
            name: name,
            progress: 0,
            count: count,
            length: 100,
            start_char: '[',
            end_char: ']',
            progress_char: '=',
            tip_char: '>',
            empty_char: '-',
        }
    }


    /**
     * Increases the progress of the bar by one and updates the bar.
     */
    pub fn increase(&mut self) {
        self.progress += 1;
        let num_progress: u32 = (self.progress * self.length)/self.count;
        let mut output = format!("{} {}", self.name, self.start_char);
        for i in 0..self.length {
            if i < num_progress {
                output = format!("{}{}", output, self.progress_char);
            } else if i == num_progress {
                output = format!("{}{}", output, self.tip_char);
            } else {
                output = format!("{}{}", output, self.empty_char);
            }
        }
        output = format!("{}{} {}/{} ", output, self.end_char, self.progress, self.count);

        print!("\r{}", output);
        std::io::stdout().flush().unwrap();
    }


    /**
     * Prints a finished message.
     */
    pub fn finished(&mut self) {
        print!("\nTask {:?} is complete\n", self.name);
        std::io::stdout().flush().unwrap(); 
    }
}

