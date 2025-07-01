from keplemon import get_license_file_path, LICENSE_PATH


def test_license_file_path():
    assert get_license_file_path() == LICENSE_PATH.as_posix()
    assert LICENSE_PATH.exists()
