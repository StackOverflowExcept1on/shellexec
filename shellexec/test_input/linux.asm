BITS 64

; sys_write(fd, buf, count)
push 0x01
pop rax

; fd = stdout
push 0x01
pop rdi

; buf = "hello from shellcode!"
call message_label
message: db 'hello from shellcode!'
message_label: pop rsi

; count = 21
message_length: equ message_label - message
push message_length
pop rdx

syscall

; sys_exit(error_code)
push 0x3c
pop rax

; error_code
push 0
pop rdi

syscall
