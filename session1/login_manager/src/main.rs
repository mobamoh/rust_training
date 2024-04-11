use clap::{Parser, Subcommand};
use authentication::{get_users, hash_password, LoginRole, save_users, User};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a new user.
    Add {
        /// The user's login name.
        username: String,
        /// The user's password (plaintext)
        password: String,
        /// Optional - mark as an admin.
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete an existing user.
    Delete {
        /// The user's login name.
        username: String
    },
    /// Update user's password.
    UpdatePwd {
        /// The user's login name.
        username: String,
        /// The user's new password.
        password: String,
    },
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users()
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete { username }) => {
            remove_user(username);
        }
        Some(Commands::UpdatePwd { username, password }) => {
            update_password(username, password);
        }
        None => println!("Run --help to see instructions.")
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");
    authentication::get_users().iter().for_each(|(_, u)| println!("{:<20}{:<20?}", u.username, u.role));
}

fn add_user(username: String, password: String, admin: bool) {
    let role = if admin {
        LoginRole::Admin
    } else { LoginRole::User };

    let mut users = get_users();
    let new_user = User::new(&username, &password, role);
    users.insert(username, new_user);
    save_users(users);
    println!("✅ User added successfully!")
}

fn remove_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
        println!("✅ User deleted successfully!")
    } else {
        println!("❌ Username does not exist!")
    }
}

fn update_password(username: String, new_password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = hash_password(&new_password);
        save_users(users);
        println!("✅ Password updated successfully!")
    } else {
        println!("❌ Username does not exist!")
    }
}