section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 8
mov [rsp - 16], rax
mov rax, [rsp - 16]
sub rax, 1
  ret
