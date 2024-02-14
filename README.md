Converts lines from an image into parametric equations using Fourier transforms.

Usage: pretty rudimentary at the moment, just change "img" input in main.rs to desired image and run it like a normal rust program. generated equations will appear in images/equations.txt. see generated islands in images/lines.png.

Basic premise of how it works:
- Step 1: Convert image into edges using an edge detection algorithm. For now, it just uses a basic Sobel filter modified with a threshold, but I'm currently working on implementing the Canny edge detection algorithm.
- Step 2: Get lines from images using a DFS that keeps track of the path and explores all 8 neighbouring pixels. 
- Step 3: Construct equations of all the lines using Fourier transformations, omitting lines containing < n points. (currently n = 16)
  - Computes as follows:
    1) coefficients $cx_k = \sum_{i=0}^{N-1} \cos (\frac{2\pi k i}{N}) p_{x,i} + \sin(\frac{2\pi k i}{N}) p_{y,i}$ and $cy_k = \sum_{i=0}^{N-1} \cos(\frac{2\pi k i}{N}) p_{y,i} - \sin(\frac{2\pi k i}{N}) p_{x,i}$

    2) then we have $x(t) = \sum_{k=-\text{freqs}}^{\text{freqs}} \frac{cx_k}{N} \cos(k\pi t) - \frac{cy_k}{N} \sin(k\pi t)$ and $y(t) = \sum_{k=-\text{freqs}}^{\text{freqs}} \frac{cx_k}{N} \cos(k\pi t) + \frac{cy_k}{N} \sin(k\pi t)$

Example usage:

| Description | Input image | Edges | Generated lines/paths | Graphed equations |
| ------------|-------------|-------|---------------|-------------------|
| simple example with 1 pixel wide lines, without edge detection (lines2.png) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/a95159d4-1fd9-4009-b682-877f2aab996b) | n/a | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/ede80409-3ed4-4859-ba00-ebb3e3010328) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/49bfa00d-f5d4-49b7-a47b-5de8afadf8c5) |
| slightly more complex example, with Sobel filter applied (shapes1.png) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/2997fd73-8b29-417d-aebf-41451b5ae4a9) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/573868b4-3a13-4112-8379-131160d634f5) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/7c3d7600-99f4-4048-a9b0-52596e8fcf83) | much more of a mess! ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/de7558ff-2e30-45dd-8e90-b699285b3bb5)  |
| Let's see it attempt the Toronto skyline (toronto.png) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/c8f38b31-cc90-4438-8ef0-4dd9427ed4e9) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/357b352c-5a4e-4571-94a4-36eb5ba607ac) | ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/7d97f69a-0e47-4453-b172-fa4e4ee585b9) | Surprisingly lucid, although Desmos was barely keeping it together ![image](https://github.com/hunterchen7/WaveFormer/assets/34012681/86f02862-79a3-4e92-886a-cad562e6a40b) |
