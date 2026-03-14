section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 10
mov [rsp - 16], rax
mov rax, 3
mov rbx, [rsp - 16]
sub rbx, rax
mov rax, rbx
  ret
