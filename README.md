Converts lines from an image into parametric equations using Fourier transforms.

Usage: pretty rudimentary at the moment, just change "img" input in main.rs to desired image and run it like a normal rust program. generated equations will appear in images/equations.txt. see generated islands in images/lines.png.

Basic premise of how it works:
- Step 1: Convert image into edges using an edge detection algorithm currently just uses a basic Sobel filter, will likely implement a Canny edge detector or something similar in the future.
- Step 2: Get lines from images using a DFS that keeps track of the path and explores all 8 neighbouring pixels.
- Step 3: Construct equations of all the lines using Fourier transformations

Example usage:

Input image:

![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/a95159d4-1fd9-4009-b682-877f2aab996b)

Generated lines/paths:

![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/ede80409-3ed4-4859-ba00-ebb3e3010328)

Graphed equations:

![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/49bfa00d-f5d4-49b7-a47b-5de8afadf8c5)

