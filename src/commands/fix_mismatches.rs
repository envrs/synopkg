use colored::Colorize;

/// Run the fix-mismatches command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in synopkg v14".red().bold());
  eprintln!();
  eprintln!("{}", "fix-mismatches → fix".yellow().bold());
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
  eprintln!("{}", "Migration Example:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "synopkg fix-mismatches".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "synopkg fix".cyan());
  eprintln!();
  eprintln!(
    "{}",
    "https://envrs.github.io/synopkg/guide/migrate-v14#fix-mismatches-fix"
      .blue()
      .underline()
  );
  eprintln!();
  1
}
