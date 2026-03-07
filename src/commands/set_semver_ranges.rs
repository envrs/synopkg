use colored::Colorize;

/// Run the set-semver-ranges command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in synopkg v14".red().bold());
  eprintln!();
  eprintln!("{}", "set-semver-ranges → fix".yellow().bold());
  eprintln!();
  eprintln!(
    "{} and {} have been merged into a single {} command which autofixes",
    "fix-mismatches".cyan(),
    "set-semver-ranges".cyan(),
    "fix".green()
  );
  eprintln!(
    "issues found by {}. The {} command no longer fixes formatting, which is now handled",
    "synopkg lint".green(),
    "fix".green()
  );
  eprintln!("by {}.", "synopkg format".green());
  eprintln!();
  eprintln!("It's no longer possible to manage semver ranges without also checking version mismatches,");
  eprintln!("because the two things are so closely linked. Changes to semver ranges affect which");
  eprintln!("versions are considered valid and can indirectly cause version mismatches, so they are");
  eprintln!(
    "now always checked and changed together via the {} and {} commands.",
    "lint".green(),
    "fix".green()
  );
  eprintln!();
  eprintln!("{}", "Migration Example:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "synopkg set-semver-ranges".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "synopkg fix".cyan());
  eprintln!();
  eprintln!(
    "{}",
    "https://envrs.github.io/synopkg/guide/migrate-v14#set-semver-ranges-fix"
      .blue()
      .underline()
  );
  eprintln!();
  1
}
