use crate::{
    attribute::{Attribute, ToAttribute},
    AttributeGetter, PreBaseState, Style, StyleBaseState,
};

simple_attr::define_attributes!(
align-content "how flex lines are distributed along the cross axis in a flexbox container."
:stretch|center|flex-start|flex-end|space-between|space-around|space-evenly|initial|inherit;

align-items "the default alignment for items inside a flexbox or grid container."
:stretch|center|flex-start|flex-end|start|end|baseline|initial|inherit;

align-self "the alignment in the block direction for the selected item inside a flexbox or grid container."
:auto|stretch|center|flex-start|flex-end|baseline|initial|inherit;

all "resets all properties, apart from unicode-bidi and direction, to their initial or inherited value."
:initial|inherit|unset;

animation-direction "defines whether an animation should be played forward, backward or in alternate cycles."
:normal|reverse|alternate|alternate-reverse|initial|inherit;

animation-fill-mode "a style for the element when the animation is not playing (before it starts, after it ends, or both)."
:none|forwards|backwards|both|initial|inherit;

animation-play-state "whether the animation is running or paused."
:paused|running|initial|inherit;

backface-visibility "defines whether or not the back face of an element should be visible when facing the user."
:visible|hidden|initial|inherit;

background-attachment "sets whether a background image scrolls with the rest of the page, or is fixed."
:scroll|fixed|local|initial|inherit;

background-blend-mode "defines the blending mode of each background layer (color and/or image)."
:normal|multiply|screen|overlay|darken|lighten|color-dodge|saturation|color|luminosity;

background-clip "defines how far the background (color or image) should extend within an element."
:padding-box|border-box|content-box|initial|inherit;

background-origin "specifies the origin position (the background positioning area) of a background image."
=background-clip;

background-repeat "sets if/how a background image will be repeated."
:repeat-x|repeat-y|no-repeat|space|round|initial|inherit;

border-style "sets the style of an element's four borders."
:none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;

border-block-start-style "sets the style of an element's border at the start in the block direction."
=border-style;

border-block-end-style "sets the style of an element's border at the end in the block direction."
=border-style;

border-top-style "sets the style of an element's top border."
=border-style;

border-bottom-style "sets the style of an element's bottom border."
=border-style;

border-left-style "sets the style of an element's left border."
=border-style;

border-right-style "sets the style of an element's right border."
=border-style;

border-inline-style "sets the style of an element's border at the end in the inline direction."
=border-style;

border-inline-start-style "sets the style of an element's border at the start in the inline direction."
=border-style;

border-inline-end-style "sets the style of an element's border at the end in the inline direction."
=border-style;

border-collapse "sets whether table borders should collapse into a single border or be separated as in standard HTML."
:separate|collapse|initial|inherit;

box-decoration-break "how the background, padding, border, border-image, box-shadow, margin, and clip-path of an element is applied when the box for the element is fragmented."
:slice|clone|initial|inherit|unset;

box-sizing "how the width and height of an element are calculated: should they include padding and borders, or not."
:content-box|border-box|initial|inherit;

break-after "whether or not a page break, column break, or region break should occur after the specified element."
:auto|all|always|avoid|avoid-column|avoid-page|avoid-region|column|left|page|recto|region|right|verso|initial|inherit;

break-before "whether or not a page break, column break, or region break should occur before the specified element."
=break-after;

break-inside "whether or not a page break, column break, or region break should occur inside the specified element."
=break-after;

caption-side "the placement of a table caption."
:top|bottom|initial|inherit;

clear "controls the flow next to floated elements."
:none|left|right|both|initial|inherit;

color-scheme "indicates which operating system color scheme an element should render with. "
:normal|light|dark|only;

column-fill "specifies how to fill columns, balanced or not."
:balance|auto|initial|inherit;

column-rule-style "specifies the style of the rule between columns."
=border-style;

column-span "how many columns an element should span across."
:none|all|initial|inherit;

cursor "the mouse cursor to be displayed when pointing over an element."
:alias|all-scroll|auto|cell|context-menu|col-resize|copy|crosshair|e-resize|ew-resize|grab|grabbing|help|n-resize|ne-resize|nesw-resize|ns-resize|nw-resize|nwse-resize|no-drop|none|not-allowed|pointer|progress|row-resize|s-resize|se-resize|sw-resize|text|vertical-text|w-resize|wait|zoom-in|zoom-out|initial|default_|move_;

direction "the text direction/writing direction within a block-level element."
:ltr|rtl|initial|inherit;

display "the display behavior (the type of rendering box) of an element."
:inline|block|inline-block|flex|contents|inline-flex|grid|inline-grid|table|inline-table|list-item|none|run-in;

empty-cells "whether or not to display borders on empty cells in a table."
:show|hide|initial|inherit;

flex-direction "the direction of the flexible items."
:row|row-reverse|column|column-reverse|initial|inherit;

flex-wrap "the flexible items should wrap or not."
:nowrap|wrap|wrap-reverse|initial|inherit;

float "whether an element should float to the left, right, or not at all."
:none|left|right|initial|inherit;

font-kerning "controls the usage of the kerning information stored in a font."
:auto|normal|none;

font-stretch "make text narrower (condensed) or wider (expanded)."
:ultra-condensed|extra-condensed|condensed|semi-condensed|normal|semi-expanded|expanded|extra-expanded|ultra-expanded|initial|inherit;

font-style "the font style for a text."
:normal|italic|oblique|initial|inherit;

font-variant "whether or not a text should be displayed in a small-caps font."
:normal|small-caps|initial|inherit;

font-variant-caps "controls the usage of alternate glyphs for capital letters."
:normal|small-caps|all-small-caps|petite-caps|all-petite-caps|unicase|titling-caps|initial|inherit|unset;

font-weight "how thick or thin characters in text should be displayed."
:normal|bold|bolder|lighter|number|initial|inherit;

hanging-punctuation "whether a punctuation mark may be placed outside the line box at the start or at the end of a full line of text."
:none|first|last|allow-end|force-end|initial|inherit;

hyphens "whether hyphenation is allowed to create more soft wrap opportunities within a line of text."
:none|manual|auto|initial|inherit;

image-rendering "the type of algorithm to be used for image scaling."
:auto|smooth|high-quality|crisp-edges|pixelated|initial|inherit;

isolation "whether an element must create a new stacking content."
:auto|isolate|initial|inherit;

justify-content "aligns the flexible container's items when the items do not use all available space on the main-axis (horizontally)."
:flex-start|flex-end|center|space-between|space-around|space-evenly|initial|inherit;

list-style-position "the position of the list-item markers (bullet points)."
:inside|outside|initial|inherit;

list-style-type "the type of list-item marker in a list."
:disc|circle|square|decimal|decimal-leading-zero|lower-alpha|lower-greek|lower-latin|lower-roman|upper-alpha|upper-greek|upper-latin|upper-roman|armenian|cjk-ideographic|georgian|hebrew|hiragana|hiragana-iroha|katakana|katakana-iroha|none;

mask-clip "which area is affected by a mask image."
:border-box|content-box|padding-box|fill-box|stroke-box|view-box|no-clip|border|padding|content|text|initial|inherit;

mask-composite " a compositing operation used on the current mask layer with the mask layers below it."
:add|subtract|intersect|exclude|initial|inherit;

);
