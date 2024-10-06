use crate::{color::ColorAttribute, length::LengthAttribute, AttrValue, Attributer, Home, Styling};

simple_attr::define_attributes!(
accent-color r#"the accent color for user-interface controls like: <input type="checkbox">, <input type="radio">, <input type="range"> and <progress>."#,
background-color "the background color of an element.",
border-color "the color of an element's four borders. This property can have from one to four values.",
border-block-color "color of an element's borders in the block direction.",
border-left-color "the color of an element's left border.",
border-right-color "the color of an element's right border.",
border-top-color "the color of an element's top border.",
border-bottom-color "the color of an element's bottom border.",
caret-color "color of the cursor in input elements:",
outline-color "specifies the color of an outline.",
color "specifies the color of text."
are color;
font-size,top,bottom,left,right,margin,height,width,padding are length;

border-style "sets the style of an element's four borders.",
border-block-start-style "sets the style of an element's border at the start in the block direction.",
border-block-end-style "sets the style of an element's border at the end in the block direction.",
border-top-style "sets the style of an element's top border.",
border-bottom-style "sets the style of an element's bottom border.",
border-left-style "sets the style of an element's left border.",
border-right-style "sets the style of an element's right border.",
border-inline-style "sets the style of an element's border at the end in the inline direction.",
border-inline-start-style "sets the style of an element's border at the start in the inline direction.",
border-inline-end-style "sets the style of an element's border at the end in the inline direction.",
outline-style "the style of an outline.",
column-rule-style "specifies the style of the rule between columns."
:none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset;

align-content "how flex lines are distributed along the cross axis in a flexbox container."
:stretch|center|flex-start|flex-end|space-between|space-around|space-evenly;

align-items "the default alignment for items inside a flexbox or grid container."
:stretch|center|flex-start|flex-end|start|end|baseline;

align-self "the alignment in the block direction for the selected item inside a flexbox or grid container."
:auto|stretch|center|flex-start|flex-end|baseline;

all "resets all properties, apart from unicode-bidi and direction, to their initial or inherited value."
:initial|inherit|unset;

animation-direction "defines whether an animation should be played forward, backward or in alternate cycles."
:normal|reverse|alternate|alternate-reverse;

animation-fill-mode "a style for the element when the animation is not playing (before it starts, after it ends, or both)."
:none|forwards|backwards|both;

animation-play-state "whether the animation is running or paused."
:paused|running;

backface-visibility "defines whether or not the back face of an element should be visible when facing the user."
:visible|hidden;

background-attachment "sets whether a background image scrolls with the rest of the page, or is fixed."
:scroll|fixed|local;

background-blend-mode "defines the blending mode of each background layer (color and/or image)."
:normal|multiply|screen|overlay|darken|lighten|color-dodge|saturation|color|luminosity;

background-clip "defines how far the background (color or image) should extend within an element.",
background-origin "specifies the origin position (the background positioning area) of a background image."
:padding-box|border-box|content-box;

background-repeat "sets if/how a background image will be repeated."
:repeat-x|repeat-y|no-repeat|space|round;

border-collapse "sets whether table borders should collapse into a single border or be separated as in standard HTML."
:separate|collapse;

box-decoration-break "how the background, padding, border, border-image, box-shadow, margin, and clip-path of an element is applied when the box for the element is fragmented."
:slice|clone|unset;

box-sizing "how the width and height of an element are calculated: should they include padding and borders, or not."
:content-box|border-box;

break-after "whether or not a page break, column break, or region break should occur after the specified element.",
break-before "whether or not a page break, column break, or region break should occur before the specified element.",
break-inside "whether or not a page break, column break, or region break should occur inside the specified element."
:auto|all|always|avoid|avoid-column|avoid-page|avoid-region|column|left|page|recto|region|right|verso;

caption-side "the placement of a table caption."
:top|bottom;

clear "controls the flow next to floated elements."
:none|left|right|both;

color-scheme "indicates which operating system color scheme an element should render with. "
:normal|light|dark|only;

column-fill "specifies how to fill columns, balanced or not."
:balance|auto;


column-span "how many columns an element should span across."
:none|all;

cursor "the mouse cursor to be displayed when pointing over an element."
:alias|all-scroll|auto|cell|context-menu|col-resize|copy|crosshair|e-resize|ew-resize|grab|grabbing|help|n-resize|ne-resize|nesw-resize|ns-resize|nw-resize|nwse-resize|no-drop|none|not-allowed|pointer|progress|row-resize|s-resize|se-resize|sw-resize|text|vertical-text|w-resize|wait|zoom-in|zoom-out|initial|default_|move_;

direction "the text direction/writing direction within a block-level element."
:ltr|rtl;

display "the display behavior (the type of rendering box) of an element."
:inline|block|inline-block|flex|contents|inline-flex|grid|inline-grid|table|inline-table|list-item|none|run-in;

empty-cells "whether or not to display borders on empty cells in a table."
:show|hide;

flex-direction "the direction of the flexible items."
:row|row-reverse|column|column-reverse;

flex-wrap "the flexible items should wrap or not."
:nowrap|wrap|wrap-reverse;

float "whether an element should float to the left, right, or not at all."
:none|left|right;

font-kerning "controls the usage of the kerning information stored in a font."
:auto|normal|none;

font-stretch "make text narrower (condensed) or wider (expanded)."
:ultra-condensed|extra-condensed|condensed|semi-condensed|normal|semi-expanded|expanded|extra-expanded|ultra-expanded;

font-style "the font style for a text."
:normal|italic|oblique;

font-variant "whether or not a text should be displayed in a small-caps font."
:normal|small-caps;

font-variant-caps "controls the usage of alternate glyphs for capital letters."
:normal|small-caps|all-small-caps|petite-caps|all-petite-caps|unicase|titling-caps|unset;

font-weight "how thick or thin characters in text should be displayed."
:normal|bold|bolder|lighter|number;

hanging-punctuation "whether a punctuation mark may be placed outside the line box at the start or at the end of a full line of text."
:none|first|last|allow-end|force-end;

hyphens "whether hyphenation is allowed to create more soft wrap opportunities within a line of text."
:none|manual|auto;

image-rendering "the type of algorithm to be used for image scaling."
:auto|smooth|high-quality|crisp-edges|pixelated;

isolation "whether an element must create a new stacking content."
:auto|isolate;

justify-content "aligns the flexible container's items when the items do not use all available space on the main-axis (horizontally)."
:flex-start|flex-end|center|space-between|space-around|space-evenly;

list-style-position "the position of the list-item markers (bullet points)."
:inside|outside;

list-style-type "the type of list-item marker in a list."
:disc|circle|square|decimal|decimal-leading-zero|lower-alpha|lower-greek|lower-latin|lower-roman|upper-alpha|upper-greek|upper-latin|upper-roman|armenian|cjk-ideographic|georgian|hebrew|hiragana|hiragana-iroha|katakana|katakana-iroha|none;

mask-clip "which area is affected by a mask image."
:border-box|content-box|padding-box|fill-box|stroke-box|view-box|no-clip|border|padding|content|text;

mask-composite "a compositing operation used on the current mask layer with the mask layers below it."
:add|subtract|intersect|exclude;

mask-mode "whether the mask layer image should be treated as a luminance mask or as an alpha mask."
:match-source|luminance|alpha;

mask-origin "the origin position (the mask position area) of a mask layer image."
:border-box|content-box|padding-box|fill-box|stroke-box|view-box;

mask-repeat "sets if/how a mask image will be repeated."
:repeat|repeat-x|repeat-y|space|round|no-repeat;

mask-type "whether an SVG <mask> element is treated as a luminance mask or as an alpha mask."
:luminance|alpha;

mix-blend-mode "how an element's content should blend with its direct parent background."
:normal|multiply|screen|overlay|darken|lighten|color-dodge|color-burn|difference|exclusion|hue|saturation|color|luminosity;

object-fit "how an <img> or <video> should be resized to fit its container."
:fill|contain|cover|scale-down|none;

object-position "used together with object-fit to specify how an <img> or <video> should be positioned with x/y coordinates inside its 'own content box'."
:position;


overflow "what should happen if content overflows an element's box."
:visible|hidden|clip|scroll|auto;

overflow-anchor "makes it possible to turn off scroll anchoring."
:auto|none;

overflow-wrap "whether or not the browser can break lines with long words, if they overflow the container."
:normal|anywhere|break-word;

overflow-x "whether to clip the content, add a scroll bar, or display overflow content of a block-level element, when it overflows at the left and right ",
overflow-y "whether to clip the content, add a scroll bar, or display overflow content of a block-level element, when it overflows at the top and bottom edges."
:visible|hidden|scroll|auto;

overscroll-behavior "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary.",
overscroll-behavior-block "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the block direction.",
overscroll-behavior-inline "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the inline direction.",
overscroll-behavior-x "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the x-direction.",
overscroll-behavior-y "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in y-direction.",
:auto|contain|none;

place-self "align individual grid items, and is a shorthand property for the following properties:"
:auto|value;

pointer-events "whether or not an element reacts to pointer events."
:auto|none;

position "positioning method used for an element (static, relative, absolute, fixed, or sticky)."
:static_|absolute|fixed|relative|sticky;

resize "if (and how) an element is resizable by the user."
:none|both|horizontal|vertical;

scroll-behavior "whether to smoothly animate the scroll position, instead of a straight jump, when the user clicks on a link within a scrollable box."
:auto|smooth;

scroll-snap-stop "whether to scroll past elements or to stop and snap on the next element."
:normal|always;

table-layout "the algorithm used to lay out table cells, rows, and columns."
:auto|fixed;

text-align "the horizontal alignment of text in an element."
:left|right|center|justify;

text-align-last "alignment for all last lines within the selected element. So, if you have a <div> with three paragraphs in it."
:auto|left|right|center|justify|start|end;

text-decoration-line "the kind of text decoration to use (like underline, overline, line-through)."
:none|underline|overline|line-through;

text-decoration-style "style of the text decoration (like solid, wavy, dotted, dashed, double)."
:solid|double|dotted|dashed|wavy;

text-emphasis "apply emphasis marks to text."
:none|filled|open|dot|circle|double-circle|triangle|sesame|string|color;

text-emphasis-position "the position of the emphasis marks (over, under, left, right)."
:over|under|left|right;

text-emphasis-style "the style of emphasis marks."
:none|filled|open|dot|circle|double-circle|triangle|sesame|color;

text-justify "the justification method of text when text-align is set to 'justify'."
:auto|inter-word|inter-character|none;

text-orientation "the orientation of characters."
:mixed|upright|sideways|sideways-right|use-glyph-orientation;

text-overflow "how overflowed content that is not displayed should be signaled to the user. It can be clipped, display an ellipsis (...), or display a custom string."
:clip|ellipsis|string;

text-transform "controls the capitalization of text."
:none|capitalize|uppercase|lowercase;

text-underline-position "the position of underline text decorations."
:auto|under|from-font|left|right;

transform-style "how nested elements are rendered in 3D space."
:flat|preserve-3d;

unicode-bidi "together with the direction property to set or return whether the text should be overridden to support multiple languages in the same document."
:normal|embed|bidi-override;

user-select "whether the text of an element can be selected."
:auto|none|text|all;

vertical-align "the vertical alignment of an element."
:baseline|length|sub|super_|top|text-top|middle|bottom|text-bottom;

visibility "whether or not an element is visible."
:visible|hidden|collapse;

white-space "how white-space inside an element is handled."
:normal|nowrap|pre|pre-line|pre-wrap;

word-break "how words should break when reaching the end of a line."
:normal|break-all|keep-all|break-word;

word-wrap "allows long words to be able to be broken and wrap onto the next line."
:normal|break-word;

writing-mode "whether lines of text are laid out horizontally or vertically."
:horizontal-tb|vertical-rl|vertical-lr;
);
