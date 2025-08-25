use clap::{Arg, Command};

pub fn build() -> Command {
	Command::new("Jidhom Control")
		.bin_name(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.subcommand_required(true)
		.disable_help_subcommand(true)
		.subcommand(
			Command::new("user")
				.about("Managing and creating users")
				.subcommand_required(true)
				.subcommand(
					Command::new("new-recruitment-manager")
						.about("Create a new recuitment manager user with a temporaty password"),
				)
				.subcommand(
					Command::new("reset-password")
						.about("Reset user's password and create a new temporary one")
						.arg(Arg::new("user_id").value_name("USER_ID").required(true)),
				),
		)
}
