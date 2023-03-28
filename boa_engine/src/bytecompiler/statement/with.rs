use crate::{bytecompiler::ByteCompiler, vm::Opcode};
use boa_ast::statement::With;

impl ByteCompiler<'_, '_> {
    /// Compile a [`With`] `boa_ast` node
    pub(crate) fn compile_with(&mut self, with: &With, configurable_globals: bool) {
        self.compile_expr(with.expression(), true);
        self.context.push_compile_time_environment(false);
        self.emit_opcode(Opcode::PushObjectEnvironment);
        self.compile_stmt(with.statement(), false, configurable_globals);
        self.context.pop_compile_time_environment();
        self.emit_opcode(Opcode::PopEnvironment);
    }
}