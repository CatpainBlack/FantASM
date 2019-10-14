	org 32768

const_x = 0
const_y = 1
const_z = 0

if  const_x = 0
    if const_y = 0
        ld hl,1
    else
        ld  hl,2
    endif
else
    if const_z = 0
        ld hl,3
    else
        ld  hl,4
    endif
endif

