section .data
    yesMsg db "Yes", 0
    noMsg  db "No", 0
    badMsg db "-", 0

section .text
    global check_palindrome3

; Вход: eax = число
; Выход: rax = адрес строки результата
check_palindrome3:
    cmp eax, 100
    jl not_three
    cmp eax, 999
    jg not_three

    mov ebx, eax
    mov ecx, 100
    xor edx, edx
    div ecx
    mov esi, eax        ; сотни

    mov eax, ebx
    xor edx, edx
    mov ecx, 10
    div ecx
    mov eax, edx        ; единицы

    cmp esi, eax
    je is_pal

    lea rax, [rel noMsg]
    ret

is_pal:
    lea rax, [rel yesMsg]
    ret

not_three:
    lea rax, [rel badMsg]
    ret
