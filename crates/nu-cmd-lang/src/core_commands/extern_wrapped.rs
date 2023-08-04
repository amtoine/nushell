use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Type};

#[derive(Clone)]
pub struct ExternWrapped;

impl Command for ExternWrapped {
    fn name(&self) -> &str {
        "extern-wrapped"
    }

    fn usage(&self) -> &str {
        "Define a signature for an external command."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("extern-wrapped")
            .input_output_types(vec![(Type::Null, Type::Null)])
            .required("def_name", SyntaxShape::String, "definition name")
            .required("params", SyntaxShape::Signature, "parameters")
            .required("body", SyntaxShape::Block, "wrapper function block")
            .category(Category::Core)
    }

    fn extra_usage(&self) -> &str {
        r#"This command is a parser keyword. For details, check:
  https://www.nushell.sh/book/thinking_in_nu.html"#
    }

    fn is_parser_keyword(&self) -> bool {
        true
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(PipelineData::empty())
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Write a signature for an external command",
            example: r#"extern-wrapped echo [text: string] { print $text }"#,
            result: None,
        }]
    }
}
