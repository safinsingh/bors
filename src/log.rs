macro_rules! success {
    ($($arg:tt)*) => ({
        if *crate::LOG {
            println!("{} {}", "[SCSS]".green().bold(), format_args!($($arg)*))
        }
    })
}

macro_rules! info {
    ($($arg:tt)*) => ({
        if *crate::LOG {
            println!("{} {}", "[INFO]".cyan().bold(), format_args!($($arg)*))
        }
    })
}

macro_rules! warn {
    ($($arg:tt)*) => ({
        if *crate::LOG {
            println!("{} {}", "[WARN]".yellow().bold(), format_args!($($arg)*))
        }
    })
}
