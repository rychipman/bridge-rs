use super::bidding::{self, Deal, Exercise};
use super::game::Bid;
use cursive::{
    traits::*,
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

fn show_exercises(s: &mut Cursive) {
    let exercises = Exercise::all().expect("failed to get exercises");
    let (exercise, deal) = &exercises[0];

    s.pop_layer();

    let next_seat = exercise.bids.next_seat(deal.dealer);
    let deal = TextView::new(format!(
        "{}{}",
        deal.header(),
        deal.view_for_seat(next_seat)
    ));
    let ex = TextView::new(format!("{}", exercise));

    let content = LinearLayout::horizontal().child(deal).child(ex);
    let dialog = Dialog::around(content)
        .title(format!("Exercise #{}", exercise.id))
        .button("Prev", |_s| println!("not implemented"))
        .button("Back", show_main_menu)
        .button("Next", |_s| println!("not implemented"));

    s.add_layer(dialog);
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
    let exercise = Exercise::get_random().expect("failed to get exercise");
    let exercise_id = exercise.id;
    let deal = Deal::get(exercise.deal_id).expect("failed to get deal");

    s.pop_layer();

    let next_seat = exercise.bids.next_seat(deal.dealer);
    let deal = TextView::new(format!(
        "{}{}",
        deal.header(),
        deal.view_for_seat(next_seat)
    ));
    let ex = TextView::new(format!("{}", exercise));
    let ex_view = LinearLayout::horizontal().child(deal).child(ex);

    let bid_message = TextView::new("Please enter your bid:").with_id("bid_message");
    let bid_input = EditView::new().on_submit(move |s, bid| {
        let bid = Bid::parse(&bid).expect("failed to parse bid");
        let user = bidding::current_user().expect("failed to get current user");
        let _ex_bid = exercise
            .insert_bid(user.id, &bid)
            .expect("failed to create exercise bid");
        s.call_on_id("bid_message", |view: &mut TextView| {
            view.set_content(format!("Submitted bid: {}", bid))
        });
        s.call_on_id("bid_view", |view: &mut LinearLayout| view.remove_child(1));
    });
    let bid_view = LinearLayout::vertical()
        .child(bid_message)
        .child(bid_input)
        .with_id("bid_view");

    let content = LinearLayout::vertical().child(ex_view).child(bid_view);

    let dialog = Dialog::around(content)
        .title(format!("Exercise #{}", exercise_id))
        .button("Prev", |_s| println!("not implemented"))
        .button("Back", show_main_menu)
        .button("Next", |_s| println!("not implemented"));

    s.add_layer(dialog);
}

fn show_review(s: &mut Cursive) {
    show_exercises(s);
    /*
    s.pop_layer();
    s.add_layer(
        Dialog::around(TextView::new("No review actions available yet."))
            .title("Review")
            .button("Back", show_main_menu),
    );
    */
}
