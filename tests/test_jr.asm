    org 32768

start   jr  start
        jr  finish
        jr  .middle
.middle jr  .middle
        jr  .middle
        jr  finish
        jr  start
finish  ret

startz  jr  z,startz
        jr  z,finishz
        jr  z,.middlez
.middlez
        jr  z,.middlez
        jr  z,.middlez
        jr  z,finishz
        jr  z,startz
finishz ret