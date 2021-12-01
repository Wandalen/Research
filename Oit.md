## Order Independent Transparency Methods 

### Depth peeling
[Paper](https://my.eng.utah.edu/~cs5610/handouts/order_independent_transparency.pdf)
[Sample](https://raw.githack.com/pailhead/three.js/depth-peel-stencil/examples/webgl_materials_depthpeel.html)
Pros: 
 - Sufficient visual quality
 - Constant memory allocation
 - Don't require post sorting
 - Support older hardware
Cons:
 - Can induce a significant overhead on the geometry processing stage and rasterization, cumullative over all iteration steps.
 - Complexity O(n/2)

A linear complexity algorithm to capture and sort multiple fragment in single pass. Multiple rasterizations of the scenes.Produces good images but is too slow for scenes with many layers.

### Dual Depth Peeling 
[Paper](https://my.eng.utah.edu/~cs5610/handouts/DualDepthPeeling.pdf)
[Article](https://medium.com/@shrekshao_71662/dual-depth-peeling-implementation-in-webgl-11baa061ba4b)
[Sample](https://tsherif.github.io/webgl2examples/oit-dual-depth-peeling.html)

Dual depth peeling reduces the number of passes from N to N/2+1, which may speed up
performance.

### Layered Weighted Blended Order-Independent Transparency
[Paper](https://jcgt.org/published/0002/02/09)
[Implemtation article](http://casual-effects.blogspot.com/2015/03/implemented-weighted-blended-order.html)
[WebGL sample](https://tsherif.github.io/webgl2examples/oit.html)
[Three.js sample](https://raw.githack.com/arose/three.js/oit/examples/webgl_oit.html)
[Cesium sample](http://bagnell.github.io/cesium/Apps/Sandcastle/gallery/OIT.html)
[Video](https://youtu.be/axvmoEqcTp8)

Decent performance. Requires output to two render targets.

### Multi-Layer Depth Peeling via Fragment Sort
[Paper](https://www.microsoft.com/en-us/research/wp-content/uploads/2006/06/tr-2006-81.pdf)
[Sample](?)

The dual depth peeling method was extended to extract two fragments per uniform depth bucket in each iteration. This reduced number of required iterations.

Extended version of original DP. Reduces the computation time significantly for real scenes.

## Performance

### DP vs WB

[Paper](https://graphics.tudelft.nl/Publications-new/2021/FEE21/layered_weighted_blended_order_independent_transparency.pdf)
Scene: engine, ~150k  non-opaque triangles
Card: NVIDIA GTX 480

| Method           | Frame time( ms ) |
| ---------------- | ---------------- |
| DP               | ~100             |
| Weighted Blended | ~5               |

### DP vs DP via Fragment Sort

[Paper](https://www.microsoft.com/en-us/research/wp-content/uploads/2006/06/tr-2006-81.pdf)
DPFS - DP via Fragment Sort
Card: NVIDIA Geforce 6800

| Scene  | Polygons | Max number of transparent layers | DPFS (fps) | DP (fps) |
| ------ | -------- | -------------------------------- | ---------- | -------- |
| grass  | 384      | 20                               | 2.11       | 1.09     |
| tree   | 384      | 20                               | 2.23       | 1.18     |
| villy  | 15472    | 31                               | 4.51       | 2.62     |
| stpaul | 14780    | 23                               | 4.72       | 3.15     |

## Summary

| Method              | WebGL Support    | Image quality | Performance           | Implementation difficulty | Sample | Description                                                                                 |
| ------------------- | ---------------- | ------------- | --------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------- |
| DP                  | Desktop,Mobile   | Good          | Slow on bigger scenes | Normal     | +      | Produces good images but is too slow for scenes with many layers                            |
| Dual DP             | Desktop,Mobile   | Good          | Faster                | Medium     | +      | Faster implementation of depth peeling                                                      |
| Weighted Blended    | Desktop,Mobile   | Decent        | Good                  | High       | +      | Happy medium of cost/fidelity                                                               |
| DP via Fragmen Sort | Desktop?,Mobile? | Good          | Good                  | High       | -      | Extended version of original DP. Reduces the computation time significantly for real scenes |


Methods in the order in which they should be tested on real project:
1. Dual DP 
2. Weighted Blended
3. DP via Fragmen Sort


