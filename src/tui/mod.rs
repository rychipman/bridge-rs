use cursive::{
    views::{Dialog, TextView},
    Cursive,
};

pub fn run() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    // Adds a keyboard shortcut for quitting at any time.
    siv.add_global_callback('q', |s| s.quit());

    // Shows the main menu.
    show_main_menu(&mut siv);

    // Starts the event loop.
    siv.run();
}

fn show_main_menu(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(TextView::new("Practice your bridge skills!"))
            .title("Main Menu")
            .button("Bid", show_bidding)
            .button("Review", show_review)
            .button("Quit", |s| s.quit()),
    );
}

fn show_bidding(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(TextView::new("No bidding actions available yet."))
            .title("Bidding")
            .button("Back", show_main_menu),
    );
}

fn show_review(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(TextView::new("No review actions available yet."))
            .title("Review")
            .button("Back", show_main_menu),
    );
}
