#stat bars

Simple library just for drawing stat bars 
above sprites to display their health
or whatever.

Only goal is to be an easy to use crate that
you can drop into any project and have stat bars 
in less than ten minutes.

* Targets Bevy 0.7
* Only supports 2D.
* Will never be super-customisable or fancy.
* Only centered, horizontal bars atm. 
Next release supports arbitrary orientations etc.

# basic how-to

Add the StatBarsPlugin to your app,
then you can spawn stat bars like so:

```rust
commands
.spawn_bundle((
    // color of the statbar (required)
    StatBarColor(Color::GREEN), 

    // color of the empty part of the statbar (optional)
    StatBarEmptyColor(Color::BLACK),

    // color and thickness of the border (optional)
    StatBarBorder { color: Color::DARK_GRAY, thickness: 3.0 },

    // initial value (0.0 is empty, 1.0 is full) (required) 
    StatBarValue(1.0),

    // length and thickness of the bar (required)
    StatBarSize { full_length: 50.0, thickness: 6.0 },

    // entity the statbar tracks (required)
    StatBarSubject(player_entity),

    // position relative to the subject entity (40 units above in this case) (optional)
    StatBarPosition(40.0 * Vec2::Y),

    // takes a |&Component| -> f32 closure, which is called each tick with the
    //      respective component of the subject entity.
    //      returned f32 value is used to update StatBarValue
    // (optional, can leave out and just update StatBarValue manually)
    component_observer(|hp: &Hp| hp.current as f32 / hp.max as f32)
));   
```
and they should just work.

## Notes

* A stat bar is a single entity. It uses the bevy 
sprites renderer for drawing but doesn't create any
intermediate sprite entities. 

* By default stat bars have a z_depth of 950, you 
can set it yourself with the StatBarZDepth component.

* The bars automatically despawn when the subject is despawned. 

* Doesn't use the built in Parent-Child transform propagation as
most of the time games don't want their indicator elements rotating 
or scaling with the sprite they are tracking.

* Not optimised, doesn't use change detection etc or anything yet.
Extremely unlikely to be a serious performance bottle neck though, unless you spawn
10,000 stat bars at once or something.

* You can have more than one stat bar track a subject entity. Just make sure each has a different ```StatBarPosition```, so they don't all draw on top of each other.

* The ComponentObserver implementation is relatively slow, if you have a lot of status bars you might want to update StatusBarValue directly.
The advantage of the component observer is seperation. You shouldn't need to make any changes to your existing entities and systems to use them.

There is a complete-ish working example in the /examples folder.

Any feedback / improvement suggestions / code submissions will be super appreciated.