from keplemon import get_license_file_path, ASSETS_DIRECTORY


def test_license_file_path():
    assert get_license_file_path() == ASSETS_DIRECTORY.as_posix()
    assert ASSETS_DIRECTORY.exists()
