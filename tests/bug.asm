	org 32768


STRUCT	Window
	top.b
	left.b
	width.b
	height.b
	colour.b
	title.w
END

w_title:    dz  "WRD"

w:  Window 1,2,3,4,5,w_title


