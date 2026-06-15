<div align="center">
  <a href="https://iaca-electronique.com">
    <img src="https://iaca-electronique.com/img/IACA_couleurs.svg" alt="IACA Electronique Logo" width="300px">
  </a>

# Synthetic data generator for Yolo

![](https://img.shields.io/badge/-Rust-000000?style=flat&logo=rust&logoColor=white)
![](https://img.shields.io/badge/-YOLO-FFCC00?style=flat&logo=natsdotio&logoColor=white)
![](https://img.shields.io/badge/version-in%20development-blue)
</div>

___

## 💬 Purpose

This program allows you to create synthetic data for YOLO object detection models, enhancing training datasets with realistic and diverse examples.

## 📖 Usage

```bash
program \
--background-dir <path_to_background_images> \
--object-dir <path_to_object_images> \
--distraction-dir <path_to_object_images> \
--output-dir <path_to_output_directory>
```

> Object directory should contain images of the objects you want to detect in YOLO format (JPEG or PNG). Place each category of objects in its own subfolder (e.g., `object_dir/vehicle`, `object_dir/person`, etc.).

## 🧪 Tests

```bash
cargo test -- --test-threads=1
```

> Because the test implementation uses Mockall context overrides, it must run on a single thread to prevent interference between test cases.

## 🛠️ Tools

### Dataset viewer

A basic dataset viewer is available in [`tools/viewer/`](tools/viewer/) directory.
See attached README for more information.

## 🤖 AI Assistance

AI assistance guidelines are defined in [.ai/RULES.md](.ai/RULES.md).

This project uses Claude, which reads instructions from `CLAUDE.md`. To avoid duplicating the same rules across multiple AI context files, each context file should only reference the shared rules file.

## 📜 License

This project is licensed under the terms of the GNU General Public License v3.0.

See the [LICENSE](LICENSE) file for the full text.

<div align="center">
  <p>Powered by <a href="https://iaca-electronique.com">IACA Electronique</a></p>
</div>
