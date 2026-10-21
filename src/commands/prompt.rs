use {
  crate::{context::Context, errors::SynopkgError},
  colored::Colorize,
};

/// Run the prompt command (deprecated in v14)
pub fn run(_ctx: Context) -> Result<Context, SynopkgError> {
  eprintln!("{}", "Deprecated in synopkg v14".red().bold());
  eprintln!();
  eprintln!("{}", "prompt → Removed".yellow().bold());
  eprintln!();
  eprintln!(
    "The {} command is an interactive prompt which lists all current issues which synopkg",
    "prompt".cyan()
  );
  eprintln!("can't fix automatically. It is not yet available in v14 and will be added at a later date.");
  eprintln!();
  eprintln!("Synopkg can't automatically fix mismatches between specifiers it does not support,");
  eprintln!("which are usually specifiers which are not semver, such as pnpm overrides, or complex");
  eprintln!("semver specifiers like {}.", "^1.2.3 || ^2.0.0".cyan());
  eprintln!();
  eprintln!("{}", "Status:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "synopkg prompt".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "# Not yet implemented".red());
  eprintln!();
  eprintln!("{}", "https://envrs.github.io/synopkg/guide/migrate-v14#prompt-removed".blue().underline());
  eprintln!();
  Err(SynopkgError::DeprecatedCommand)
}
