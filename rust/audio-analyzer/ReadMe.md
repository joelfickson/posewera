#### Audi Analyzer

I started this project to learn more about the audio processing and how to use
the [FFT](https://en.wikipedia.org/wiki/Fast_Fourier_transform) algorithm. I used the [KissFFT](

### Inspiration

I want to use the functions in this project to:

- Create visualizations for music on Vwaza
- Detect the BPM of a song
- Detect a songs quality

### Objectives

- [ ] Generate SVG visualizations
- [ ] Create a simple audio player
- [ ] Create a simple audio visualizer
- [ ] Create a simple BPM detector
- [ ] Create a simple quality detector
- [ ] Ideally, should be able to read the file input from a URL

### Dependencies

Ideally, I don't want to use any third party libraries. I want to learn how to do this from scratch.

### Progress

- Dec 16, 2023 : 1:15AM => I can read a sample file and get the raw
  data. [Playfully, I am using a recursive method to re-write the bytes from the file. Not sure what use case this would be for, but it's fun to see the bytes in reverse order].
  I have started implementing the builder pattern. 