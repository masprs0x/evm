use super::routines;
use crate::standard::{Config, TransactGasometer};
use crate::{
	Color, ColoredMachine, Control, Etable, ExitError, ExitResult, Gasometer, InvokerControl,
	Machine, Opcode, RuntimeBackend, RuntimeState,
};
use alloc::rc::Rc;
use primitive_types::H160;

pub trait Resolver<S, G, H, Tr> {
	type Color: Color<S, G, H, Tr>;

	fn resolve_call(
		&self,
		code_address: H160,
		input: Vec<u8>,
		is_static: bool,
		state: S,
		gasometer: G,
		handler: &mut H,
	) -> Result<
		InvokerControl<ColoredMachine<S, G, Self::Color>, (ExitResult, (S, G, Vec<u8>))>,
		ExitError,
	>;

	fn resolve_create(
		&self,
		init_code: Vec<u8>,
		is_static: bool,
		state: S,
		gasometer: G,
		handler: &mut H,
	) -> Result<
		InvokerControl<ColoredMachine<S, G, Self::Color>, (ExitResult, (S, G, Vec<u8>))>,
		ExitError,
	>;
}

pub trait PrecompileSet<S, G, H> {
	fn execute(
		&self,
		code_address: H160,
		input: &[u8],
		is_static: bool,
		state: &mut S,
		gasometer: &mut G,
		handler: &mut H,
	) -> Option<(ExitResult, Vec<u8>)>;
}

impl<S, G, H> PrecompileSet<S, G, H> for () {
	fn execute(
		&self,
		_code_address: H160,
		_input: &[u8],
		_is_static: bool,
		_state: &mut S,
		_gasometer: &mut G,
		_handler: &mut H,
	) -> Option<(ExitResult, Vec<u8>)> {
		None
	}
}

pub struct EtableResolver<'config, 'precompile, 'etable, S, H, Pre, Tr, F> {
	config: &'config Config,
	etable: &'etable Etable<S, H, Tr, F>,
	precompiles: &'precompile Pre,
}

impl<'config, 'precompile, 'etable, S, H, Pre, Tr, F>
	EtableResolver<'config, 'precompile, 'etable, S, H, Pre, Tr, F>
{
	pub fn new(
		config: &'config Config,
		precompiles: &'precompile Pre,
		etable: &'etable Etable<S, H, Tr, F>,
	) -> Self {
		Self {
			config,
			precompiles,
			etable,
		}
	}
}

impl<'config, 'precompile, 'etable, S, G, H, Pre, Tr, F> Resolver<S, G, H, Tr>
	for EtableResolver<'config, 'precompile, 'etable, S, H, Pre, Tr, F>
where
	S: AsRef<RuntimeState> + AsMut<RuntimeState>,
	G: Gasometer<S, H> + TransactGasometer<'config, S>,
	F: Fn(&mut Machine<S>, &mut H, Opcode, usize) -> Control<Tr>,
	H: RuntimeBackend,
	Pre: PrecompileSet<S, G, H>,
{
	type Color = &'etable Etable<S, H, Tr, F>;

	fn resolve_call(
		&self,
		code_address: H160,
		input: Vec<u8>,
		is_static: bool,
		mut state: S,
		mut gasometer: G,
		handler: &mut H,
	) -> Result<
		InvokerControl<
			ColoredMachine<S, G, &'etable Etable<S, H, Tr, F>>,
			(ExitResult, (S, G, Vec<u8>)),
		>,
		ExitError,
	> {
		if let Some((r, retval)) = self.precompiles.execute(
			code_address,
			&input,
			is_static,
			&mut state,
			&mut gasometer,
			handler,
		) {
			return Ok(InvokerControl::DirectExit((r, (state, gasometer, retval))));
		}

		let code = handler.code(code_address);

		let machine = Machine::<S>::new(
			Rc::new(code),
			Rc::new(input),
			self.config.stack_limit,
			self.config.memory_limit,
			state,
		);

		let mut ret = InvokerControl::Enter(ColoredMachine {
			machine,
			gasometer,
			is_static,
			color: self.etable,
		});
		routines::maybe_analyse_code(&mut ret);

		Ok(ret)
	}

	fn resolve_create(
		&self,
		init_code: Vec<u8>,
		is_static: bool,
		state: S,
		gasometer: G,
		_handler: &mut H,
	) -> Result<
		InvokerControl<
			ColoredMachine<S, G, &'etable Etable<S, H, Tr, F>>,
			(ExitResult, (S, G, Vec<u8>)),
		>,
		ExitError,
	> {
		let machine = Machine::new(
			Rc::new(init_code),
			Rc::new(Vec::new()),
			self.config.stack_limit,
			self.config.memory_limit,
			state,
		);

		let mut ret = InvokerControl::Enter(ColoredMachine {
			machine,
			gasometer,
			is_static,
			color: self.etable,
		});
		routines::maybe_analyse_code(&mut ret);

		Ok(ret)
	}
}