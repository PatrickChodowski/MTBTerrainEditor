use belly::prelude::*;
use bevy::prelude::*;
use belly::widgets::common::Label;
pub struct MTBColorsPlugin;

impl Plugin for MTBColorsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(BellyPlugin)
        .add_event::<OpenMTBColorEvent>()
        .add_system(open.run_if(on_event::<OpenMTBColorEvent>()))
        ;
    }
}


pub struct OpenMTBColorEvent;

const COLORS: &[&'static str] = &[
    // from https://colorswall.com/palette/105557
    // Red     Pink       Purple     Deep Purple
    "#f44336", "#e81e63", "#9c27b0", "#673ab7",
    // Indigo  Blue       Light Blue Cyan
    "#3f51b5", "#2196f3", "#03a9f4", "#00bcd4",
    // Teal    Green      Light      Green Lime
    "#009688", "#4caf50", "#8bc34a", "#cddc39",
    // Yellow  Amber      Orange     Deep Orange
    "#ffeb3b", "#ffc107", "#ff9800", "#ff5722",
];

fn open(mut commands: Commands) {
    commands.add(StyleSheet::load("color-picker.ess"));
    let colorbox = commands.spawn_empty().id();
    let label_r = commands.spawn_empty().id();
    let label_g = commands.spawn_empty().id();
    let label_b = commands.spawn_empty().id();
    let label_a = commands.spawn_empty().id();
    commands.add(eml! {
        <body>
            <div c:color-window>
                <div c:color-picking>
                    <span c:controls>
                        <slider c:red
                            bind:value=to!(colorbox, BackgroundColor:0|r)
                            bind:value=from!(colorbox, BackgroundColor:0.r())
                            bind:value=to!(label_r, Label:value|fmt.v(" R: {v:0.2}"))
                        />
                        <slider c:green
                            bind:value=to!(colorbox, BackgroundColor:0|g)
                            bind:value=from!(colorbox, BackgroundColor:0.g())
                            bind:value=to!(label_g, Label:value|fmt.v(" G: {v:0.2}"))
                        />
                        <slider c:blue
                            bind:value=to!(colorbox, BackgroundColor:0|b)
                            bind:value=from!(colorbox, BackgroundColor:0.b())
                            bind:value=to!(label_b, Label:value|fmt.v(" B: {v:0.2}"))
                        />
                        <slider c:alpha
                            bind:value=to!(colorbox, BackgroundColor:0|a)
                            bind:value=from!(colorbox, BackgroundColor:0.a())
                            bind:value=to!(label_a, Label:value|fmt.v(" A: {v:0.2}"))
                        />
                    </span>
                    <img c:colorbox-holder src="trbg.png">
                        <span {colorbox} c:colorbox s:background-color=managed()
                            on:ready=run!(|c: &mut BackgroundColor| c.0 = Color::WHITE)/>
                    </img>
                    <span c:colors>
                    <for color in = COLORS>
                        <button on:press=run!(for colorbox |c: &mut BackgroundColor| { c.0 = Color::from_hex(color) })>
                            <span s:background-color=*color s:width="100%" s:height="100%"/>
                        </button>
                    </for>
                    </span>
                </div>
                <div c:color-labels >
                // style on labels seems not to work from ess, but no clue why
                    <label s:color="black" s:font="bold" {label_r}/>
                    <label s:color="black" s:font="bold" {label_g}/>
                    <label s:color="black" s:font="bold" {label_b}/>
                    <label s:color="black" s:font="bold" {label_a}/>
                </div>
            </div>
        </body>
    });
}