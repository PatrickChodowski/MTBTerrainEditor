# Text based modular terrain generator in bevy

Creates terrain out of json config file. 

## Features:

- text based: Fill out the config and get the terrain
- modular: Specify as many planes as you want with its parameters and modifiers
- extensible: define your own modifiers and apply them


## Steps:

- In config.json specify which file should be loaded
- Define the scene in scenes folder


## Plane data:
- name:  String
- location: [x,y,z]
- subdivisions: [u32, u32]
- dims: [width, length]
- color: 
    - gradient (low color, high color)
    - steps     (color till 10.0, color till 20.0 etc.)
    - single color 
- modifiers: [Modifiers]
    - Noise
    - Wander noise
    - Smooth edge
    - Flat edge
    - Easing
    - more to come...

## TODO:
- wander noise without target modifier
- convert it to a plugin
- add some smarter picker/loader for the files
- terrain exporter to gltf maybe?
- More modifiers..?



PR's welcome :)