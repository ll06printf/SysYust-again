use clap::Parser;



#[derive(clap::Parser, Debug)]
#[command(name = "sysyust")]
#[command(version = "0.0.0")]
#[command(about = "The SysY2022 Compiler.", long_about = None)]
struct Args {
    #[command(flatten)]
    target: TransformTarget
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct TransformTarget {

    /// transform source file to exeutable that implemented by a interpreter.
    #[arg(long)]
    #[arg(default_value_t = true)]
    interpreter: bool,

    /// tranfrom source file to llvm ir
    #[arg(long)]
    llvm_ir_dump: bool,

    /// to rsicv assembly executable
    #[arg(long)]
    riscv: bool,


}

#[derive(Debug)]
enum Target {
    Interpreter,
    LLVMIr,
    RiscV,
}

fn map_argments_to_target(arg: &Args) -> Target {
    if arg.target.interpreter {
        Target::Interpreter
    } else if arg.target.llvm_ir_dump {
        Target::LLVMIr
    } else if arg.target.riscv {
        Target::RiscV
    } else {
        panic!("Unexpected target")
    }
}

fn main() {
    let arg = Args::parse();
    let target = map_argments_to_target(&arg);
    println!("{:?}", target);
}
