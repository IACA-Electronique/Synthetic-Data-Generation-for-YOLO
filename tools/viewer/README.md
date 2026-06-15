# Dataset Viewer

A small utility that renders annotated images from a **YOLO OBB** (Oriented Bounding Box) dataset.

It reads `images/` and `labels/` from the `train`, `val`, and `test` splits, draws the oriented bounding boxes on a random sample of images, and writes the result to an output directory.

> **Only YOLO OBB format is supported.** Each label file must contain lines of 9 space-separated values: `class_id x1 y1 x2 y2 x3 y3 x4 y4` (normalized coordinates). Standard axis-aligned YOLO format is not compatible.

---

## Docker (recommended)

### 1. Build the image

```bash
docker build -t yolo-obb-viewer tools/viewer/
```

### 2. Run

Mount your dataset and a local output directory into the container, then call the script:

```bash
docker run --rm \
  -v /path/to/your/dataset:/dataset:ro \
  -v /path/to/output:/output \
  yolo-obb-viewer \
  -c "python /app/visualize_obb.py /dataset /output 10"
```

- `/path/to/your/dataset` — root of the YOLO OBB dataset (must contain `dataset.yaml` and `images/`/`labels/` subdirectories).
- `/path/to/output` — directory where annotated images will be written (created if absent).
- `10` — number of images to annotate (optional, default: `5`).

Annotated images are saved to the output directory with the same filenames as the originals.

---

## Expected dataset layout

```
dataset/
├── dataset.yaml          # must contain a "names" key (list or dict)
├── images/
│   ├── train/
│   ├── val/
│   └── test/
└── labels/
    ├── train/
    ├── val/
    └── test/
```

Each label file pairs with an image by name (e.g. `images/train/foo.png` → `labels/train/foo.txt`).
