### Things to be done - in no particular order
* create bases classes we can inherit from, for example
    ```javascript
    class BaseOject() {
        location(x, y) : Vector
        size(width, height) : Vector, object literal, or individual width and height
        velocity(x, y) : Vector
        color : p5 color
    }
    ```
    * this should be good starting point to unify our other game things like:
    *box, box-dropper, and platform*

* handle orphaned boxes that have no connection to anything
* handle boxes that have connections beyond threshold *n*
* add commands and/or random events to change direction of the dropper or platform
* do the same for water and/or maybe have it vary itself over time
* drop boxes that can't fit screen on resize
* alternately scale things on resize to a min size

### General code changes
* refactor away duplicated code
* cleanup any TODO or FIXME comments
* improve comments in code so they make sense
* remove comments that are unecessary
* better structure code and files
