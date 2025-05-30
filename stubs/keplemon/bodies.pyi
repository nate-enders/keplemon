# flake8: noqa
from keplemon.elements import TLE, CartesianState, Ephemeris
from keplemon.catalogs import TLECatalog
from keplemon.time import Epoch, TimeSpan
from keplemon.events import CloseApproach, CloseApproachReport

class Earth:
    @staticmethod
    def get_equatorial_radius() -> float:
        """
        Returns:
            Equatorial radius of the Earth in kilometers
        """
        ...

    @staticmethod
    def get_kem() -> float: ...

class Satellite:
    satellite_id: int
    """Number used to distinguish the satellite from other objects.
    
    !!! note
        Every attempt should be made to make this unique to support satellite methods that perform comparisons or
        bookkeeping.
    """
    name: str | None
    """Human-readable name of the satellite"""

    @classmethod
    def from_tle(cls, tle: TLE) -> Satellite:
        """
        Instantiate a Satellite from a legacy TLE

        Args:
            tle: Two-line element set for the satellite
        """
        ...

    def get_close_approach(
        self,
        other: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> None | CloseApproach: ...
    def get_ephemeris(
        self,
        start: Epoch,
        end: Epoch,
        step: TimeSpan,
    ) -> Ephemeris: ...
    def get_state_at_epoch(self, epoch: Epoch) -> CartesianState: ...
    def to_tle(self) -> TLE | None:
        """
        Returns:
            Satellite as a two-line element set or None if no state is loaded

        """
        ...

class Constellation:
    """
    Args:
        name: Identifier of the constellation
    """

    count: int
    """Number of satellites in the constellation"""

    name: str | None
    """Human-readable name of the constellation"""

    def __init__(self, name: str) -> None: ...
    @classmethod
    def from_tle_catalog(cls, tle_catalog: TLECatalog) -> Constellation:
        """
        Instantiate a Constellation from a TLE catalog

        Args:
            tle_catalog: TLE catalog for the constellation
        """
        ...

    def get_states_at_epoch(self, epoch: Epoch) -> dict[int, CartesianState]:
        """
        Args:
            epoch: UTC epoch at which the states will be calculated

        Returns:
            (satellite_id, state) dictionary for the constellation at the given epoch
        """
        ...

    def get_ephemeris(
        self,
        start: Epoch,
        end: Epoch,
        step: TimeSpan,
    ) -> dict[int, Ephemeris]:
        """
        Args:
            start: UTC epoch of the start of the ephemeris
            end: UTC epoch of the end of the ephemeris
            step: Time step for the ephemeris

        Returns:
            (satellite_id, ephemeris) dictionary for the constellation
        """
        ...

    def get_ca_report_vs_one(
        self,
        other: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> CloseApproachReport:
        """
        Calculate close approaches between the constellation and a given satellite.

        Args:
            other: Satellite to compare against
            start: UTC epoch of the start of the close approach report
            end: UTC epoch of the end of the close approach report
            distance_threshold: Distance threshold for close approach screening in **_kilometers_**

        Returns:
            Close approach report for the constellation vs. the given satellite
        """
        ...

    def get_ca_report_vs_many(
        self,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> CloseApproachReport:
        """
        Calculate close approaches among satellites in the calling constellation.

        !!! warning
            This is a long-running operation when the constellation is large.

        Args:
            start: UTC epoch of the start of the close approach report
            end: UTC epoch of the end of the close approach report
            distance_threshold: Distance threshold for close approach screening in **_kilometers_**

        Returns:
            Close approach report for the constellation vs. all other satellites
        """
        ...

    def __getitem__(self, satellite_id: int) -> Satellite: ...

class Sensor:
    """
    Args:
        name: Identifier of the sensor
        angular_noise: Angular noise in **_degrees_**
    """

    name: str
    angular_noise: float
    range_noise: float | None
    """Range noise in **_kilometers_**"""

    range_rate_noise: float | None
    """Range rate noise in **_kilometers per second_**"""

    angular_rate_noise: float | None
    """Angular rate noise in **_degrees per second_**"""
    def __init__(self, name: str, angular_noise: float) -> None: ...

class Observatory:
    """
    Args:
        name: Identifier of the observatory
        latitude: Latitude in **_degrees_**
        longitude: Longitude in **_degrees_**
        altitude: Altitude in **_kilometers_**
    """

    name: str
    latitude: float
    longitude: float
    altitude: float
    sensors: list[Sensor]
    """List of sensors at the observatory"""
    def __init__(
        self,
        name: str,
        latitude: float,
        longitude: float,
        altitude: float,
    ) -> None: ...
    def get_state_at_epoch(self, epoch: Epoch) -> CartesianState:
        """
        Args:
            epoch: UTC epoch of the state

        Returns:
            TEME Cartesian state of the observatory in **_kilometers_** and **_kilometers per second_**
        """
        ...
