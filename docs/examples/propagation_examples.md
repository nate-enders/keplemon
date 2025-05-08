# Propagation Examples

## Propagating a Catalog

```python title="propagate_catalog.py"
from keplemon.catalogs import TLECatalog
from keplemon.bodies import Constellation
from keplemon.time import Epoch
from keplemon.enums import TimeSystem

# This can be a file in TLE or 3LE format
catalog = TLECatalog.from_tle_file("2025-04-15-space-track.3le")
all_sats = Constellation.from_tle_catalog(catalog)

propagation_epoch = Epoch.from_iso("2025-04-16T00:00:00.000", TimeSystem.UTC)
states = all_sats.get_states_at_epoch(propagation_epoch)

# We can safely assume this state will be available
iss_id = 25544

# A failure to propagate will return None.  These have likely decayed
none_count = 0
for sat_id in states:
    if states[sat_id] is None:
        none_count += 1

# TEME cartesian states are returned.  Converting to a Keplerian state will show orbit details
iss_kep = states[iss_id].to_keplerian()

print(f"Number of satellites loaded: {all_sats.count}\n")
print(f"Number of decayed objects: {none_count}\n")
print(f"{iss_id} at {propagation_epoch.to_iso()}:")
print(f"  Eccentricity: {iss_kep.eccentricity:8.3f}")
print(f"  Inclination:  {iss_kep.inclination:8.3f} degrees")
print(f"  RAAN:         {iss_kep.raan:8.3f} degrees")
print(f"  Apogee:       {iss_kep.apoapsis:8.3f} km")
print(f"  SMA:          {iss_kep.semi_major_axis:8.3f} km")
print(f"  Perigee:      {iss_kep.periapsis:8.3f} km")
print(f"  Mean Motion:  {iss_kep.mean_motion:8.3f} rev/day")
```

```text title="terminal"
>>> python propagate_catalog.py
Number of satellites loaded: 27485

Number of decayed objects: 101

25544 at 2025-04-16T00:00:00.000:
  Eccentricity:    0.002
  Inclination:    51.656 degrees
  RAAN:          255.031 degrees
  Apogee:       6812.827 km
  SMA:          6802.217 km
  Perigee:      6791.608 km
  Mean Motion:    15.475 rev/day
```
