### TODO for rust port/expansion of "Stack It!"
* create object structs
    - [X] base object struct
    - [X] platform constructor
    - [X] dropper constructor
    - [X] box constructor
    - [X] game constructor
* core funtionality
    - [X] dropper emits a box at *n* angle
    - [X] dropper moves back/forth
    - [X] platform moves back/forth
* collision handling
    - [X] all should stay in bounds
    - [X] boxes should stick to platform and other stuck boxes
    - [X] boxes should fall off of platform/others when meeting screen edge or the dropper
* extended functionality
    - [ ] handle orphaned boxes
    - [ ] boxes should fall when supporting *n* other boxes
* communication / interactivity
    - [X] impl mouse/keyboard interface for dev
    - [ ] integrate twitch for chat interface to play
* waves
    - [ ] calculation
    - [ ] fancy-ish inexpensive draw calls
* network
    - [ ] simple server interface to receive command messages
___
* feature creep considerations
    * consider using images for textures instead of draw calls
    * consider using streamers emotes as the boxes
    * platform that capsizes when too many blocks on a side
    * maybe use a shader for water
___
* general ongoing
    * reduce dupe code
    * refactor long functions
    * refactor unecessary complexity
    * break code apart into modules