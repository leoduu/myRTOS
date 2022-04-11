

use crate::{kernel::thread::Thread, println};

const SHELL_NUM: usize = 3;
const SHELL_STACK_SIZE: usize = 5 * 1024;
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

    SHELL.init("shell", shell_entry, SHELL_STACK.as_ptr() as usize, SHELL_STACK_SIZE);

}

pub fn shell_entry() -> ! {

    println!(">>>>>>>> console <<<<<<<<");

    loop {
        
    }
}

fn shell_show_commands() {
    // println!();
    // for i in 0..SHELL_NUM {
    //     print!("{} ", SHELL_COMMAND[i]);
    // }
}

fn shell_command_memory() {
    
}

fn shell_command_lsof() {
    // scheduler().lock(|sc| println!("{}", sc));
}

