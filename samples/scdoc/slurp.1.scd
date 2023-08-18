slurp(1)

# NAME

slurp - select a region in a Wayland compositor

# SYNOPSIS

_slurp_ [options...]

# DESCRIPTION

slurp is a command-line utility to select a region from Wayland compositors
which support the layer-shell protocol. It lets the user hold the pointer to
select, or click to cancel the selection.

If the standard input is not a TTY or the -r option is used, slurp will read a
list of predefined rectangles for quick selection. Each line must be in the form
"<x>,<y> <width>x<height> [label]". The label is optional and can be any string
that doesn't contain newlines. It can be accessed using the "%l" sequence in a
format string.

If the _Esc_ key is pressed, selection is cancelled. If the _Space_ key is
held, the selection is moved instead of being resized.

# OPTIONS

_-h_
Show help message and quit.

_-d_
Display dimensions of selection.

_-b_ _color_
Set background color. See _COLORS_ for more detail.

_-c_ _color_
Set border color. See _COLORS_ for more detail.

_-s_ _color_
Set selection color. See _COLORS_ for more detail.

_-B_ _color_
Set color for highlighting predefined rectangles from standard input when not
selected.

_-F_ _font family_
Set the font family name when displaying the dimensions box. Only useful
when combined with the -d option. The available font family names guaranteed
to work are the standard generic CSS2 options: serif, sans-serif,
monospace, cursive and fantasy. It defaults to the sans-serif family name.

_-w_ _weight_
Set border weight.

_-f_ _format_
Set format. See _FORMAT_ for more detail.

_-p_
Select a single pixel instead of a rectangle. This mode ignores any
predefined rectangles read from the standard input.

_-o_
Add predefined rectangles for all outputs, as if provided on standard input.
The label will be the name of the output.

_-r_
Require the user to select one of the predefined rectangles. These can come
from standard input, if _-o_ is used, the rectangles of all display outputs.
This option conflicts with _-p_.

_-a_ _width_:_height_
Force selections to have the given aspect ratio. This constraint is not
applied to the predefined rectangles specified using _-o_.

# COLORS

Colors may be specified in #RRGGBB or #RRGGBBAA format. The # is optional.

# FORMAT

Interpreted sequences are:

%x The x-coordinate of the selection

%y The y-coordinate of the selection

%w The width of the selection

%h The height of the selection

%X The x-coordinate of the selection with coordinates relative to the output
containing the top left corner.

%Y The y-coordinate of the selection with coordinates relative to the output
containing the top left corner.

%W The width of the selection cropped to the output containing the top left
corner.

%H The height of the selection cropped to the output containing the top left
corner.

%l Label included with region from stdin

%o The name of the output containing the top left corner, or "<unknown>" if
not known

The default format is "%x,%y %wx%h\\n".

# KEYBOARD CONTROLS

The following keyboard actions can be used during selection:

_Escape_ Cancel the selection and exit slurp

_Space_ If currently making a selection, while space is held down, move the
entire selection rather than change the selection's size as you move the
pointer.

_Shift_ Experimental. If the _-a_ option wasn't specified, then set the aspect
ratio to 1:1 while shift is held down, releasing it restores the un-constrained
aspect ratio. _Note:_ This behavior may change in the future depending on
feedback.

# AUTHORS

Maintained by Simon Ser <contact@emersion.fr>, who is assisted by other
open-source contributors. For more information about slurp development, see
https://github.com/emersion/slurp.
