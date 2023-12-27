macro_rules! include_input {
    ($year:literal $day:literal) => {
        include_str!(concat!(
            "../../../inputs/20",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        ))
    };
}
