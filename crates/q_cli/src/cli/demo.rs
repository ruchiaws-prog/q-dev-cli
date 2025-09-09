use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use anstream::{eprintln, println};
use clap::Args;
use crossterm::style::Stylize;
use eyre::Result;
use tokio::time::sleep;

use crate::util::CliContext;

#[derive(Debug, Args, PartialEq, Eq)]
pub struct DemoArgs {
    /// Skip interactive prompts and run full demo
    #[arg(long, short)]
    auto: bool,
    /// Show only a specific feature demo
    #[arg(long, value_enum)]
    feature: Option<DemoFeature>,
}

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum DemoFeature {
    /// Demonstrate chat functionality
    Chat,
    /// Demonstrate translate functionality
    Translate,
    /// Demonstrate settings management
    Settings,
    /// Demonstrate doctor diagnostics
    Doctor,
}

impl DemoArgs {
    pub async fn execute(&self, _cli_context: &CliContext) -> Result<ExitCode> {
        println!("{}", "🎯 Amazon Q CLI Demo".bold().magenta());
        println!("{}", "=".repeat(50).dim());
        println!();

        if let Some(feature) = &self.feature {
            self.run_feature_demo(feature).await?;
        } else if self.auto {
            self.run_full_demo().await?;
        } else {
            self.run_interactive_demo().await?;
        }

        Ok(ExitCode::SUCCESS)
    }

    async fn run_interactive_demo(&self) -> Result<()> {
        println!("Welcome to the Amazon Q CLI interactive demo!");
        println!("This demo will showcase the key features of the Amazon Q CLI.");
        println!();

        if !self.prompt_continue("Would you like to start the demo?").await? {
            println!("Demo cancelled. Run {} to try again!", "q demo".cyan());
            return Ok(());
        }

        self.run_full_demo().await
    }

    async fn run_full_demo(&self) -> Result<()> {
        let features = [
            DemoFeature::Chat,
            DemoFeature::Translate,
            DemoFeature::Settings,
            DemoFeature::Doctor,
        ];

        for (i, feature) in features.iter().enumerate() {
            if i > 0 {
                println!();
                if !self.auto && !self.prompt_continue("Continue to next feature?").await? {
                    break;
                }
                println!();
            }
            self.run_feature_demo(feature).await?;
        }

        self.show_next_steps().await?;
        Ok(())
    }

    async fn run_feature_demo(&self, feature: &DemoFeature) -> Result<()> {
        match feature {
            DemoFeature::Chat => self.demo_chat().await,
            DemoFeature::Translate => self.demo_translate().await,
            DemoFeature::Settings => self.demo_settings().await,
            DemoFeature::Doctor => self.demo_doctor().await,
        }
    }

    async fn demo_chat(&self) -> Result<()> {
        println!("{}", "💬 Chat Feature".bold().blue());
        println!("{}", "-".repeat(20).dim());
        println!("The {} command starts an AI-powered chat session in your terminal.", "q chat".cyan());
        println!("You can ask questions about your code, get help with commands, and more!");
        println!();
        println!("{}", "Example usage:".bold());
        println!("  {} {}", "❯".dim(), "q chat".cyan());
        println!("  {} {}", "❯".dim(), "q chat \"How do I list files in Linux?\"".cyan());
        println!();
        println!("{}", "Key features:".bold());
        println!("  • Natural language interaction");
        println!("  • Context-aware responses");
        println!("  • Code generation and explanation");
        println!("  • Integration with your local environment");
        
        self.typing_effect("  • Real-time assistance with development tasks").await;
        Ok(())
    }

    async fn demo_translate(&self) -> Result<()> {
        println!("{}", "🔄 Translate Feature".bold().green());
        println!("{}", "-".repeat(20).dim());
        println!("The {} command converts natural language to shell commands.", "q translate".cyan());
        println!("Perfect for when you know what you want to do but not the exact command!");
        println!();
        println!("{}", "Example usage:".bold());
        println!("  {} {}", "❯".dim(), "q translate \"find all python files\"".cyan());
        println!("  {} {}", "❯".dim(), "q ai \"compress this folder\"".cyan());
        println!();
        println!("{}", "Sample translations:".bold());
        
        let examples = [
            ("\"list all files larger than 100MB\"", "find . -size +100M -type f"),
            ("\"show running processes\"", "ps aux"),
            ("\"create a backup of config.json\"", "cp config.json config.json.bak"),
        ];

        for (input, output) in examples {
            self.typing_effect(&format!("  {} → {}", input.yellow(), output.cyan())).await;
            sleep(Duration::from_millis(500)).await;
        }
        Ok(())
    }

