# Signals

What's the deal with all these signals?

They are not the solution you think they are.

It might be cumbersome for YOU to always reference a single object but it's better than several hundred objects.

You shouldn't use signals because you will eventually tie local state to global state.

As in, you're just gonna sync signal state with a central state. And now you have multiple states.

When memory doesn't even work that way.

There are addresses.

It's javascript you are passing a reference to an object, you are never passing an actual object.
