use crate::{color::ColorAttribute, length::LengthAttribute, AttrValue, Attributer, Home, Styling};

simple_attr::define_attributes!(
accent_color r#"
the accent color for user_interface controls like: 
    \<input type="checkbox">,
    \<input type="radio">,
    \<input type="range">
    and \<progress>.
"#,
caret_color r#"
The caret-color property specifies the color of the cursor (caret) in inputs, textareas, or any element that is editable.
"#
: auto $color;

background_color r#"
The background-color property sets the background color of an element.

The background of an element is the total size of the element, including padding and border (but not the margin).

Tip: Use a background color and a text color that makes the text easy to read.
"#,
border_color r#"
The border-color property sets the color of an element's four borders. This property can have from one to four values.

If the border-color property has four values:

    border-color: red green blue pink;
        top border is red
        right border is green
        bottom border is blue
        left border is pink

If the border-color property has three values:

    border-color: red green blue;
        top border is red
        right and left borders are green
        bottom border is blue

If the border-color property has two values:

    border-color: red green;
        top and bottom borders are red
        right and left borders are green

If the border-color property has one value:

    border-color: red;
        all four borders are red

Note: Always declare the border-style property before the border-color property. An element must have borders before you can change the color.
"#,
border_block_color r#"
The border-block-color property sets the color of an element's borders in the block direction.

Note: For the border-block-color property to take effect, the border-block-style must be set.

Values for the border-block-color property can be set in different ways:

If the border-block-color property has two values:

    border-block-color: pink lightblue;
        border color at block start is pink
        border color at block end is lightblue

If the border-block-color property has one value:

    border-block-color: pink;
        border color at block start and end is pink

The CSS border-block-color property is very similar to CSS properties border-bottom-color, border-left-color, border-right-color and border-top-color, but the border-block-color property is dependent on block direction.

Note: The related CSS property writing-mode defines block direction. This affects where the start and end of a block is and the result of the border-block-color property. For pages in English, inline direction is left to right and block direction is downward.
"#,
border_left_color r#"
The border-left-color property sets the color of an element's left border.

Note: Always declare the border-style or the border-left-style property before the border-left-color property. An element must have a border before you can change the color.
"#,
border_right_color r#"
The border-right-color property sets the color of an element's right border.

Note: Always declare the border-style or the border-right-style property before the border-right-color property. An element must have a border before you can change the color.
"#,
border_top_color r#"
The border-top-color property sets the color of an element's top border.

Note: Always declare the border-style or the border-top-style property before the border-top-color property. An element must have a border before you can change the color.
"#,
border_bottom_color r#"
The border-bottom-color property sets the color of an element's bottom border.

Note: Always declare the border-style or the border-bottom-style property before the border-bottom-color property. An element must have a border before you can change the color.
"#,
border_inline_color r#"
The border-inline-color property sets the color of an element's borders in the inline direction.

Note: For the border-inline-color property to take effect, the border-inline-style must be set.

Values for the border-inline-color property can be set in different ways:

If the border-inline-color property has two values:

    border-inline-color: pink blue;
        border color at inline start is pink
        border color at inline end is blue

If the border-inline-color property has one value:

    border-inline-color: blue;
        border color at inline start and end is blue

The CSS border-inline-color property is very similar to CSS properties border-bottom-color, border-left-color, border-right-color and border-top-color, but the border-inline-color property is dependent on inline direction.

Note: The related CSS properties writing-mode, text-orientation and direction define inline direction. This affects where the start and end of a line is and the result of the border-inline-color property. For pages in English, inline direction is left to right and block direction is downward.    
"#,
border_inline_start_color r#"
The border-inline-start-color property sets the color of an element's border at the start in the inline direction.

Note: For the border-inline-start-color property to take effect, the border-inline-start-style property must be set.

The CSS border-inline-start-color property is very similar to CSS properties border-bottom-color, border-left-color, border-right-color and border-top-color, but the border-inline-start-color property is dependent on inline direction.

Note: The related CSS properties writing-mode, text-orientation and direction define inline direction. This affects where the start and end of a line is and the result of the border-inline-start-color property. For pages in English, inline direction is left to right and block direction is downward.    
"#,
border_inline_end_color r#"
The border-inline-end-color property sets the color of an element's border at the end in the inline direction.

Note: For the border-inline-end-color property to take effect, the border-inline-end-style property must be set.

The CSS border-inline-end-color property is very similar to CSS properties border-bottom-color, border-left-color, border-right-color and border-top-color, but the border-inline-end-color property is dependent on inline direction.

Note: The related CSS properties writing-mode, text-orientation and direction define inline direction. This affects where the start and end of a line is and the result of the border-inline-end-color property. For pages in English, inline direction is left to right and block direction is downward.    
"#,
border_block_start_color r#"
The border-block-start-color property sets the width of an element's border at the start in the block direction.

Note: For the border-block-start-color property to take effect, the border-block-start-style property must be set.

The CSS border-block-start-color property is very similar to CSS properties border-bottom-width, border-left-width, border-right-width and border-top-width, but the border-block-start-color property is dependent on block direction.

Note: The related CSS property writing-mode defines block direction. This affects where the start and end of a block is and the result of the border-block-start-color property. For pages in English, inline direction is left to right and block direction is downward.    
"#,
border_block_end_color r#"
The border-block-end-color property sets the color of an element's border at the end in the block direction.

Note: For the border-block-end-color property to take effect, the border-block-end-style property must be set.

The CSS border-block-end-color property is very similar to CSS properties border-bottom-width, border-left-width, border-right-width and border-top-width, but the border-block-end-color property is dependent on block direction.

Note: The related CSS property writing-mode defines block direction. This affects where the start and end of a block is and the result of the border-block-end-color property. For pages in English, inline direction is left to right and block direction is downward.    
"#,
: transparent $color;

color r#"
The color property specifies the color of text.

Tip: Use a background color combined with a text color that makes the text easy to read.
"#,
outline_color r#"
An outline is a line that is drawn around elements, outside the borders, to make the element "stand out".

The outline-color property specifies the color of an outline.

Note: Always declare the outline-style property before the outline-color property. An element must have an outline before you change the color of it.
"#,
column_rule_color r#"
The column-rule-color property specifies the color of the rule between columns.    
"#
: $color;

font_size,top,bottom,left,right,margin,height,width,padding : $length;

border_style r#"
sets the style of an element's four borders.
"#,
border_block_start_style r#"
sets the style of an element's border at the start in the block direction.
"#,
border_block_end_style r#"
sets the style of an element's border at the end in the block direction.
"#,
border_top_style r#"
sets the style of an element's top border.
"#,
border_bottom_style r#"
sets the style of an element's bottom border.
"#,
border_left_style r#"
sets the style of an element's left border.
"#,
border_right_style r#"
sets the style of an element's right border.
"#,
border_inline_style r#"
sets the style of an element's border at the end in the inline direction.
"#,
border_inline_start_style r#"
sets the style of an element's border at the start in the inline direction.
"#,
border_inline_end_style r#"
sets the style of an element's border at the end in the inline direction.
"#,
outline_style r#"
the style of an outline.
"#,
column_rule_style r#"
specifies the style of the rule between columns.
"#
:none hidden dotted dashed solid double groove ridge inset outset;

align_content r#"
The align-content property specifies how flex lines are distributed along the cross axis in a flexbox container.
    In flexbox layout, the main axis is in the flex-direction (default is row, horizontal), and the cross axis is perpendicular to the main axis (default is column, vertical).
    TIP: Use the justify-content property to align the items on the main axis (horizontally).
    NOTE: The align-content property can also be used on a grid container to align grid items in the block direction. For pages in English, block direction is downward and inline direction is left to right.    
"#
:stretch center flex_start flex_end space_between space_around space_evenly;

align_items r#"
The align-items property specifies the default alignment for items inside a flexbox or grid container.
        In a flexbox container, the flexbox items are aligned on the cross axis, which is vertical by default (opposite of flex-direction).
        In a grid container, the grid items are aligned in the block direction. For pages in English, block direction is downward and inline direction is left to right.
    For this property to have any alignment effect, the items need available space around themselves in the appropriate direction.
    TIP: Use the align-self property of each item to override the align-items property.    
"#
:stretch center flex_start flex_end start end baseline;

align_self_ r#"
the alignment in the block direction for the selected item inside a flexbox or grid container.
"#
:auto stretch center flex_start flex_end baseline;

all r#"
resets all properties, apart from unicode_bidi and direction, to their initial or inherited value.
"#
:initial inherit unset;

animation_direction r#"
defines whether an animation should be played forward, backward or in alternate cycles.
"#
:normal reverse alternate alternate_reverse;

animation_fill_mode r#"
a style for the element when the animation is not playing (before it starts, after it ends, or both).
"#
:none forwards backwards both;

animation_play_state r#"
whether the animation is running or paused.
"#
:paused running;

backface_visibility r#"
defines whether or not the back face of an element should be visible when facing the user.
"#
:visible hidden;

background_attachment r#"
sets whether a background image scrolls with the rest of the page, or is fixed.
"#
:scroll fixed local;

background_blend_mode r#"
defines the blending mode of each background layer (color and/or image).
"#
:normal multiply screen overlay darken lighten color_dodge saturation color luminosity;

background_clip r#"
defines how far the background (color or image) should extend within an element.
"#,
background_origin r#"
specifies the origin position (the background positioning area) of a background image.
"#
:padding_box border_box content_box;

background_repeat r#"
sets if/how a background image will be repeated.
"#
:repeat_x repeat_y no_repeat space round;

border_collapse r#"
sets whether table borders should collapse into a single border or be separated as in standard HTML.
"#
:separate collapse;

box_decoration_break r#"
how the background, padding, border, border_image, box_shadow, margin, and clip_path of an element is applied when the box for the element is fragmented.
"#
:slice clone unset;

box_sizing r#"
how the width and height of an element are calculated: should they include padding and borders, or not.
"#
:content_box border_box;

break_after r#"
whether or not a page break, column break, or region break should occur after the specified element.
"#,
break_before r#"
whether or not a page break, column break, or region break should occur before the specified element.
"#,
break_inside r#"
whether or not a page break, column break, or region break should occur inside the specified element.
"#
:auto all always avoid avoid_column avoid_page avoid_region column left page recto region right verso;

caption_side r#"
the placement of a table caption.
"#
:top bottom;

clear r#"
controls the flow next to floated elements.
"#
:none left right both;

color_scheme r#"
indicates which operating system color scheme an element should render with.
 "#
:normal light dark only;

column_fill r#"
specifies how to fill columns, balanced or not.
"#
:balance auto;


column_span r#"
how many columns an element should span across.
"#
:none all;

cursor r#"
the mouse cursor to be displayed when pointing over an element.
"#
:alias all_scroll auto cell context_menu col_resize copy crosshair e_resize ew_resize grab grabbing help n_resize ne_resize nesw_resize ns_resize nw_resize nwse_resize no_drop none not_allowed pointer progress row_resize s_resize se_resize sw_resize text vertical_text w_resize wait zoom_in zoom_out initial default_ move_;

direction r#"
the text direction/writing direction within a block_level element.
"#
:ltr rtl;

display r#"
the display behavior (the type of rendering box) of an element.
"#
:inline block inline_block flex contents inline_flex grid inline_grid table inline_table list_item none run_in;

empty_cells r#"
whether or not to display borders on empty cells in a table.
"#
:show hide;

flex_direction r#"
the direction of the flexible items.
"#
:row row_reverse column column_reverse;

flex_wrap r#"
the flexible items should wrap or not.
"#
:nowrap wrap wrap_reverse;

float r#"
whether an element should float to the left, right, or not at all.
"#
:none left right;

font_kerning r#"
controls the usage of the kerning information stored in a font.
"#
:auto normal none;

font_stretch r#"
make text narrower (condensed) or wider (expanded).
"#
:ultra_condensed extra_condensed condensed semi_condensed normal semi_expanded expanded extra_expanded ultra_expanded;

font_style r#"
the font style for a text.
"#
:normal italic oblique;

font_variant r#"
whether or not a text should be displayed in a small_caps font.
"#
:normal small_caps;

font_variant_caps r#"
controls the usage of alternate glyphs for capital letters.
"#
:normal small_caps all_small_caps petite_caps all_petite_caps unicase titling_caps unset;

font_weight r#"
how thick or thin characters in text should be displayed.
"#
:normal bold bolder lighter number;

hanging_punctuation r#"
whether a punctuation mark may be placed outside the line box at the start or at the end of a full line of text.
"#
:none first last allow_end force_end;

hyphens r#"
whether hyphenation is allowed to create more soft wrap opportunities within a line of text.
"#
:none manual auto;

image_rendering r#"
the type of algorithm to be used for image scaling.
"#
:auto smooth high_quality crisp_edges pixelated;

isolation r#"
whether an element must create a new stacking content.
"#
:auto isolate;

justify_content r#"
aligns the flexible container's items when the items do not use all available space on the main_axis (horizontally).
"#
:flex_start flex_end center space_between space_around space_evenly;

list_style_position r#"
the position of the list_item markers (bullet points).
"#
:inside outside;

list_style_type r#"
the type of list_item marker in a list.
"#
:disc circle square decimal decimal_leading_zero lower_alpha lower_greek lower_latin lower_roman upper_alpha upper_greek upper_latin upper_roman armenian cjk_ideographic georgian hebrew hiragana hiragana_iroha katakana katakana_iroha none;

mask_clip r#"
which area is affected by a mask image.
"#
:border_box content_box padding_box fill_box stroke_box view_box no_clip border padding content text;

mask_composite r#"
a compositing operation used on the current mask layer with the mask layers below it.
"#
:add subtract intersect exclude;

mask_mode r#"
whether the mask layer image should be treated as a luminance mask or as an alpha mask.
"#
:match_source luminance alpha;

mask_origin r#"
the origin position (the mask position area) of a mask layer image.
"#
:border_box content_box padding_box fill_box stroke_box view_box;

mask_repeat r#"
sets if/how a mask image will be repeated.
"#
:repeat repeat_x repeat_y space round no_repeat;

mask_type r#"
whether an SVG <mask> element is treated as a luminance mask or as an alpha mask.
"#
:luminance alpha;

mix_blend_mode r#"
how an element's content should blend with its direct parent background.
"#
:normal multiply screen overlay darken lighten color_dodge color_burn difference exclusion hue saturation color luminosity;

object_fit r#"
how an <img> or <video> should be resized to fit its container.
"#
:fill contain cover scale_down none;

object_position r#"
used together with object_fit to specify how an <img> or <video> should be positioned with x/y coordinates inside its 'own content box'.
"#
:position;


overflow r#"
what should happen if content overflows an element's box.
"#
:visible hidden clip scroll auto;

overflow_anchor r#"
makes it possible to turn off scroll anchoring.
"#
:auto none;

overflow_wrap r#"
whether or not the browser can break lines with long words, if they overflow the container.
"#
:normal anywhere break_word;

overflow_x r#"
whether to clip the content, add a scroll bar, or display overflow content of a block_level element, when it overflows at the left and right 
"#,
overflow_y r#"
whether to clip the content, add a scroll bar, or display overflow content of a block_level element, when it overflows at the top and bottom edges.
"#
:visible hidden scroll auto;

overscroll_behavior r#"
turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary.
"#,
overscroll_behavior_block r#"
turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the block direction.
"#,
overscroll_behavior_inline r#"
turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the inline direction.
"#,
overscroll_behavior_x r#"
turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in the x_direction.
"#,
overscroll_behavior_y r#"
turn off scroll chaining or overscroll affordance on an element when you try to scroll past the scroll boundary in y_direction.
"#,
:auto contain none;

place_self_ r#"
align individual grid items, and is a shorthand property for the following properties
:"#
:auto value;

pointer_events r#"
whether or not an element reacts to pointer events.
"#
:auto none;

position r#"
positioning method used for an element (static, relative, absolute, fixed, or sticky).
"#
:static_ absolute fixed relative sticky;

resize r#"
if (and how) an element is resizable by the user.
"#
:none both horizontal vertical;

scroll_behavior r#"
whether to smoothly animate the scroll position, instead of a straight jump, when the user clicks on a link within a scrollable box.
"#
:auto smooth;

scroll_snap_stop r#"
whether to scroll past elements or to stop and snap on the next element.
"#
:normal always;

table_layout r#"
the algorithm used to lay out table cells, rows, and columns.
"#
:auto fixed;

text_align r#"
the horizontal alignment of text in an element.
"#
:left right center justify;

text_align_last r#"
alignment for all last lines within the selected element. So, if you have a <div> with three paragraphs in it.
"#
:auto left right center justify start end;

text_decoration_line r#"
the kind of text decoration to use (like underline, overline, line_through).
"#
:none underline overline line_through;

text_decoration_style r#"
style of the text decoration (like solid, wavy, dotted, dashed, double).
"#
:solid double dotted dashed wavy;

text_emphasis r#"
apply emphasis marks to text.
"#
:none filled open dot circle double_circle triangle sesame string color;

text_emphasis_position r#"
the position of the emphasis marks (over, under, left, right).
"#
:over under left right;

text_emphasis_style r#"
the style of emphasis marks.
"#
:none filled open dot circle double_circle triangle sesame color;

text_justify r#"
the justification method of text when text_align is set to 'justify'.
"#
:auto inter_word inter_character none;

text_orientation r#"
the orientation of characters.
"#
:mixed upright sideways sideways_right use_glyph_orientation;

text_overflow r#"
how overflowed content that is not displayed should be signaled to the user. It can be clipped, display an ellipsis (...), or display a custom string.
"#
:clip ellipsis string;

text_transform r#"
controls the capitalization of text.
"#
:none capitalize uppercase lowercase;

text_underline_position r#"
the position of underline text decorations.
"#
:auto under from_font left right;

transform_style r#"
how nested elements are rendered in 3D space.
"#
:flat preserve_3d;

unicode_bidi r#"
together with the direction property to set or return whether the text should be overridden to support multiple languages in the same document.
"#
:normal embed bidi_override;

user_select r#"
whether the text of an element can be selected.
"#
:auto none text all;

vertical_align r#"
the vertical alignment of an element.
"#
:baseline length sub super_ top text_top middle bottom text_bottom;

visibility r#"
whether or not an element is visible.
"#
:visible hidden collapse;

white_space r#"
how white_space inside an element is handled.
"#
:normal nowrap pre pre_line pre_wrap;

word_break r#"
how words should break when reaching the end of a line.
"#
:normal break_all keep_all break_word;

word_wrap r#"
allows long words to be able to be broken and wrap onto the next line.
"#
:normal break_word;

writing_mode r#"
whether lines of text are laid out horizontally or vertically.
"#
:horizontal_tb vertical_rl vertical_lr;
);
