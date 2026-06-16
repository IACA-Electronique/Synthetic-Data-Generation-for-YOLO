#!/usr/bin/env python3
"""Visualize YOLO OBB format dataset by drawing annotated bounding boxes on images."""

import argparse
import random
import sys
from pathlib import Path

import cv2
import numpy as np
import yaml


def load_dataset_yaml(dataset_path: Path) -> dict:
    yaml_file = dataset_path / "dataset.yaml"
    if not yaml_file.exists():
        return {"names": {}}
    with open(yaml_file) as f:
        return yaml.safe_load(f) or {}


def collect_samples(dataset_path: Path) -> list[tuple[Path, Path]]:
    """Return (image_path, label_path) pairs found in all splits."""
    samples = []
    for split in ("train", "val", "test"):
        images_dir = dataset_path / "images" / split
        labels_dir = dataset_path / "labels" / split
        if not images_dir.exists():
            continue
        for img_path in sorted(images_dir.iterdir()):
            if img_path.suffix.lower() not in {".png", ".jpg", ".jpeg", ".bmp", ".webp"}:
                continue
            label_path = labels_dir / (img_path.name + ".txt")
            if not label_path.exists():
                # try without double extension (e.g. "foo.png" → "foo.txt")
                label_path = labels_dir / (img_path.stem + ".txt")
            if label_path.exists():
                samples.append((img_path, label_path))
    return samples


def class_color(class_id: int) -> tuple[int, int, int]:
    rng = random.Random(class_id * 2654435761)
    return (rng.randint(50, 255), rng.randint(50, 255), rng.randint(50, 255))


def draw_obb(image: np.ndarray, label_path: Path, class_names: dict) -> np.ndarray:
    h, w = image.shape[:2]
    annotated = image.copy()

    with open(label_path) as f:
        for line in f:
            parts = line.strip().split()
            if len(parts) != 9:
                continue
            class_id = int(parts[0])
            coords = list(map(float, parts[1:]))
            # coords: x1 y1 x2 y2 x3 y3 x4 y4 (normalized)
            pts = np.array(
                [[coords[i] * w, coords[i + 1] * h] for i in range(0, 8, 2)],
                dtype=np.int32,
            )

            color = class_color(class_id)
            cv2.polylines(annotated, [pts], isClosed=True, color=color, thickness=2)

            label = class_names.get(class_id, str(class_id))
            origin = tuple(pts[0])
            cv2.putText(
                annotated,
                label,
                origin,
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                color,
                1,
                cv2.LINE_AA,
            )

    return annotated


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Generate annotated images from a YOLO OBB dataset."
    )
    parser.add_argument("dataset", help="Path to the YOLO OBB dataset root (contains dataset.yaml)")
    parser.add_argument("output", help="Directory where annotated images will be saved")
    parser.add_argument(
        "count",
        nargs="?",
        type=int,
        default=5,
        help="Number of images to annotate (default: 5)",
    )
    args = parser.parse_args()

    dataset_path = Path(args.dataset)
    output_path = Path(args.output)

    if not dataset_path.exists():
        sys.exit(f"Error: dataset path '{dataset_path}' does not exist.")

    output_path.mkdir(parents=True, exist_ok=True)

    meta = load_dataset_yaml(dataset_path)
    raw_names = meta.get("names", {})
    # names can be a list or a dict
    if isinstance(raw_names, list):
        class_names = {i: name for i, name in enumerate(raw_names)}
    else:
        class_names = {int(k): v for k, v in raw_names.items()}

    samples = collect_samples(dataset_path)
    if not samples:
        sys.exit("Error: no image/label pairs found in the dataset.")

    random.shuffle(samples)
    selected = samples[: args.count]

    for img_path, label_path in selected:
        image = cv2.imread(str(img_path))
        if image is None:
            print(f"Warning: could not read image '{img_path}', skipping.")
            continue

        annotated = draw_obb(image, label_path, class_names)
        out_file = output_path / img_path.name
        cv2.imwrite(str(out_file), annotated)
        print(f"Saved: {out_file}")

    print(f"Done. {len(selected)} image(s) annotated in '{output_path}'.")


if __name__ == "__main__":
    main()
