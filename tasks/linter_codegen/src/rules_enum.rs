use std::fmt::Write as _;

use convert_case::{Case, Casing};

use crate::rules::RuleEntry;

/// Generate the RuleEnum and related code that replaces `declare_all_lint_rules!` macro.
pub fn generate_rules_enum(rule_entries: &[RuleEntry<'_>]) -> String {
    let mut out = String::new();

    out.push_str("// Auto-generated code, DO NOT EDIT DIRECTLY!\n");
    out.push_str("// To regenerate: `cargo run -p oxc_linter_codegen`\n\n");

    // Generate use statements and type aliases
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(
            out,
            "pub use crate::rules::{}::{}::{} as {};",
            rule.plugin_module_name,
            rule.rule_module_name,
            rule.rule_struct_name(),
            enum_name,
        )
        .unwrap();
    }

    out.push('\n');

    // Generate imports
    out.push_str(
        "use crate::{
    context::{ContextHost, LintContext},
    rule::{Rule, RuleCategory, RuleFixMeta, RuleMeta, RuleRunner, RuleRunFunctionsImplemented},
    utils::PossibleJestNode,
    AstNode
};
use oxc_semantic::AstTypesBitset;

",
    );

    // Generate RuleEnum
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("#[expect(clippy::enum_variant_names)]\n");
    out.push_str("pub enum RuleEnum {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "    {enum_name}({enum_name}),").unwrap();
    }
    out.push_str("}\n\n");

    // Generate RuleEnum impl
    out.push_str("impl RuleEnum {\n");

    // id()
    generate_match_method(&mut out, "id", "&self", "usize", rule_entries, |_, idx| {
        format!("{idx}")
    });

    // name()
    generate_match_method(&mut out, "name", "&self", "&'static str", rule_entries, |rule, _| {
        format!("{}::NAME", make_enum_name(rule))
    });

    // category()
    generate_match_method(
        &mut out,
        "category",
        "&self",
        "RuleCategory",
        rule_entries,
        |rule, _| format!("{}::CATEGORY", make_enum_name(rule)),
    );

    // fix()
    out.push_str("    /// This [`Rule`]'s auto-fix capabilities.\n");
    generate_match_method(&mut out, "fix", "&self", "RuleFixMeta", rule_entries, |rule, _| {
        format!("{}::FIX", make_enum_name(rule))
    });

    // documentation() - cfg(feature = "ruledocs")
    out.push_str("    #[cfg(feature = \"ruledocs\")]\n");
    generate_match_method(
        &mut out,
        "documentation",
        "&self",
        "Option<&'static str>",
        rule_entries,
        |rule, _| format!("{}::documentation()", make_enum_name(rule)),
    );

    // schema() - cfg(feature = "ruledocs")
    out.push_str("    #[cfg(feature = \"ruledocs\")]\n");
    out.push_str("    pub fn schema(&self, generator: &mut schemars::SchemaGenerator) -> Option<schemars::schema::Schema> {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(
            out,
            "            Self::{enum_name}(_) => {enum_name}::config_schema(generator).or_else(|| {enum_name}::schema(generator)),"
        )
        .unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // plugin_name()
    out.push_str("    pub fn plugin_name(&self) -> &'static str {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        let plugin_name = rule.plugin_module_name;
        writeln!(out, "            Self::{enum_name}(_) => \"{plugin_name}\",").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // from_configuration()
    out.push_str("    pub fn from_configuration(&self, value: serde_json::Value) -> Result<Self, serde_json::error::Error> {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(
            out,
            "            Self::{enum_name}(_) => Ok(Self::{enum_name}({enum_name}::from_configuration(value)?)),"
        )
        .unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // to_configuration()
    out.push_str("    pub fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(rule) => rule.to_configuration(),").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // run()
    out.push_str("    pub(crate) fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(rule) => rule.run(node, ctx),").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // run_once()
    out.push_str("    pub(crate) fn run_once<'a>(&self, ctx: &LintContext<'a>) {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(rule) => rule.run_once(ctx),").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // run_on_jest_node()
    out.push_str("    pub(crate) fn run_on_jest_node<'a, 'c>(\n");
    out.push_str("        &self,\n");
    out.push_str("        jest_node: &PossibleJestNode<'a, 'c>,\n");
    out.push_str("        ctx: &'c LintContext<'a>,\n");
    out.push_str("    ) {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(
            out,
            "            Self::{enum_name}(rule) => rule.run_on_jest_node(jest_node, ctx),"
        )
        .unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // should_run()
    out.push_str("    pub(crate) fn should_run(&self, ctx: &ContextHost) -> bool {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(rule) => rule.should_run(ctx),").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // is_tsgolint_rule()
    out.push_str("    pub fn is_tsgolint_rule(&self) -> bool {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(_) => {enum_name}::IS_TSGOLINT_RULE,")
            .unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // types_info()
    out.push_str("    pub fn types_info(&self) -> Option<&'static AstTypesBitset> {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(rule) => rule.types_info(),").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // run_info()
    out.push_str("    pub fn run_info(&self) -> RuleRunFunctionsImplemented {\n");
    out.push_str("        match self {\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "            Self::{enum_name}(rule) => rule.run_info(),").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n");

    out.push_str("}\n\n");

    // Generate Hash impl
    out.push_str("impl std::hash::Hash for RuleEnum {\n");
    out.push_str("    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {\n");
    out.push_str("        self.id().hash(state);\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate PartialEq impl
    out.push_str("impl PartialEq for RuleEnum {\n");
    out.push_str("    fn eq(&self, other: &Self) -> bool {\n");
    out.push_str("        self.id() == other.id()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate Eq impl
    out.push_str("impl Eq for RuleEnum {}\n\n");

    // Generate Ord impl
    out.push_str("impl Ord for RuleEnum {\n");
    out.push_str("    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n");
    out.push_str("        self.id().cmp(&other.id())\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate PartialOrd impl
    out.push_str("impl PartialOrd for RuleEnum {\n");
    out.push_str("    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {\n");
    out.push_str("        Some(self.cmp(other))\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate RULES static
    out.push_str("pub static RULES: std::sync::LazyLock<Vec<RuleEnum>> = std::sync::LazyLock::new(|| vec![\n");
    for rule in rule_entries {
        let enum_name = make_enum_name(rule);
        writeln!(out, "    RuleEnum::{enum_name}({enum_name}::default()),").unwrap();
    }
    out.push_str("]);\n");

    out
}

/// Create the enum variant name from a rule entry.
/// e.g., `eslint::no_debugger` -> `EslintNoDebugger`
fn make_enum_name(rule: &RuleEntry<'_>) -> String {
    format!(
        "{}{}",
        rule.plugin_module_name.to_case(Case::Pascal),
        rule.rule_module_name.to_case(Case::Pascal)
    )
}

/// Helper to generate simple match methods
fn generate_match_method<F>(
    out: &mut String,
    name: &str,
    self_param: &str,
    return_type: &str,
    rule_entries: &[RuleEntry<'_>],
    body_fn: F,
) where
    F: Fn(&RuleEntry<'_>, usize) -> String,
{
    writeln!(out, "    pub fn {name}({self_param}) -> {return_type} {{").unwrap();
    out.push_str("        match self {\n");
    for (idx, rule) in rule_entries.iter().enumerate() {
        let enum_name = make_enum_name(rule);
        let body = body_fn(rule, idx);
        writeln!(out, "            Self::{enum_name}(_) => {body},").unwrap();
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");
}