    async fn demo_settings(&self) -> Result<()> {
        println!("{}", "⚙️  Settings Feature".bold().yellow());
        println!("{}", "-".repeat(20).dim());
        println!("The {} command helps you customize Amazon Q CLI behavior.", "q settings".cyan());
        println!("Configure themes, integrations, and personal preferences.");
        println!();
        println!("{}", "Example usage:".bold());
        println!("  {} {}", "❯".dim(), "q settings".cyan());
        println!("  {} {}", "❯".dim(), "q settings all".cyan());
        println!("  {} {}", "❯".dim(), "q settings open".cyan());
        println!();
        println!("{}", "What you can configure:".bold());
        println!("  • Appearance and themes");
        println!("  • Shell integrations");
        println!("  • Autocomplete behavior");
        println!("  • Telemetry preferences");
        
        self.typing_effect("  • Custom shortcuts and aliases").await;
        Ok(())
    }

    async fn demo_doctor(&self) -> Result<()> {
        println!("{}", "🩺 Doctor Feature".bold().red());
        println!("{}", "-".repeat(20).dim());
        println!("The {} command diagnoses and fixes common installation issues.", "q doctor".cyan());
        println!("Run this when something isn't working as expected!");
        println!();
        println!("{}", "Example usage:".bold());
        println!("  {} {}", "❯".dim(), "q doctor".cyan());
        println!("  {} {}", "❯".dim(), "q doctor --all".cyan());
        println!();
        println!("{}", "What it checks:".bold());
        println!("  • Shell integration status");
        println!("  • Permission issues");
        println!("  • Configuration problems");
        println!("  • Network connectivity");
        
        self.typing_effect("  • System compatibility").await;
        Ok(())
    }

    async fn show_next_steps(&self) -> Result<()> {
        println!();
        println!("{}", "🚀 Next Steps".bold().magenta());
        println!("{}", "=".repeat(50).dim());
        println!();
        println!("Ready to get started? Here's what you can do:");
        println!();
        println!("1. {} - Start chatting with Amazon Q", "q chat".cyan());
        println!("2. {} - Try natural language translation", "q translate \"your command here\"".cyan());
        println!("3. {} - Customize your experience", "q settings".cyan());
        println!("4. {} - Get help if you run into issues", "q doctor".cyan());
        println!("5. {} - See all available commands", "q --help".cyan());
        println!();
        println!("For more information, visit: {}", "https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html".blue().underlined());
        println!();
        println!("{}", "Happy coding! 🎉".bold().green());
        Ok(())
    }

    async fn prompt_continue(&self, message: &str) -> Result<bool> {
        print!("{} (y/N): ", message);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_lowercase().starts_with('y'))
    }

    async fn typing_effect(&self, text: &str) -> () {
        if self.auto {
            println!("{}", text);
            return;
        }

        for char in text.chars() {
            print!("{}", char);
            io::stdout().flush().unwrap_or(());
            sleep(Duration::from_millis(30)).await;
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_args_parsing() {
        // Test basic parsing
        let args = DemoArgs {
            auto: false,
            feature: None,
        };
        assert!(!args.auto);
        assert!(args.feature.is_none());

        // Test with feature
        let args = DemoArgs {
            auto: true,
            feature: Some(DemoFeature::Chat),
        };
        assert!(args.auto);
        assert_eq!(args.feature, Some(DemoFeature::Chat));
    }

    #[test]
    fn test_demo_feature_enum() {
        // Test all variants exist
        let features = [
            DemoFeature::Chat,
            DemoFeature::Translate,
            DemoFeature::Settings,
            DemoFeature::Doctor,
        ];
        
        assert_eq!(features.len(), 4);
    }
}