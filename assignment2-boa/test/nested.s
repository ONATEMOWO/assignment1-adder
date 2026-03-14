section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 1
mov [rsp - 16], rax
mov rax, 2
mov [rsp - 24], rax
mov rax, [rsp - 16]
mov [rsp - 32], rax
mov rax, [rsp - 24]
add rax, [rsp - 32]
mov [rsp - 32], rax
mov rax, [rsp - 32]
mov [rsp - 40], rax
mov rax, [rsp - 32]
imul rax, [rsp - 40]
  ret
