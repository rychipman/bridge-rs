use chrono::NaiveDateTime;
use diesel::{dsl::sql, insert_into, prelude::*, result::Error as DieselError, update};
use rand::Rng;
use schema::{tokens, users};
use std::iter;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
pub struct Token {
    pub token: String,
    user_id: i32,
}

impl Token {
    pub fn new(conn: &SqliteConnection, user_id: i32) -> Result<Token, DieselError> {
        use schema::tokens::dsl::tokens;
        let token = Token::generate(user_id);
        insert_into(tokens).values(&token).execute(conn)?;
        Ok(token)
    }

    fn generate(user_id: i32) -> Token {
        let mut rng = rand::thread_rng();
        let token = iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .take(64)
            .collect();
        Token { token, user_id }
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserInsert {
    email: String,
    pw_hash: String,
}

impl UserInsert {
    pub fn new(email: String, password: String) -> Self {
        if password.len() < 5 {
            panic!("password must be at least five chars long")
        }
        let pw_hash = User::hash_password(&password);
        UserInsert { email, pw_hash }
    }

    pub fn insert(&self, conn: &SqliteConnection) -> Result<usize, DieselError> {
        use schema::users::dsl::*;
        insert_into(users).values(self).execute(conn)
    }
}

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip)]
    pub pw_hash: String,
    pub last_active: NaiveDateTime,
}

impl User {
    pub fn all(conn: &SqliteConnection) -> Result<Vec<User>, DieselError> {
        use schema::users::dsl::*;
        users.load(conn)
    }

    pub fn register_activity(&self, conn: &SqliteConnection) -> Result<(), DieselError> {
        use schema::users::dsl::*;
        update(users.filter(id.eq(self.id)))
            .set(last_active.eq(sql("datetime('now')")))
            .execute(conn)
            .map(|_| ())
    }

    fn hash_password(password: &str) -> String {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("failed to hash password")
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.pw_hash).expect("unexpected error while verifying password")
    }

    pub fn by_email(conn: &SqliteConnection, user_email: &str) -> Result<User, DieselError> {
        use schema::users::dsl::*;
        users.filter(email.eq(user_email)).first(conn)
    }

    pub fn by_id(conn: &SqliteConnection, uid: i32) -> Result<User, DieselError> {
        use schema::users::dsl::*;
        users.filter(id.eq(uid)).first(conn)
    }

    pub fn by_token(conn: &SqliteConnection, tok: String) -> Result<User, DieselError> {
        use schema::{tokens::dsl::*, users::dsl::users};
        tokens
            .inner_join(users)
            .filter(token.eq(tok))
            .first(conn)
            .map(|(_, user): (Token, User)| user)
    }
}
