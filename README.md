# Similari

[![Rust](https://github.com/insight-platform/Similari/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/insight-platform/Similari/actions/workflows/rust.yml)

Similari is a framework that helps build sophisticated tracking systems that collect observable object features and 
merge them into tracks based on feature vectors.

With Similari one can develop highly efficient parallelized [SORT](https://github.com/abewley/sort), 
[DeepSORT](https://github.com/nwojke/deep_sort), and another sophisticated tracking engines.

## Introduction

The purpose of Similari is to provide tools to build sophisticated in-memory objects similarity engines.

Similarity calculation is the important resource demanding task in machine learning and AI systems. Vectors in 
similarity engines are compared by calculating of n-dimensional distance - Euclidian, cosine or another one.
The distance is used to estimate how the vectors are close between each other.

The library helps to build various kinds of similarity engines - the simplest one is that holds vector features and 
allows comparing new vectors against the ones kept in the database. More sophisticated engines operates over tracks - 
series of observations for the same feature collected during the lifecycle. Such kind of systems are often used in video 
processing or other class of systems where observer receives fuzzy or changing observation results.

The crate provides the tools to collect tracks, build track storages, find similar tracks, and merge them. The crate 
doesn't provide any persistence layer yet.

## Applicability Notes

Despite the fact Similari allows building various similarity engines, there are competitive tools that sometime (or 
often) may be more desirable. In the section we will explain where it is applicable and what alternatives exist.

Similari fits best for the tasks where objects are dynamic - you want to remove them from the index or modify them  
as often as add new ones. This is the very important point - it is less efficient than tools that work with growing  
or static object spaces.

Similari fits best when an object is described by multiple observations for a certain feature class, not a single  
feature vector.

* **Fit**: track of the person across the room: person ReID, age/gender, Face ReID are gathered multiple times 
  during the tracking and used to merge tracks or provide aggregated results at the end of the track;
* **Not fit**: plagiarism database, when a single document is described by a number (or just one) 
  constant ReID vectors, documents are added but not removed often.

If your task looks like **Not fit**, you're probably looking for `HNSW` or `NMS` implementations:
* HNSW Rust - [Link](https://github.com/jean-pierreBoth/hnswlib-rs)
* HNSW C/Python - [link](https://github.com/nmslib/hnswlib)
* NMS - [link](https://github.com/nmslib/nmslib)

Objects in Similari index support following features:

* **Track lifecycle** - object is represented in Similarity by its lifetime (track) - it appears, evolves and 
  disappears. During the lifetime object evolves according to its behavioral properties (attributes, and feature 
  observations).
* **Feature Observation** - Similari assumes that track is observed by external observer entity that monitors its 
  features multiple times. Those features are presented by vectors or matrices of float numbers. When the 
  observation happened, the track is updated with gathered features. Later feature observations are used to find 
  similar tracks in the index.
* **Feature Observation Quality** - Every observation is accompanied by `quality` characteristic that may be used 
  when distances are calculated and when feature observations are merged. The reason to use `quality` is to be able  
  to keep only the best observations for every track, so when a low quality observation is added to the track it  
  can be dropped to optimize its feature space. 
* **Attributes** - arbitrary attributes describe additional track properties aside of feature observations. 
  Attributes is crucial part when you are comparing objects in the wild, because there may be attributes state when 
  objects are incompatible, like `animal_type` that prohibits you from comparing `dogs` and `cats` between each other. 
  Another popular use of attributes is a spatial or temporal characteristic of an object, e.g. objects that are at 
  distant locations in the same time cannot be the same. Attributes in Similari are dynamic and evolve upon every 
  feature observation addition and when objects are merged. They are used in both distance calculations and 
  compatibility guessing (which decreases compute space by skipping incompatible objects).

If you are planning to use Similari to search in a huge index, consider use of attributes to decrease the bruteforce 
space. If the attributes of two tracks are not compatible, their distance calculations are skipped.

## Performance

To keep the calculations performant the crate uses:
* [ultraviolet](https://crates.io/crates/ultraviolet) - fast SIMD computations.

Parallel computations are implemented with index sharding and parallel computations based on dedicated thread workers 
pool.

The vector operations performance depends a lot on the optimization level defined for the build. On low or default 
optimization levels Rust may not use f32 vectorization, so when running benchmarks take care of proper 
optimization levels configured.

### Rust optimizations

Use `RUSTFLAGS="-C target-cpu=native"` to enable all cpu features like AVX, AVX2, etc. It is beneficial to ultraviolet.

Alternatively you can add build instructions to `.cargo/config`:

```
[build]
rustflags = "-C target-cpu=native"
```

Take a look at [benchmarks](benches) for numbers.

### Numbers

IoU tracking benchmark for N simultaneously observed objects run on 4 cores of 
Intel(R) Core(TM) i5-7440HQ CPU @ 2.80GHz. The benchmark doesn't use heuristics
that separate the observed objects based on object distances.

The benchmark is located at [benches/iou_tracker.rs](benches/iou_tracker.rs).

```
10 objects   :      261,184 ns/iter (+/- 170,940)      [3800 FPS]
100 objects  :    1,440,733 ns/iter (+/- 361,937)      [694  FPS]
500 objects  :  17,705,508 ns/iter (+/- 5,622,983)     [57   FPS]
1000 objects :  58,834,824 ns/iter (+/- 12,626,173)    [17   FPS]
```

## Usage Examples

Take a look at 
* [examples/simple.rs](examples/simple.rs) for an idea of simple usage.
* [examples/track_merging.rs](examples/track_merging.rs) for an idea of intra-cam track merging.
* [examples/incremental_track_build.rs](examples/incremental_track_build.rs) very simple feature-based tracker.
* [examples/iou_tracker.rs](examples/iou_tracker.rs) very simple IoU tracker (without Kalman filter).

