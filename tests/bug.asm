	org 32768

STRUCT rect
    top.b
    left
    width.w
    height.w
END

    ld  hl,window.top

window  rect 1,2,3,4
