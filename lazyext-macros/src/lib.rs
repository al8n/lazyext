/// `#[cfg(windows)]`
#[macro_export]
macro_rules! cfg_windows {
    ($($item:item)*) => {
        $(
            #[cfg(windows)]
            #[cfg_attr(docsrs, doc(cfg(windows)))]
            $item
        )*
    }
}

/// `#[cfg(unix)]`
#[macro_export]
macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(unix)]
            #[cfg_attr(docsrs, doc(cfg(unix)))]
            $item
        )*
    }
}

/// `#[cfg(test)]`
#[macro_export]
macro_rules! cfg_test {
    ($($item:item)*) => {
        $(
            #[cfg(test)]
            #[cfg_attr(docsrs, doc(cfg(test)))]
            $item
        )*
    }
}
