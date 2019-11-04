	org 32768

ENUM Gadget 10,2
    None
    Button
    Box = 10
END

STRUCT mystruct
    top.b
    left
    width.w
    height.w
END

    ld  a,Gadget.None

    ld  a,(ix+mystruct.top)

    db  0,0


