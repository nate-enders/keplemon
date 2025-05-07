# flake8: noqa
def get_thread_count() -> int:
    """
    Returns:
        Number of cores allocated for use by KepLemon
    """
    ...

def set_thread_count(n: int) -> None:
    """
    Set the number of cores allocated for use by KepLemon

    !!! warning
        This function must be called before any other functions in the library

    Args:
        n: Number of cores to allocate
    """
    ...
