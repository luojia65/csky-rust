use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".to_string(),
        llvm_target: "csky".to_string(),
        pointer_width: 32,
        arch: "csky".to_string(),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker: Some("rust-lld".to_string()),
            cpu: "generic".to_string(),
            max_atomic_width: Some(0),
            atomic_cas: false,
            features: "e2".to_string(), // todo: features
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
