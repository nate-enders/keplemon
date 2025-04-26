from keplemon._keplemon.time import load_time_constants  # type: ignore
from keplemon._keplemon import get_thread_count  # type: ignore
from pathlib import Path
from shutil import copyfile

current_dir = Path(__file__).parent

# Copy the license file to the current working directory if it doesn't exist
working_license_path = Path.cwd() / "SGP4_Open_License.txt"
if not working_license_path.exists():
    copyfile(current_dir / "SGP4_Open_License.txt", working_license_path)

# Load the time constants from the assets directory
time_constants_path = current_dir / "assets" / "time_constants.dat"
load_time_constants(time_constants_path.as_posix())

__all__ = ["get_thread_count"]
