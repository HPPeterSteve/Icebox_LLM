mod chroot;
mod directory;

/*
Autor [Peter Steve - Software Engineer]
module seccomp for LLM execute
declared module chroot
*/

extern crate libc; 
extern crate seccomp_sys;

use seccomp_sys::scmp_compare::*;
use seccomp_sys::*;
use std::convert::Into;
use std::error::Error;
use std::fmt;

pub type comparison = scmp_arg_cmp;

#[derive(Debug, Clone, Copy)]

pub enum ScmpAction {
    KillProcess,
    KillThread,
    Trap,
    Notify,
    Errno(i32),
    Trace(u16),
    Log,
    Allow,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* 
    necessario termos um filtro basico para fins de estudo, logo será estudado uma forma de
    implementar ops filtros com menos ajuda, apenas com um manual, 
    o filtro sera optimizado para que o agente LLM apenas tenha acesso ao que é minimamente seguro,
    algumas chamadas do kernel serão permitidas como: acesso a internet, porém apenas para ler.
    */
    let mut filter = ScmpFilterContext::new(ScmpAction::Allow)?; 

    /* It's important to define the architecture; 
    I opted for 64 bits,
     which is the most common architecture used by most users.
    */
    filter.add_arch(ScmpArch::X8664)?;

}

