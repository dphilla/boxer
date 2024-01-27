use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use dockerfile_parser::*;


mod builder {
    pub mod builder;
}

use builder::builder::*;

fn main() {

// TODO: check validity first
let dockerfile = Dockerfile::parse(r#"
FROM ruby:3.0

WORKDIR /usr/src/app

COPY . .

CMD ["./your-script.rb"]
"#).unwrap();

//FROM python:3

//WORKDIR /usr/src/app

//COPY . .

//CMD [ "python", "./your-script.py" ]



let mut builder = Builder::new();
for stage in dockerfile.iter_stages() {
  for ins in stage.instructions {
    match ins {
        Instruction::From(instr) => execute_from(&mut builder, instr.clone()),
        Instruction::Arg(instr) => execute_arg(instr.clone()),
        Instruction::Label(instr) => execute_label(instr.clone()),
        Instruction::Run(instr) => execute_run(instr.clone()),
        Instruction::Entrypoint(instr) => execute_entrypoint(instr.clone()),
        Instruction::Cmd(instr) => execute_cmd(instr.clone()),
        Instruction::Copy(instr) => execute_copy(instr.clone()),
        Instruction::Env(instr) => execute_env(instr.clone()),
        Instruction::Misc(instr) => execute_misc(instr.clone()),
    }
  }

  builder.build(true);
}

}

fn execute_from(builder: &mut Builder, instr: FromInstruction) {
    builder.config_base(&instr.image.content);

}

fn execute_run(instr: RunInstruction) {
}

fn execute_cmd(instr: CmdInstruction) {
}

fn execute_label(instr: LabelInstruction) {
}

//fn execute_expose(instr: ExposeInstruction) {
//}

fn execute_env(instr: EnvInstruction) {
}

//fn execute_add(instr: AddInstruction) {
//}

fn execute_copy(instr: CopyInstruction) {
}

fn execute_entrypoint(instr: EntrypointInstruction) {
}

fn execute_arg(instr: ArgInstruction) {
}

fn execute_misc(instr: MiscInstruction) {
    //TODO: handles: `MAINTAINER`, `EXPOSE`, `VOLUME`,
        // `USER`, `WORKDIR`, `ONBUILD`, `STOPSIGNAL`, `HEALTHCHECK`, `SHELL`
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
