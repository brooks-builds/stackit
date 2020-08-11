### TODO for rust port/expansion of "Stack It!"
* create object structs
    - [ ] platform
    - [ ] dropper
    - [ ] box
* core funtionality
    - [ ] dropper emits a box at *n* angle
    - [ ] dropper moves back/forth
    - [ ] platform moves back/forth
* collision handling
    - [ ] all should stay in bounds
    - [ ] boxes should stick to platform and other stuck boxes
    - [ ] boxes should fall off of platform/others when meeting screen edge or the dropper
* extended functionality
    - [ ] handle orphaned boxes
    - [ ] boxes should fall when supporting *n* other boxes
* communication / interactivity
    - [ ] impl mouse/keyboard interface for dev
    - [ ] integrate twitch for chat interface to play
* waves
    - [ ] calculation
    - [ ] fancy-ish inexpensive draw calls
___
* feature creep considerations
    * consider using images for textures instead of draw calls
    * consider using streamers emotes as the boxes
    * platform that capsizes when too many blocks on a side
    * maybe use a shader for water