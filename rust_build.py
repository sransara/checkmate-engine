from subprocess import check_call, DEVNULL, STDOUT
from pathlib import Path
import os


def build_exe():
    project_root = Path(__file__).parent
    check_call(
        ["cargo", "+nightly", "build", "--release", "-Z", "build-std=std,panic_abort", "-Z", "build-std-features=optimize_for_size", "-Z", "build-std-features=panic_immediate_abort"], 
        cwd=project_root, 
        env={
            **dict(os.environ),
            "RUSTFLAGS": "-C target-cpu=native -Zlocation-detail=none -Zfmt-debug=none",
        },
    )
    output_path = project_root / "target/release/checkmate_engine"
    # check_call(["upx", "--best", "--lzma", output_path], cwd=project_root)
    return output_path