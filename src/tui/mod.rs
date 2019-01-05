use super::bidding;
use cursive::{
    views::{Dialog, EditView, LinearLayout, TextView},
    Cursive,
};

pub fn run() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    // Adds a keyboard shortcut for quitting at any time.
    siv.add_global_callback('q', |s| s.quit());

    // Shows the main menu.
    show_login_menu(&mut siv);

    // Starts the event loop.
    siv.run();
}

fn show_login_menu(s: &mut Cursive) {
    s.pop_layer();
    let dialog = if let Ok(u) = bidding::current_user() {
        let text = format!("Currently logged in as <{}>", u.email);
        let content = TextView::new(text);
        Dialog::around(content)
            .title("Login")
            .button("Continue", show_main_menu)
            .button("Change User", |s| {
                bidding::logout();
                show_login_menu(s);
            })
    } else {
        let input = EditView::new().on_submit(try_login);
        let content = LinearLayout::vertical()
            .child(TextView::new(
                "please log in with your email address to continue",
            ))
            .child(input);
        Dialog::around(content).title("Login")
    };
    s.add_layer(dialog)
}

fn try_login(s: &mut Cursive, email: &str) {
    match bidding::login(email) {
        Ok(()) => show_login_menu(s),
        Err(e) => panic!("{:?}", e),
    }
}

fn show_main_menu(s: &mut Cursive) {
    s.pop_layer();
    let content = TextView::new("Practice your bridge bidding!");
    let dialog = Dialog::around(content)
        .title("Main Menu")
        .button("Bid", show_bidding)
        .button("Review", show_review)
        .button("Quit", |s| s.quit());
    s.add_layer(dialog);
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
