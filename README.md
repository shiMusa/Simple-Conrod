# Simple-Conrod
A simple, user-friendly and multi-threading enabled gui-framework in <b>Rust</b> based on <b>Conrod</b>
Example can be found in main() function.
<b>heavy WIP!</b>

# Preamble

For now, only a few visible elements like Button and Label are implemented. But the idea is that it should be relatively easy to create new elements which can be immediately included in you gui-design.
Check back in future for more stuff :3

Also: <b>There's much more already</b> than what's explained down below! check out the ```main()``` function!

# Adding Elements

<i>Before we start, let me say that you can also add and remove Elements dynamically during runtime!
For more info, see below.</i>

Creating a new window with title and size:
```rust
let mut base_window = BaseWindow::new("Container".to_string(), 800, 800);
```

All elements implement the trait Element so that they can be arbitrarily nested.
There are <i>container</i> elements to help organize the layout, e.g. a list with
a vertical arrangement of elements:
```rust
let mut list = List::new(ListAlignment::Vertical);
```

At the end of the chain we want a element to display or interact with - like a <i>Button</i>.
Let's add a Button with a Text on it immediately to the list
```rust
list.push(
    Button::new().with_label("Hey".to_string())
);
```

In the end, we want to add the list to the window and let the window run:
```rust
base_window.add_element(list);
base_window.run(-1f64);
```
A negative number indicates that the window is only redrawn, if there is a state change somewhere 
(like clicking a Button or resizing the window).
A positive number corresponds to the fps the window is forced to redraw. This is helpful if you
want to update dynamic content (real-time data, games ;)

# Actions (Events)

We can also use an event-system!
This is using the channel-system on the ```channel```-system. So that means, that 
signals from and to the gui are multi-threading enabled. Eg. you can run the gui in its own thread
while your model runs on another thread!

Senders can be added to the elements via ```.with_sender(...)```, e.g.
```rust
use std::sync::mpsc::{self, Sender, Receiver};

let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();
let button = Button::new().with_sender(sender);
```
These channels use the ```ActionMsg``` enum as information-carrier. You can also define a custom action-function:
```rust
let button = Button::new().with_action_click(Box::new(|| {
    // do something
}));
```

On the receiver side, the window accepts a receiver
```rust
base_window.add_receiver(receiver);
```
which will transmit the messages down the chain of Elements.
For an Element to receive a message, we need to wrap it in an <i>Socket</i>
```rust
let socket = Socket::new(/* some element */)
    .with_action_receive(Box::new(|element, msg|{
        match (msg.sender_id.as_ref(), msg.msg) {
            /* match messages */
            _ => ()
        }
}));
```

It is here, where you can now think of adding/removing elements during runtime. If e.g. in the above example "element" is a <i>List</i>, then you can just ```push(...)``` or ```insert(...)``` new elements or ```pop()``` and ```remove(...)``` elements.

<i>In very near future, you'll be able to add an arbitrary number of receivers to any Socket.</i>


# Goal and Future

The main goal is
 - a simple interface for users, especially for beginners and intermediates
 - not necessary performance, but ease of understanding
 - fast prototyping and construction
 
Main inspiration: Java framework.
 
<b>BEWARE:</b> I'm new to Rust, I'm not a professional programmer, I make mistakes - a lot...
 
Plans:
 - many more Elements
 - gui animations
 - scientific plotting
 - implementing other backends (Gfx, glutin)

![example application](https://github.com/shiMusa/Simple-Conrod/blob/master/example_new.PNG)


 

