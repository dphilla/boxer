use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use dockerfile_parser::*;

fn main() {

// TODO: check validity first
let dockerfile = Dockerfile::parse(r#"
FROM python:3

WORKDIR /usr/src/app

COPY . .

CMD [ "python", "./your-script.py" ]
"#).unwrap();

for stage in dockerfile.iter_stages() {
  println!("stage #{}", stage.index);
  let mut builder = Builder::new();
  for ins in stage.instructions {
    match ins {
        Instruction::From(instr) => execute_from(builder, instr.clone()),
        Instruction::Arg(instr) => execute_arg(instr.clone()),
        Instruction::Label(instr) => execute_label(instr.clone()),
        Instruction::Run(instr) => execute_run(instr.clone()),
        Instruction::Entrypoint(instr) => execute_entrypoint(instr.clone()),
        Instruction::Cmd(instr) => execute_cmd(instr.clone()),
        Instruction::Copy(instr) => execute_copy(instr.clone()),
        Instruction::Env(instr) => execute_env(instr.clone()),
        Instruction::Misc(instr) => execute_misc(instr.clone()),
    }
    println!("  {:?}", ins);
    println!("\n");
  }

  builder.build();
}

}

fn execute_from(builder: Builder, instr: FromInstruction) {
    builder.config_base(instr.content);

    println!("  {:?}", instr);
}

fn execute_run(instr: RunInstruction) {
    println!("  {:?}", instr);
}

fn execute_cmd(instr: CmdInstruction) {
    println!("  {:?}", instr);
}

fn execute_label(instr: LabelInstruction) {
    println!("  {:?}", instr);
}

//fn execute_expose(instr: ExposeInstruction) {
    //println!("  {:?}", instr);
//}

fn execute_env(instr: EnvInstruction) {
    println!("  {:?}", instr);
}

//fn execute_add(instr: AddInstruction) {
    //println!("  {:?}", instr);
//}

fn execute_copy(instr: CopyInstruction) {
    println!("  {:?}", instr);
}

fn execute_entrypoint(instr: EntrypointInstruction) {
    println!("  {:?}", instr);
}

fn execute_arg(instr: ArgInstruction) {
    println!("  {:?}", instr);
}

fn execute_misc(instr: MiscInstruction) {
    //TODO: handles: `MAINTAINER`, `EXPOSE`, `VOLUME`,
        // `USER`, `WORKDIR`, `ONBUILD`, `STOPSIGNAL`, `HEALTHCHECK`, `SHELL`
    println!("  {:?}", instr);
}

//fn execute_volume() {
    //println!("Executing VOLUME directive");
//}

//fn execute_user() {
    //println!("Executing USER directive");
//}

//fn execute_workdir() {
    //println!("Executing WORKDIR directive");
//}


//fn execute_onbuild() {
    //println!("Executing ONBUILD directive");
//}

//fn execute_stopsignal() {
    //println!("Executing STOPSIGNAL directive");
//}

//fn execute_healthcheck() {
    //println!("Executing HEALTHCHECK directive");
//}

//fn execute_shell() {
    //println!("Executing SHELL directive");
//}

//fn execute_maintainer() {
    //println!("Executing MAINTAINER directive");
//}
