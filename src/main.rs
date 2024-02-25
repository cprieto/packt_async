use core::arch::asm;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[inline(never)]
fn linux_raw_syscall(message: String) {
    let message_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") message_ptr,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _,
        );
    }
}

fn main() {
    let message = String::from("Hello world from raw syscall\n");
    linux_raw_syscall(message);
}
