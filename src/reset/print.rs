use crate::reset::Reset;

/// Prints a Reset
/// 
/// # Examples
///
/// ```
/// let mut r = reset::new();
/// r.print;
/// ```
impl Reset {
    pub fn print(&self) -> String {
        let mut reset_text: String = "a".to_owned();
        let appender: &str = "b";
        reset_text.push_str(appender);
        println!("{}",reset_text);
        reset_text
    }
}


