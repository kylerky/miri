use rustc_const_eval::interpret::{InterpResult, interp_ok};

use super::backtrace::EvalContextExt as _;
use crate::*;

impl<'tcx> EvalContextExt<'tcx> for crate::MiriInterpCx<'tcx> {}
pub trait EvalContextExt<'tcx>: crate::MiriInterpCxExt<'tcx> {
    fn register_handler(&mut self, operand: &OpTy<'tcx>) -> InterpResult<'tcx, ()> {
        let this = self.eval_context_mut();
        let (instance, _, _, _) = this.resolve_frame_pointer(operand)?;
        info!("Registering {} as the dangling pointer handler", instance);
        this.machine.dangling_ptr_handler.replace(instance);
        interp_ok(())
    }
}
