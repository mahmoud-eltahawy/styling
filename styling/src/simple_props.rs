use crate::{
    attribute::{Attribute, ToAttribute},
    AttributeGetter, PreBaseState, Style, StyleBaseState,
};

simple_attr::define_attributes!(
///resets all properties, apart from unicode-bidi and direction, to their initial or inherited value.
all:initial|inherit|unset;
align-content:stretch|center|flex-start|flex-end|space-between|space-around|space-evenly|initial|inherit;
align-items:stretch|center|flex-start|flex-end|start|end|baseline|initial|inherit;
align-self:auto|stretch|center|flex-start|flex-end|baseline|initial|inherit;
animation-direction:normal|reverse|alternate|alternate-reverse|initial|inherit;
animation-fill-mode:none|forwards|backwards|both|initial|inherit;
animation-play-state:paused|running|initial|inherit;

background-origin:padding-box|border-box|content-box|initial|inherit;
background-clip=background-origin;

background-repeat:repeat-x|repeat-y|no-repeat|space|round|initial|inherit;
background-blend-mode:normal|multiply|screen|overlay|darken|lighten|color-dodge|saturation|color|luminosity;
background-attachment:scroll|fixed|local|initial|inherit;

border-style              :none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
border-left-style         =border-style;
border-right-style        =border-style;
border-top-style          =border-style;
border-bottom-style       =border-style;
border-block-end-style    =border-style;
border-block-start-style  =border-style;
border-inline-style       =border-style;
border-inline-start-style =border-style;
border-inline-end-style   =border-style;

break-after   :auto|all|always|avoid|avoid-column|avoid-page|avoid-region|column|left|page|recto|region|right|verso|initial|inherit;
break-before  =break-after;
break-inside  =break-after;

box-decoration-break:slice|clone|initial|inherit|unset;
box-sizing:content-box|border-box|initial|inherit;
position:static_|relative|fixed|absolute|sticky|initial|inherit;
backface-visibility:visible|hidden|initial|inherit;

border-collapse:separate|collapse|initial|inherit;
caption-side:top|bottom|initial|inherit;
clear:none|left|right|both|initial|inherit;
color-scheme:normal|light|dark|only;
);
