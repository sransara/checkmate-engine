import shutil
from subprocess import check_call
from pathlib import Path
import re
import sys
from rust_build import build_exe

project_dir = Path(__file__).parent
exepath = build_exe()

dist_dir = project_dir / "kaggle_submissions"

shutil.rmtree(dist_dir, ignore_errors=True)
shutil.rmtree(project_dir / "submission.tar", ignore_errors=True)
shutil.rmtree(project_dir / "submission.tar.gz", ignore_errors=True)

dist_dir.mkdir(exist_ok=True)
shutil.copy(exepath, dist_dir)

with open(project_dir / "main.py") as input_main_fh, open(dist_dir / "main.py", "w") as output_main_fh:
    input_main = input_main_fh.read()
    # replace all content between "# region: dev" and "# endregion" with empty string
    replacement = "exepath = '/kaggle_simulations/agent/checkmate_engine'"
    input_main = re.sub(r"# region: dev\n.*\n# endregion", replacement, input_main, flags=re.DOTALL)
    output_main_fh.write(input_main)

print("\nContents of kaggle_submissions:")
check_call(["ls", "-lha"], cwd=dist_dir, stdout=sys.stdout, stderr=sys.stderr)

print("\nCreating submission.tar.gz")
check_call(["tar", "-cf", "submission.tar", "-C", "kaggle_submissions", "."], cwd=project_dir)
check_call(["zopfli", "submission.tar"], cwd=project_dir)

print("\nSize of submission.tar.gz")
check_call(["ls", "-lha", "submission.tar.gz"], cwd=project_dir, stdout=sys.stdout, stderr=sys.stderr)