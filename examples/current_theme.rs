fn main() {
    #[cfg(all(feature = "gtk", unix, not(target_os = "macos")))]
    {
        println!("{:#?}", rusty_themy::gtk::current::current())
    }
}
