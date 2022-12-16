BITS 64

NT_WRITE_FILE_SYSCALL_ID: equ 8

sub rsp, 80

push 0x60
pop rax

mov r10, gs:[rax]
mov r10, [r10 + 0x20]
mov r10, [r10 + 0x28]

cdq
xor r8, r8
xor r9, r9
mov qword [rsp + 0x28], rsp

call message_label
message: db 'hello from shellcode!'
message_label: pop rax
mov qword [rsp + 0x30], rax

message_length: equ message_label - message
push message_length
pop rax
mov dword [rsp + 0x38], eax

mov qword [rsp + 0x40], 0
mov qword [rsp + 0x48], 0

push NT_WRITE_FILE_SYSCALL_ID
pop rax

syscall

xor eax, eax

add rsp, 80

ret
