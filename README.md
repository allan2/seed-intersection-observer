# Seed Intersection Observer Example

This is an example of Seed using the [Intersection Observer V1 API](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API) through web-sys.

The intersection observer is created. We tell it to watch the red box by giving the element reference to the observer.
When the red box is entirely in view, the label "Is intersecting entirely" will display a value of `true`.

To run:
```
trunk serve
```