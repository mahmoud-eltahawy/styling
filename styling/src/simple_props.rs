use crate::{color::ColorAttribute, length::LengthAttribute, AttrValue, Attributer, Home, Styling};

simple_attr::define_attributes!(
accent_color r#"the accent color for user_interface controls like: <input type="checkbox">, <input type="radio">, <input type="range"> and <progress>."#,
background_color "the background color of an element.",
border_color "the color of an element's four borders. This property can have from one to four values.",
border_block_color "color of an element's borders in the block direction.",
border_left_color "the color of an element's left border.",
border_right_color "the color of an element's right border.",
border_top_color "the color of an element's top border.",
border_bottom_color "the color of an element's bottom border.",
caret_color "color of the cursor in input elements:",
outline_color "specifies the color of an outline.",
color "specifies the color of text."
are color;
font_size,top,bottom,left,right,margin,height,width,padding are length;

border_style "sets the style of an element's four borders.",
border_block_start_style "sets the style of an element's border at the start in the block direction.",
border_block_end_style "sets the style of an element's border at the end in the block direction.",
border_top_style "sets the style of an element's top border.",
border_bottom_style "sets the style of an element's bottom border.",
border_left_style "sets the style of an element's left border.",
border_right_style "sets the style of an element's right border.",
border_inline_style "sets the style of an element's border at the end in the inline direction.",
border_inline_start_style "sets the style of an element's border at the start in the inline direction.",
border_inline_end_style "sets the style of an element's border at the end in the inline direction.",
outline_style "the style of an outline.",
column_rule_style "specifies the style of the rule between columns."
:none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset;

align_content "how flex lines are distributed along the cross axis in a flexbox container."
:stretch|center|flex_start|flex_end|space_between|space_around|space_evenly;

align_items "the default alignment for items inside a flexbox or grid container."
:stretch|center|flex_start|flex_end|start|end|baseline;

align_self_ "the alignment in the block direction for the selected item inside a flexbox or grid container."
:auto|stretch|center|flex_start|flex_end|baseline;

all "resets all properties, apart from unicode_bidi and direction, to their initial or inherited value."
:initial|inherit|unset;

animation_direction "defines whether an animation should be played forward, backward or in alternate cycles."
:normal|reverse|alternate|alternate_reverse;

animation_fill_mode "a style for the element when the animation is not playing (before it starts, after it ends, or both)."
:none|forwards|backwards|both;

animation_play_state "whether the animation is running or paused."
:paused|running;

backface_visibility "defines whether or not the back face of an element should be visible when facing the user."
:visible|hidden;

background_attachment "sets whether a background image scrolls with the rest of the page, or is fixed."
:scroll|fixed|local;

background_blend_mode "defines the blending mode of each background layer (color and/or image)."
:normal|multiply|screen|overlay|darken|lighten|color_dodge|saturation|color|luminosity;

background_clip "defines how far the background (color or image) should extend within an element.",
background_origin "specifies the origin position (the background positioning area) of a background image."
:padding_box|border_box|content_box;

background_repeat "sets if/how a background image will be repeated."
:repeat_x|repeat_y|no_repeat|space|round;

border_collapse "sets whether table borders should collapse into a single border or be separated as in standard HTML."
:separate|collapse;

box_decoration_break "how the background, padding, border, border_image, box_shadow, margin, and clip_path of an element is applied when the box for the element is fragmented."
:slice|clone|unset;

box_sizing "how the width and height of an element are calculated: should they include padding and borders, or not."
:content_box|border_box;

break_after "whether or not a page break, column break, or region break should occur after the specified element.",
break_before "whether or not a page break, column break, or region break should occur before the specified element.",
break_inside "whether or not a page break, column break, or region break should occur inside the specified element."
:auto|all|always|avoid|avoid_column|avoid_page|avoid_region|column|left|page|recto|region|right|verso;

caption_side "the placement of a table caption."
:top|bottom;

clear "controls the flow next to floated elements."
:none|left|right|both;

color_scheme "indicates which operating system color scheme an element should render with. "
:normal|light|dark|only;

column_fill "specifies how to fill columns, balanced or not."
:balance|auto;


column_span "how many columns an element should span across."
:none|all;

cursor "the mouse cursor to be displayed when pointing over an element."
:alias|all_scroll|auto|cell|context_menu|col_resize|copy|crosshair|e_resize|ew_resize|grab|grabbing|help|n_resize|ne_resize|nesw_resize|ns_resize|nw_resize|nwse_resize|no_drop|none|not_allowed|pointer|progress|row_resize|s_resize|se_resize|sw_resize|text|vertical_text|w_resize|wait|zoom_in|zoom_out|initial|default_|move_;

direction "the text direction/writing direction within a block_level element."
:ltr|rtl;

display "the display behavior (the type of rendering box) of an element."
:inline|block|inline_block|flex|contents|inline_flex|grid|inline_grid|table|inline_table|list_item|none|run_in;

empty_cells "whether or not to display borders on empty cells in a table."
:show|hide;

flex_direction "the direction of the flexible items."
:row|row_reverse|column|column_reverse;

flex_wrap "the flexible items should wrap or not."
:nowrap|wrap|wrap_reverse;

float "whether an element should float to the left, right, or not at all."
:none|left|right;

font_kerning "controls the usage of the kerning information stored in a font."
:auto|normal|none;

font_stretch "make text narrower (condensed) or wider (expanded)."
:ultra_condensed|extra_condensed|condensed|semi_condensed|normal|semi_expanded|expanded|extra_expanded|ultra_expanded;

font_style "the font style for a text."
:normal|italic|oblique;

font_variant "whether or not a text should be displayed in a small_caps font."
:normal|small_caps;

font_variant_caps "controls the usage of alternate glyphs for capital letters."
:normal|small_caps|all_small_caps|petite_caps|all_petite_caps|unicase|titling_caps|unset;

font_weight "how thick or thin characters in text should be displayed."
:normal|bold|bolder|lighter|number;

hanging_punctuation "whether a punctuation mark may be placed outside the line box at the start or at the end of a full line of text."
:none|first|last|allow_end|force_end;

hyphens "whether hyphenation is allowed to create more soft wrap opportunities within a line of text."
:none|manual|auto;

image_rendering "the type of algorithm to be used for image scaling."
:auto|smooth|high_quality|crisp_edges|pixelated;

isolation "whether an element must create a new stacking content."
:auto|isolate;

justify_content "aligns the flexible container's items when the items do not use all available space on the main_axis (horizontally)."
:flex_start|flex_end|center|space_between|space_around|space_evenly;

list_style_position "the position of the list_item markers (bullet points)."
:inside|outside;

list_style_type "the type of list_item marker in a list."
:disc|circle|square|decimal|decimal_leading_zero|lower_alpha|lower_greek|lower_latin|lower_roman|upper_alpha|upper_greek|upper_latin|upper_roman|armenian|cjk_ideographic|georgian|hebrew|hiragana|hiragana_iroha|katakana|katakana_iroha|none;

mask_clip "which area is affected by a mask image."
:border_box|content_box|padding_box|fill_box|stroke_box|view_box|no_clip|border|padding|content|text;

mask_composite "a compositing operation used on the current mask layer with the mask layers below it."
:add|subtract|intersect|exclude;

mask_mode "whether the mask layer image should be treated as a luminance mask or as an alpha mask."
:match_source|luminance|alpha;

mask_origin "the origin position (the mask position area) of a mask layer image."
:border_box|content_box|padding_box|fill_box|stroke_box|view_box;

mask_repeat "sets if/how a mask image will be repeated."
:repeat|repeat_x|repeat_y|space|round|no_repeat;

mask_type "whether an SVG <mask> element is treated as a luminance mask or as an alpha mask."
:luminance|alpha;

mix_blend_mode "how an element's content should blend with its direct parent background."
:normal|multiply|screen|overlay|darken|lighten|color_dodge|color_burn|difference|exclusion|hue|saturation|color|luminosity;

object_fit "how an <img> or <video> should be resized to fit its container."
:fill|contain|cover|scale_down|none;

object_position "used together with object_fit to specify how an <img> or <video> should be positioned with x/y coordinates inside its 'own content box'."
:position;


overflow "what should happen if content overflows an element's box."
:visible|hidden|clip|scroll|auto;

overflow_anchor "makes it possible to turn off scroll anchoring."
:auto|none;

overflow_wrap "whether or not the browser can break lines with long words, if they overflow the container."
:normal|anywhere|break_word;

overflow_x "whether to clip the content, add a scroll bar, or display overflow content of a block_level element, when it overflows at the left and right ",
overflow_y "whether to clip the content, add a scroll bar, or display overflow content of a block_level element, when it overflows at the top and bottom edges."
:visible|hidden|scroll|auto;

overscroll_behavior "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary.",
overscroll_behavior_block "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the block direction.",
overscroll_behavior_inline "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the inline direction.",
overscroll_behavior_x "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the x_direction.",
overscroll_behavior_y "turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in y_direction.",
:auto|contain|none;

place_self_ "align individual grid items, and is a shorthand property for the following properties:"
:auto|value;

pointer_events "whether or not an element reacts to pointer events."
:auto|none;

position "positioning method used for an element (static, relative, absolute, fixed, or sticky)."
:static_|absolute|fixed|relative|sticky;

resize "if (and how) an element is resizable by the user."
:none|both|horizontal|vertical;

scroll_behavior "whether to smoothly animate the scroll position, instead of a straight jump, when the user clicks on a link within a scrollable box."
:auto|smooth;

scroll_snap_stop "whether to scroll past elements or to stop and snap on the next element."
:normal|always;

table_layout "the algorithm used to lay out table cells, rows, and columns."
:auto|fixed;

text_align "the horizontal alignment of text in an element."
:left|right|center|justify;

text_align_last "alignment for all last lines within the selected element. So, if you have a <div> with three paragraphs in it."
:auto|left|right|center|justify|start|end;

text_decoration_line "the kind of text decoration to use (like underline, overline, line_through)."
:none|underline|overline|line_through;

text_decoration_style "style of the text decoration (like solid, wavy, dotted, dashed, double)."
:solid|double|dotted|dashed|wavy;

text_emphasis "apply emphasis marks to text."
:none|filled|open|dot|circle|double_circle|triangle|sesame|string|color;

text_emphasis_position "the position of the emphasis marks (over, under, left, right)."
:over|under|left|right;

text_emphasis_style "the style of emphasis marks."
:none|filled|open|dot|circle|double_circle|triangle|sesame|color;

text_justify "the justification method of text when text_align is set to 'justify'."
:auto|inter_word|inter_character|none;

text_orientation "the orientation of characters."
:mixed|upright|sideways|sideways_right|use_glyph_orientation;

text_overflow "how overflowed content that is not displayed should be signaled to the user. It can be clipped, display an ellipsis (...), or display a custom string."
:clip|ellipsis|string;

text_transform "controls the capitalization of text."
:none|capitalize|uppercase|lowercase;

text_underline_position "the position of underline text decorations."
:auto|under|from_font|left|right;

transform_style "how nested elements are rendered in 3D space."
:flat|preserve_3d;

unicode_bidi "together with the direction property to set or return whether the text should be overridden to support multiple languages in the same document."
:normal|embed|bidi_override;

user_select "whether the text of an element can be selected."
:auto|none|text|all;

vertical_align "the vertical alignment of an element."
:baseline|length|sub|super_|top|text_top|middle|bottom|text_bottom;

visibility "whether or not an element is visible."
:visible|hidden|collapse;

white_space "how white_space inside an element is handled."
:normal|nowrap|pre|pre_line|pre_wrap;

word_break "how words should break when reaching the end of a line."
:normal|break_all|keep_all|break_word;

word_wrap "allows long words to be able to be broken and wrap onto the next line."
:normal|break_word;

writing_mode "whether lines of text are laid out horizontally or vertically."
:horizontal_tb|vertical_rl|vertical_lr;
);
