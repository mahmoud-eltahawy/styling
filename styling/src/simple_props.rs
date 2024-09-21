use crate::{
    attribute::{Attribute, ToAttribute},
    AttributeGetter, PreBaseState, Style, StyleBaseState,
};

simple_attr::define_attributes!(
all:initial|inherit|unset;
align_content:stretch|center|flex_start|flex_end|space_between|space_around|space_evenly|initial|inherit;
align_items:stretch|center|flex_start|flex_end|start|end|baseline|initial|inherit;
align_self:auto|stretch|center|flex_start|flex_end|baseline|initial|inherit;
animation_direction:normal|reverse|alternate|alternate_reverse|initial|inherit;
animation_fill_mode:none|forwards|backwards|both|initial|inherit;
animation_play_state:paused|running|initial|inherit;

background_origin:padding_box|border_box|content_box|initial|inherit;
background_clip=background_origin;

background_repeat:repeat_x|repeat_y|no_repeat|space|round|initial|inherit;
background_blend_mode:normal|multiply|screen|overlay|darken|lighten|color_dodge|saturation|color|luminosity;
background_attachment:scroll|fixed|local|initial|inherit;

border_style              :none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
border_left_style         =border_style;
border_right_style        =border_style;
border_top_style          =border_style;
border_bottom_style       =border_style;
border_block_end_style    =border_style;
border_block_start_style  =border_style;
border_inline_style       =border_style;
border_inline_start_style =border_style;
border_inline_end_style   =border_style;

break_after   :auto|all|always|avoid|avoid_column|avoid_page|avoid_region|column|left|page|recto|region|right|verso|initial|inherit;
break_before  =break_after;
break_inside  =break_after;

box_decoration_break:slice|clone|initial|inherit|unset;
box_sizing:content_box|border_box|initial|inherit;
backface_visibility:visible|hidden|initial|inherit;

border_collapse:separate|collapse|initial|inherit;
caption_side:top|bottom|initial|inherit;
clear:none|left|right|both|initial|inherit;
color_scheme:normal|light|dark|only;
);
