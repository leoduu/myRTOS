
use crate::utilities::intrusive_linkedlist::ListPtr;
use crate::kernel::thread::Thread;
use crate::{println, print};
use alloc::string::String;

const SHELL_PRIO: u8 = 31;
const SHELL_NUM: usize = 3;
const SHELL_STACK_SIZE: usize = 5120;
#[link_section = ".bss.stack_shell"]
static mut SHELL_STACK:[u8; SHELL_STACK_SIZE] = [0; SHELL_STACK_SIZE];

pub static mut SHELL: Thread = Thread::new();


static SHELL_COMMAND: [&'static str; SHELL_NUM] = [
    "cmd",
    "mem", 
    "lsof",
];
static SHELL_HANDLE:[fn(); SHELL_NUM] = [
    shell_show_commands,
    shell_command_memory, 
    shell_command_lsof,
];

pub unsafe fn init() {

    SHELL.init("shell", SHELL_PRIO, shell_entry, 
                SHELL_STACK.as_ptr() as usize, SHELL_STACK_SIZE);

}

pub fn shell_entry(_ptr: ListPtr) -> ! {

    print!(">>>>>>>> console <<<<<<<<\n\n>> ");

    let mut buff = String::new();

    loop {

        use crate::kernel::syscall::*;
        let ch = syscall2_1(SysCallClass::HardwareAccess, 
                                    SysCallDevice::UartRx as usize);

        if ch != 0 {
            let c = unsafe {char::from_u32_unchecked(ch as u32)};

            if c == '\r' || c == '\n' {
                println!();
                shell_prase(&buff);
                buff.clear();
                print!(">> ");  
            } else {
                buff.push(c);
                print!("{}", c);
            }
        }
    }
}

fn shell_prase(buff: &String) {

    if buff.is_empty() {
        return;
    }

    for (i, cmd) in SHELL_COMMAND.iter().enumerate() {
        if buff.eq(*cmd) {
            SHELL_HANDLE[i]();
            return;
        } 
    }

    println!("{}: command not found", buff.as_str());   
}

fn shell_show_commands() {
    print!("all commands: ");
    for i in 0..SHELL_NUM {
        print!("{} ", SHELL_COMMAND[i]);
    }
    println!();
}

fn shell_command_memory() {
    println!("{:?}", crate::mem::allocator());
}

fn shell_command_lsof() {
    println!("{}", crate::kernel::sched::scheduler());
}
